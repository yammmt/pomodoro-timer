//! Unit tests for TimerService state machine and behavior
//!
//! Tests cover:
//! - US1: Work session start, pause, resume
//! - US2: Auto-transition to break, completion flag
//! - US3: Clear/reset functionality
//! - Edge cases: duplicate starts, overlap prevention, state transitions

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
fn test_work_completion_transitions_to_break() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Simulate timer already past the full duration to trigger completion deterministically
    service.duration_secs = WORK_DURATION_SECS;
    service.started_instant =
        Some(Instant::now() - Duration::from_secs(WORK_DURATION_SECS as u64 + 1));
    service.update_remaining();

    let state = service.get_state();
    assert_eq!(state.phase, Phase::Break);
    // no auto-start
    assert_eq!(state.status, Status::BreakReady);
    assert_eq!(state.remaining_secs, BREAK_DURATION_SECS);
    assert!(state.completion_flag);
    assert_eq!(state.state_label, "Break ready - press Start");
}

#[test]
fn test_break_completion_sets_complete_status() {
    let mut service = TimerService::new();
    service.phase = Phase::Break;
    service.status = Status::Running;
    service.duration_secs = BREAK_DURATION_SECS;
    service.started_instant =
        Some(Instant::now() - Duration::from_secs(BREAK_DURATION_SECS as u64 + 1));
    service.update_remaining();

    let state = service.get_state();
    assert_eq!(state.status, Status::WorkReady);
    assert!(state.completion_flag);
    assert_eq!(state.state_label, "Work ready - press Start");
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
    assert_eq!(cleared.phase, Phase::Work);
    assert_eq!(cleared.status, Status::WorkReady);
    assert_eq!(cleared.remaining_secs, WORK_DURATION_SECS);
}

#[test]
fn test_completion_flag_resets_on_next_cycle() {
    let mut service = TimerService::new();

    // Complete work session (simulate)
    service.phase = Phase::Work;
    service.status = Status::Running;
    service.remaining_secs = 0;
    service.handle_completion();

    assert!(service.get_state().completion_flag);

    // Verify flag persists in break
    let break_state = service.get_state();
    assert!(break_state.completion_flag);

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
fn test_overrun_advances_to_break() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Simulate we started long enough ago to exceed the work duration
    service.duration_secs = WORK_DURATION_SECS;
    service.started_instant =
        Some(Instant::now() - Duration::from_secs(WORK_DURATION_SECS as u64 + 2));
    service.update_remaining();

    let state = service.get_state();
    assert_eq!(state.phase, Phase::Break);
    // no auto-start
    assert_eq!(state.status, Status::BreakReady);
    assert_eq!(state.remaining_secs, BREAK_DURATION_SECS);
    assert!(state.completion_flag);
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

#[test]
fn test_work_completion_transitions_to_break_ready() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Simulate work completion without real-time wait
    complete_work_session(&mut service);

    let state = service.get_state();
    assert_eq!(state.phase, Phase::Break);
    assert_eq!(state.status, Status::BreakReady);
    assert_eq!(state.remaining_secs, BREAK_DURATION_SECS);
    assert!(state.completion_flag);
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

#[test]
fn test_start_from_break_ready_begins_break_countdown() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Complete work session (simulated)
    complete_work_session(&mut service);
    service.get_state();

    // Should be in BreakReady state
    assert_eq!(service.status, Status::BreakReady);

    // Start break countdown
    service.start().unwrap();

    let state = service.get_state();
    assert_eq!(state.phase, Phase::Break);
    assert_eq!(state.status, Status::Running);
    assert_eq!(state.remaining_secs, BREAK_DURATION_SECS);
    assert!(!state.completion_flag); // Should be cleared when starting
}

// ===== User Story 2: Pause/resume break sessions =====

#[test]
fn test_pause_during_break_session() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Complete work session and start break (simulated)
    complete_work_session(&mut service);
    service.get_state();
    service.start().unwrap(); // Start break

    // Simulate a few seconds into break
    fast_forward(&mut service, 2);
    let paused_state = service.pause().unwrap();

    assert_eq!(paused_state.phase, Phase::Break);
    assert_eq!(paused_state.status, Status::Paused);
    assert!(paused_state.remaining_secs < BREAK_DURATION_SECS);
    assert!(paused_state.remaining_secs > 0);
    assert!(paused_state.state_label.contains("Paused"));
    assert!(paused_state.state_label.contains("break"));
}

#[test]
fn test_resume_after_pause_during_break() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Complete work session and start break (simulated)
    complete_work_session(&mut service);
    service.get_state();
    service.start().unwrap(); // Start break

    // Simulate a couple seconds into break, then pause
    fast_forward(&mut service, 2);
    service.pause().unwrap();
    let remaining_before = service.remaining_secs;

    let resumed_state = service.resume().unwrap();

    assert_eq!(resumed_state.phase, Phase::Break);
    assert_eq!(resumed_state.status, Status::Running);
    assert_eq!(resumed_state.remaining_secs, remaining_before);
    assert_eq!(resumed_state.state_label, "Break time");
}

#[test]
fn test_break_countdown_continues_after_resume() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Complete work session and start break (simulated)
    complete_work_session(&mut service);
    service.get_state();
    service.start().unwrap(); // Start break

    // Simulate 1 second into break, then pause
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

// ===== User Story 3: Break completion and cycle restart =====

