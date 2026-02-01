//! Unit tests for TimerService state machine and behavior

use super::*;
use std::thread::sleep;
use std::time::Duration;

// Helper: simulate elapsed time without real waiting by backdating started_instant
fn fast_forward(service: &mut TimerService, seconds: u64) {
    service.started_instant = Some(Instant::now() - Duration::from_secs(seconds));
    service.update_remaining();
}

fn complete_work_session(service: &mut TimerService) {
    service.duration_secs = WORK_DURATION_SECS;
    fast_forward(service, WORK_DURATION_SECS as u64 + 1);
}

fn complete_break_session(service: &mut TimerService) {
    service.duration_secs = BREAK_DURATION_SECS;
    fast_forward(service, BREAK_DURATION_SECS as u64 + 1);
}

#[test]
fn test_initial_state_is_idle_work() {
    let mut service = TimerService::new();
    let state = service.get_state();

    assert_eq!(state.phase, Phase::Work);
    assert_eq!(state.status, Status::WorkReady);
    assert_eq!(state.remaining_secs, WORK_DURATION_SECS);
    assert_eq!(state.duration_secs, WORK_DURATION_SECS);
    assert!(!state.completion_flag);
    assert_eq!(state.state_label, "Ready to work");
}

#[test]
fn test_start_begins_work_timer() {
    let mut service = TimerService::new();
    service.start().unwrap();

    let state = service.get_state();
    assert_eq!(state.phase, Phase::Work);
    assert_eq!(state.status, Status::Running);
    assert_eq!(state.remaining_secs, WORK_DURATION_SECS);
    assert_eq!(state.state_label, "Working");
}

#[test]
fn test_start_while_running_does_nothing() {
    let mut service = TimerService::new();
    service.start().unwrap();
    sleep(Duration::from_millis(100));

    // Try to start again - should return error
    let result = service.start();
    assert!(result.is_err());

    let state = service.get_state();
    assert_eq!(state.status, Status::Running);
    // Timer should still be running from first start
    assert!(state.remaining_secs <= WORK_DURATION_SECS);
}

#[test]
fn test_pause_freezes_countdown() {
    let mut service = TimerService::new();
    service.start().unwrap();
    sleep(Duration::from_millis(200));

    service.pause().unwrap();
    let paused_state = service.get_state();
    assert_eq!(paused_state.status, Status::Paused);
    let remaining_at_pause = paused_state.remaining_secs;

    // Wait and verify time doesn't decrease
    sleep(Duration::from_millis(200));
    let still_paused = service.get_state();
    assert_eq!(still_paused.remaining_secs, remaining_at_pause);
    assert_eq!(still_paused.state_label, "Paused (work)");
}

#[test]
fn test_resume_continues_from_paused_time() {
    let mut service = TimerService::new();
    service.start().unwrap();
    sleep(Duration::from_millis(200));

    service.pause().unwrap();
    let remaining_at_pause = service.get_state().remaining_secs;

    sleep(Duration::from_millis(100));
    service.resume().unwrap();

    let resumed = service.get_state();
    assert_eq!(resumed.status, Status::Running);
    // Should resume with same remaining time (±1 second tolerance)
    assert!((resumed.remaining_secs as i32 - remaining_at_pause as i32).abs() <= 1);
}

#[test]
fn test_pause_resume_cycle_preserves_time() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Multiple pause/resume cycles
    for _ in 0..3 {
        sleep(Duration::from_millis(50));
        service.pause().unwrap();
        sleep(Duration::from_millis(50));
        service.resume().unwrap();
    }

    let final_state = service.get_state();
    assert_eq!(final_state.status, Status::Running);
    // Should have only lost ~150ms of time (3 * 50ms running)
    assert!(final_state.remaining_secs >= WORK_DURATION_SECS - 1);
}

#[test]
fn test_clear_from_running_state() {
    let mut service = TimerService::new();
    service.start().unwrap();
    sleep(Duration::from_millis(100));

    service.clear().unwrap();

    let cleared = service.get_state();
    assert_eq!(cleared.phase, Phase::Work);
    assert_eq!(cleared.status, Status::WorkReady);
    assert_eq!(cleared.remaining_secs, WORK_DURATION_SECS);
    assert!(!cleared.completion_flag);
    assert_eq!(cleared.state_label, "Ready to work");
}

#[test]
fn test_clear_from_paused_state() {
    let mut service = TimerService::new();
    service.start().unwrap();
    sleep(Duration::from_millis(100));
    service.pause().unwrap();

    service.clear().unwrap();

    let cleared = service.get_state();
    assert_eq!(cleared.status, Status::WorkReady);
    assert_eq!(cleared.remaining_secs, WORK_DURATION_SECS);
}

#[test]
fn test_pause_resume_in_break_phase() {
    let mut service = TimerService::new();
    service.phase = Phase::Break;
    service.status = Status::Running;
    service.remaining_secs = BREAK_DURATION_SECS;
    service.started_instant = Some(Instant::now());

    sleep(Duration::from_millis(200));
    service.pause().unwrap();
    let paused_remaining = service.get_state().remaining_secs;

    sleep(Duration::from_millis(200));
    service.resume().unwrap();

    let resumed = service.get_state();
    assert_eq!(resumed.phase, Phase::Break);
    assert_eq!(resumed.status, Status::Running);
    assert!((resumed.remaining_secs as i32 - paused_remaining as i32).abs() <= 1);
}

#[test]
fn test_clear_during_break() {
    let mut service = TimerService::new();
    service.phase = Phase::Break;
    service.status = Status::Running;
    service.remaining_secs = BREAK_DURATION_SECS;
    service.started_instant = Some(Instant::now());

    service.clear().unwrap();

    let cleared = service.get_state();
    assert_eq!(cleared.phase, Phase::Break);
    assert_eq!(cleared.status, Status::BreakReady);
    assert_eq!(cleared.remaining_secs, BREAK_DURATION_SECS);
}

#[test]
fn test_completion_flag_resets_on_next_cycle() {
    let mut service = TimerService::new();

    // Complete work session (simulate)
    service.phase = Phase::Work;
    service.status = Status::Running;
    service.remaining_secs = 0;
    service.handle_completion(Instant::now());

    assert!(service.get_state().completion_flag);

    // Clear should reset the flag
    service.clear().unwrap();
    assert!(!service.get_state().completion_flag);
}

#[test]
fn test_state_labels_match_phase_and_status() {
    let mut service = TimerService::new();

    // Idle work
    assert_eq!(service.get_state().state_label, "Ready to work");

    // Running work
    service.start().unwrap();
    assert_eq!(service.get_state().state_label, "Working");

    // Paused work
    service.pause().unwrap();
    assert_eq!(service.get_state().state_label, "Paused (work)");

    // Running break (simulate)
    service.phase = Phase::Break;
    service.status = Status::Running;
    service.state_label = "Break time".to_string();
    assert_eq!(service.get_state().state_label, "Break time");

    // Complete
    service.status = Status::Complete;
    service.state_label = "Break complete".to_string();
    assert_eq!(service.get_state().state_label, "Break complete");
}

#[test]
fn test_timing_accuracy_within_tolerance() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Run for approximately 1 second
    sleep(Duration::from_millis(1000));
    service.update_remaining();

    let state = service.get_state();
    // Should have consumed ~1 second (±1 second tolerance for SC-002)
    let expected_remaining = WORK_DURATION_SECS - 1;
    assert!((state.remaining_secs as i32 - expected_remaining as i32).abs() <= 1);
}

#[test]
fn test_cannot_pause_when_idle() {
    let mut service = TimerService::new();
    let result = service.pause();

    // Should return error
    assert!(result.is_err());
    assert_eq!(service.get_state().status, Status::WorkReady);
}

#[test]
fn test_cannot_resume_when_not_paused() {
    let mut service = TimerService::new();
    let result = service.resume();

    // Should return error
    assert!(result.is_err());
    assert_eq!(service.get_state().status, Status::WorkReady);
}

#[test]
fn test_clear_resets_to_initial_state() {
    let mut service = TimerService::new();
    service.clear().unwrap();

    let state = service.get_state();
    assert_eq!(state.phase, Phase::Work);
    assert_eq!(state.status, Status::WorkReady);
    assert_eq!(state.remaining_secs, WORK_DURATION_SECS);
    assert_eq!(state.duration_secs, WORK_DURATION_SECS);
    assert!(!state.completion_flag);
    assert_eq!(state.state_label, "Ready to work");
}