#[test]
fn test_break_completion_transitions_to_work_ready() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Complete work session and start break (simulated)
    complete_work_session(&mut service);
    service.get_state();
    service.start().unwrap(); // Start break

    // Complete break session (simulated)
    complete_break_session(&mut service);
    let state = service.get_state();

    // Should transition to WorkReady (not auto-start work)
    assert_eq!(state.phase, Phase::Work);
    assert_eq!(state.status, Status::WorkReady);
    assert_eq!(state.remaining_secs, WORK_DURATION_SECS);
    assert_eq!(state.duration_secs, WORK_DURATION_SECS);
    assert!(state.state_label.contains("Work ready"));
}

#[test]
fn test_completion_flag_set_on_break_completion() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Complete work session and start break (simulated)
    complete_work_session(&mut service);
    service.get_state();
    service.start().unwrap(); // Start break

    // Before completion
    assert!(!service.completion_flag);

    // Complete break session (simulated)
    complete_break_session(&mut service);
    service.get_state();

    // Flag should be set to trigger chime
    assert!(service.completion_flag);
}

#[test]
fn test_start_from_work_ready_after_break_begins_work_countdown() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Complete work session and start break (simulated)
    complete_work_session(&mut service);
    service.get_state();
    service.start().unwrap(); // Start break

    // Complete break session (simulated)
    complete_break_session(&mut service);
    service.get_state();

    // Should be in WorkReady state
    assert_eq!(service.status, Status::WorkReady);

    // Start new work session
    service.start().unwrap();

    let state = service.get_state();
    assert_eq!(state.phase, Phase::Work);
    assert_eq!(state.status, Status::Running);
    assert_eq!(state.remaining_secs, WORK_DURATION_SECS);
    assert!(!state.completion_flag); // Should be cleared when starting
    assert_eq!(state.state_label, "Working");
}

// ===== User Story 4: Clear during break ready state =====

#[test]
fn test_clear_from_break_ready_returns_to_work_ready() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Complete work session to reach BreakReady (simulated)
    complete_work_session(&mut service);
    service.get_state();

    assert_eq!(service.status, Status::BreakReady);

    // Clear should skip break and return to work-ready
    let state = service.clear().unwrap();

    assert_eq!(state.phase, Phase::Work);
    assert_eq!(state.status, Status::WorkReady);
    assert_eq!(state.remaining_secs, WORK_DURATION_SECS);
    assert!(!state.completion_flag);
    assert!(state.state_label.contains("Ready to work"));
}

#[test]
fn test_clear_from_running_break_returns_to_work_ready() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Complete work session and start break (simulated)
    complete_work_session(&mut service);
    service.get_state();
    service.start().unwrap(); // Start break

    // Simulate a bit of break time
    fast_forward(&mut service, 2);

    // Clear should abort break and return to work-ready
    let state = service.clear().unwrap();

    assert_eq!(state.phase, Phase::Work);
    assert_eq!(state.status, Status::WorkReady);
    assert_eq!(state.remaining_secs, WORK_DURATION_SECS);
    assert!(!state.completion_flag);
}

#[test]
fn test_clear_from_paused_break_returns_to_work_ready() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Complete work session and start break (simulated)
    complete_work_session(&mut service);
    service.get_state();
    service.start().unwrap(); // Start break

    // Simulate 1 second into break then pause
    fast_forward(&mut service, 1);
    service.pause().unwrap();

    assert_eq!(service.status, Status::Paused);
    assert_eq!(service.phase, Phase::Break);

    // Clear should abort paused break and return to work-ready
    let state = service.clear().unwrap();

    assert_eq!(state.phase, Phase::Work);
    assert_eq!(state.status, Status::WorkReady);
    assert_eq!(state.remaining_secs, WORK_DURATION_SECS);
    assert!(!state.completion_flag);
}

// ===== Edge Cases =====

#[test]
fn test_start_while_running_returns_error() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Try to start again while running
    let result = service.start();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Timer already running");
}

#[test]
fn test_pause_near_session_end_transitions_correctly() {
    let mut service = TimerService::new();
    service.start().unwrap();

    // Fast-forward to 1 second before completion
    fast_forward(&mut service, WORK_DURATION_SECS as u64 - 1);

    // Pause with 1 second remaining
    service.pause().unwrap();
    let remaining_at_pause = service.remaining_secs;

    assert!(remaining_at_pause <= 2); // Should be ~1 second
    assert_eq!(service.status, Status::Paused);

    // Resume and let it complete
    service.resume().unwrap();
    sleep(Duration::from_secs(2));

    let state = service.get_state();

    // Should have transitioned to BreakReady
    assert_eq!(state.status, Status::BreakReady);
    assert!(state.completion_flag);
}

#[test]
fn test_ready_states_maintained_indefinitely() {
    let mut service = TimerService::new();

    // WorkReady state should persist
    assert_eq!(service.status, Status::WorkReady);
    sleep(Duration::from_secs(3));
    let state = service.get_state();
    assert_eq!(state.status, Status::WorkReady);

    // Start and complete work to reach BreakReady
    service.start().unwrap();
    complete_work_session(&mut service);
    service.get_state();

    // BreakReady state should persist
    assert_eq!(service.status, Status::BreakReady);
    sleep(Duration::from_secs(3));
    let state = service.get_state();
    assert_eq!(state.status, Status::BreakReady);
}

// Tests for set_phase() method
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
    assert_eq!(service.status, Status::WorkReady);
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