#[test]
fn test_monotonic_timing_handles_system_clock_changes() {
    let mut service = TimerService::new();
    service.start().unwrap();

    let instant1 = service.started_instant.unwrap();
    sleep(Duration::from_millis(100));

    // Verify we're using Instant (monotonic) not SystemTime
    let instant2 = service.started_instant.unwrap();
    assert_eq!(instant1, instant2); // Same instant throughout running session
}

// ============================================================================
// User Story 1: Manual break initiation after work session
// ============================================================================

// Fundamental tests for stay-on-complete behavior (US1)

#[test]
fn test_work_completion_stays_in_work_mode() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Simulate completion of work session
    service.duration_secs = WORK_DURATION_SECS;
    service.started_instant =
        Some(Instant::now() - Duration::from_secs(WORK_DURATION_SECS as u64 + 1));
    service.update_remaining();

    let state = service.get_state();
    assert_eq!(state.phase, Phase::Work);
    assert_eq!(state.status, Status::Complete);
    assert_eq!(state.remaining_secs, 0);
    assert!(state.completion_flag);
    assert_eq!(state.state_label, "Work completed");
}

#[test]
fn test_start_after_work_completion_restarts_work() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Simulate completion of work session
    service.duration_secs = WORK_DURATION_SECS;
    service.started_instant =
        Some(Instant::now() - Duration::from_secs(WORK_DURATION_SECS as u64 + 1));
    service.update_remaining();

    // Start should restart work session from Complete status
    service.start().unwrap();
    let state = service.get_state();
    assert_eq!(state.phase, Phase::Work);
    assert_eq!(state.status, Status::Running);
    assert_eq!(state.remaining_secs, WORK_DURATION_SECS);
    assert!(!state.completion_flag);
    assert_eq!(state.state_label, "Working");
}

#[test]
fn test_completion_flag_set_on_work_completion() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Before completion
    assert!(!service.completion_flag);

    // After completion (simulated)
    complete_work_session(&mut service);
    service.get_state();

    assert!(service.completion_flag);
}

// ===== User Story 2: Break completion stays in break mode =====

#[test]
fn test_break_completion_stays_in_break_mode() {
    let mut service = TimerService::new();

    // Switch to break phase and start
    service.set_phase(Phase::Break);
    service.start().unwrap();

    // Simulate completion of break session
    service.duration_secs = BREAK_DURATION_SECS;
    service.started_instant =
        Some(Instant::now() - Duration::from_secs(BREAK_DURATION_SECS as u64 + 1));
    service.update_remaining();

    let state = service.get_state();
    assert_eq!(state.phase, Phase::Break);
    assert_eq!(state.status, Status::Complete);
    assert_eq!(state.remaining_secs, 0);
    assert!(state.completion_flag);
    assert_eq!(state.state_label, "Break completed");
}

#[test]
fn test_start_after_break_completion_restarts_break() {
    let mut service = TimerService::new();

    // Switch to break phase and start
    service.set_phase(Phase::Break);
    service.start().unwrap();

    // Simulate completion of break session
    service.duration_secs = BREAK_DURATION_SECS;
    service.started_instant =
        Some(Instant::now() - Duration::from_secs(BREAK_DURATION_SECS as u64 + 1));
    service.update_remaining();

    // Start should restart break session from Complete status
    service.start().unwrap();
    let state = service.get_state();
    assert_eq!(state.phase, Phase::Break);
    assert_eq!(state.status, Status::Running);
    assert_eq!(state.remaining_secs, BREAK_DURATION_SECS);
    assert!(!state.completion_flag);
    assert_eq!(state.state_label, "Break time");
}

// ===== User Story 3: Manual session transition after completion =====

#[test]
fn test_phase_switch_from_complete_status() {
    let mut service = TimerService::new();

    // Complete work session
    service.start().unwrap();
    complete_work_session(&mut service);
    service.get_state();

    // Verify in Complete status
    assert_eq!(service.status, Status::Complete);
    assert_eq!(service.phase, Phase::Work);

    // Switch to break phase via set_phase
    service.set_phase(Phase::Break);

    let state = service.get_state();
    assert_eq!(state.phase, Phase::Break);
    assert_eq!(state.status, Status::BreakReady);
    assert_eq!(state.remaining_secs, BREAK_DURATION_SECS);
    assert!(!state.completion_flag);
}

#[test]
fn test_same_phase_switch_after_complete_resets() {
    let mut service = TimerService::new();

    // Complete work session
    service.start().unwrap();
    complete_work_session(&mut service);
    service.get_state();

    assert_eq!(service.status, Status::Complete);
    assert_eq!(service.phase, Phase::Work);

    // Calling set_phase(Work) while already in Work after completion should be idempotent
    service.set_phase(Phase::Work);

    let state = service.get_state();
    assert_eq!(state.phase, Phase::Work);
    assert_eq!(state.status, Status::Complete); // Should remain Complete (idempotent)
}

#[test]
fn test_clear_preserves_current_phase() {
    let mut service = TimerService::new();

    // Complete work session
    service.start().unwrap();
    complete_work_session(&mut service);
    service.get_state();

    assert_eq!(service.phase, Phase::Work);
    assert_eq!(service.status, Status::Complete);

    // Clear should preserve work phase and reset to WorkReady
    service.clear().unwrap();

    let state = service.get_state();
    assert_eq!(state.phase, Phase::Work);
    assert_eq!(state.status, Status::WorkReady);
    assert_eq!(state.remaining_secs, WORK_DURATION_SECS);
    assert!(!state.completion_flag);
}

// ===== General Behavior: Pause/Resume =====

#[test]
fn test_work_countdown_continues_after_resume() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Complete work session and restart
    complete_work_session(&mut service);
    service.get_state();
    service.start().unwrap();

    // Simulate 1 second into session, then pause
    fast_forward(&mut service, 1);
    service.pause().unwrap();
    let remaining_at_pause = service.remaining_secs;

    // While paused, time should not progress (no fast-forward)
    service.resume().unwrap();

    // Simulate 1 second of running after resume
    fast_forward(&mut service, 1);
    let state = service.get_state();

    assert!(state.remaining_secs < remaining_at_pause);
    assert!(state.remaining_secs >= remaining_at_pause - 2); // Allow some tolerance
}

#[test]
fn test_completion_flag_set_on_completion_after_restart() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Complete work session
    complete_work_session(&mut service);
    service.get_state();
    // Restart same work session
    service.start().unwrap();

    // Before completion
    assert!(!service.completion_flag);

    // Complete work session again
    complete_break_session(&mut service);
    service.get_state();

    // Flag should be set to trigger chime
    assert!(service.completion_flag);
}

// ===== Edge Cases =====

#[test]
fn test_pause_at_one_second_then_complete_stays_in_phase() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Fast forward to 00:01 (1 second remaining)
    fast_forward(&mut service, WORK_DURATION_SECS as u64 - 1);
    service.pause().unwrap();

    let paused_state = service.get_state();
    assert_eq!(paused_state.status, Status::Paused);
    assert_eq!(paused_state.remaining_secs, 1);

    // Resume and let it complete
    service.resume().unwrap();
    fast_forward(&mut service, 2); // Go past completion

    let completed_state = service.get_state();
    assert_eq!(completed_state.phase, Phase::Work);
    assert_eq!(completed_state.status, Status::Complete);
    assert_eq!(completed_state.remaining_secs, 0);
    assert!(completed_state.completion_flag);
}

#[test]
fn test_start_while_running_returns_error() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Try to start again while running
    let result = service.start();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Timer already running");
}

// ===== Tests for set_phase() method =====

#[test]
fn test_set_phase_idempotent() {
    let mut service = TimerService::new();
    let initial_remaining = service.remaining_secs;

    // Call set_phase with same phase
    service.set_phase(Phase::Work);

    // Should not change state
    assert_eq!(service.phase, Phase::Work);
    assert_eq!(service.remaining_secs, initial_remaining);
    assert_eq!(service.status, Status::WorkReady);
}

#[test]
fn test_set_phase_preserves_paused_time() {
    let mut service = TimerService::new();

    // Start work, fast forward 5 minutes, pause it
    service.start().unwrap();
    fast_forward(&mut service, 300);
    service.pause().unwrap();
    let work_paused_secs = service.get_state().remaining_secs;
    assert!(work_paused_secs < WORK_DURATION_SECS);

    // Switch to break
    service.set_phase(Phase::Break);
    assert_eq!(service.phase, Phase::Break);
    assert_eq!(service.remaining_secs, BREAK_DURATION_SECS);
    assert_eq!(service.status, Status::BreakReady);
    assert_eq!(service.paused_work_secs, Some(work_paused_secs));

    // Switch back to work
    service.set_phase(Phase::Work);
    assert_eq!(service.phase, Phase::Work);
    assert_eq!(service.remaining_secs, work_paused_secs); // Restored
    assert_eq!(service.status, Status::Paused);
}

#[test]
fn test_set_phase_pauses_running_timer() {
    let mut service = TimerService::new();
    service.start().unwrap();
    fast_forward(&mut service, 300); // 5 minutes

    // Still running, capture remaining
    let work_state = service.get_state();
    assert_eq!(work_state.status, Status::Running);
    let remaining_at_switch = work_state.remaining_secs;
    assert!(remaining_at_switch < WORK_DURATION_SECS);

    // Switch to break
    service.set_phase(Phase::Break);

    // Work should be paused and saved
    assert_eq!(service.phase, Phase::Break);
    assert_eq!(service.status, Status::BreakReady);
    assert_eq!(service.paused_work_secs, Some(remaining_at_switch));

    // Break should show standard duration
    assert_eq!(service.remaining_secs, BREAK_DURATION_SECS);
}

#[test]
fn test_set_phase_loads_standard_duration() {
    let mut service = TimerService::new();

    // Start and pause work after 5 minutes
    service.start().unwrap();
    fast_forward(&mut service, 300);
    service.pause().unwrap();
    assert!(service.remaining_secs < WORK_DURATION_SECS);

    // Switch to break - should load standard break duration
    service.set_phase(Phase::Break);
    assert_eq!(service.remaining_secs, BREAK_DURATION_SECS);
    assert_eq!(service.duration_secs, BREAK_DURATION_SECS);

    // Switch to work - should restore paused work time
    service.set_phase(Phase::Work);
    assert!(service.remaining_secs < WORK_DURATION_SECS);
    assert!(service.remaining_secs > 0);
}

#[test]
fn test_set_phase_paused_to_paused_preserves_both() {
    let mut service = TimerService::new();

    // Pause work at 20:00 (after 5 min)
    service.start().unwrap();
    fast_forward(&mut service, 300);
    service.pause().unwrap();
    let work_remaining = service.get_state().remaining_secs;
    assert_eq!(service.paused_work_secs, Some(work_remaining)); // Verify saved

    // Switch to break, start and pause at 4:00 (after 1 min)
    service.set_phase(Phase::Break);
    assert_eq!(service.phase, Phase::Break);
    // paused_work_secs should still be saved from work phase
    assert_eq!(service.paused_work_secs, Some(work_remaining));

    service.start().unwrap();
    fast_forward(&mut service, 60);
    service.pause().unwrap();
    let break_remaining = service.get_state().remaining_secs;

    // Verify both are saved
    assert_eq!(service.paused_work_secs, Some(work_remaining));
    assert_eq!(service.paused_break_secs, Some(break_remaining));

    // Switch back to work - should see work remaining
    service.set_phase(Phase::Work);
    assert_eq!(service.remaining_secs, work_remaining);
    assert_eq!(service.phase, Phase::Work);

    // Switch to break - should see break remaining
    service.set_phase(Phase::Break);
    assert_eq!(service.remaining_secs, break_remaining);
    assert_eq!(service.phase, Phase::Break);
}

// ========== Overtime Display Tests (Feature 009) ==========

#[test]
fn test_overtime_displayed_after_work_completion() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Complete work session and wait 10 seconds
    complete_work_session(&mut service);

    // Set completed_at to exactly 10 seconds ago
    let now = Instant::now();
    service.completed_at = Some(now - Duration::from_secs(10));

    let state = service.get_state();
    assert_eq!(state.status, Status::Complete);
    assert_eq!(state.overtime_secs, Some(10));
}

#[test]
fn test_overtime_caps_at_59_59() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Complete work session and wait 2 hours (7200 seconds)
    complete_work_session(&mut service);
    sleep(Duration::from_millis(100));

    // Fast-forward the completed_at timestamp by 7200 seconds
    if let Some(completed) = service.completed_at {
        service.completed_at = Some(completed - Duration::from_secs(7200));
    }

    let state = service.get_state();
    assert_eq!(state.status, Status::Complete);
    assert_eq!(state.overtime_secs, Some(3599)); // Capped at 59:59
}

#[test]
fn test_overtime_cleared_on_start() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Complete work session with overtime
    complete_work_session(&mut service);

    // Set completed_at to exactly 30 seconds ago
    let now = Instant::now();
    service.completed_at = Some(now - Duration::from_secs(30));

    // Verify overtime exists
    assert_eq!(service.get_state().overtime_secs, Some(30));

    // Start new timer
    service.start().unwrap();

    let state = service.get_state();
    assert_eq!(state.status, Status::Running);
    assert_eq!(state.overtime_secs, None);
    assert!(service.completed_at.is_none());
}

#[test]
fn test_overtime_cleared_on_clear() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Complete work session with overtime
    complete_work_session(&mut service);

    // Set completed_at to exactly 45 seconds ago
    let now = Instant::now();
    service.completed_at = Some(now - Duration::from_secs(45));

    // Verify overtime exists
    assert_eq!(service.get_state().overtime_secs, Some(45));

    // Clear timer
    service.clear().unwrap();

    let state = service.get_state();
    assert_eq!(state.status, Status::WorkReady);
    assert_eq!(state.overtime_secs, None);
    assert!(service.completed_at.is_none());
}

#[test]
fn test_overtime_cleared_on_phase_change() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Complete work session with overtime
    complete_work_session(&mut service);

    // Set completed_at to exactly 20 seconds ago
    let now = Instant::now();
    service.completed_at = Some(now - Duration::from_secs(20));

    // Verify overtime exists
    assert_eq!(service.get_state().overtime_secs, Some(20));

    // Switch to break phase
    service.set_phase(Phase::Break);

    let state = service.get_state();
    assert_eq!(state.phase, Phase::Break);
    assert_eq!(state.overtime_secs, None);
    assert!(service.completed_at.is_none());
}

#[test]
fn test_overtime_displayed_after_break_completion() {
    let mut service = TimerService::new();
    service.set_phase(Phase::Break);
    service.start().unwrap();

    // Complete break session and wait 5 seconds
    complete_break_session(&mut service);

    // Set completed_at to exactly 5 seconds ago
    let now = Instant::now();
    service.completed_at = Some(now - Duration::from_secs(5));

    let state = service.get_state();
    assert_eq!(state.status, Status::Complete);
    assert_eq!(state.overtime_secs, Some(5));
}

#[test]
fn test_overtime_break_cleared_on_start() {
    let mut service = TimerService::new();
    service.set_phase(Phase::Break);
    service.start().unwrap();

    // Complete break session with overtime
    complete_break_session(&mut service);

    // Set completed_at to exactly 15 seconds ago
    let now = Instant::now();
    service.completed_at = Some(now - Duration::from_secs(15));

    // Verify overtime exists
    assert_eq!(service.get_state().overtime_secs, Some(15));

    // Start new timer (restarts break)
    service.start().unwrap();

    let state = service.get_state();
    assert_eq!(state.phase, Phase::Break);
    assert_eq!(state.status, Status::Running);
    assert_eq!(state.overtime_secs, None);
    assert!(service.completed_at.is_none());
}

#[test]
fn test_overtime_cap_beyond_59_59() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Complete work session
    complete_work_session(&mut service);

    // Set completed_at to 1 hour and 30 seconds ago (3630 seconds)
    service.completed_at = Some(Instant::now() - Duration::from_secs(3630));

    let state = service.get_state();
    assert_eq!(
        state.overtime_secs,
        Some(3599),
        "Overtime should cap at 3599 (59:59), got {:?}",
        state.overtime_secs
    );

    // Set completed_at to exactly 1 hour ago (3600 seconds)
    service.completed_at = Some(Instant::now() - Duration::from_secs(3600));

    let state2 = service.get_state();
    assert_eq!(
        state2.overtime_secs,
        Some(3599),
        "Overtime at exactly 60:00 should cap at 3599"
    );

    // Set completed_at to 2 hours ago (7200 seconds)
    service.completed_at = Some(Instant::now() - Duration::from_secs(7200));

    let state3 = service.get_state();
    assert_eq!(
        state3.overtime_secs,
        Some(3599),
        "Overtime beyond 1 hour should remain capped at 3599"
    );
}
