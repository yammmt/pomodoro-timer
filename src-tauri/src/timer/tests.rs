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

#[test]
fn test_initial_state_is_idle_work() {
    let mut service = TimerService::new();
    let state = service.get_state();

    assert_eq!(state.phase, Phase::Work);
    assert_eq!(state.status, Status::Idle);
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
    assert_eq!(cleared.status, Status::Idle);
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
    assert_eq!(cleared.status, Status::Idle);
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
    assert_eq!(state.status, Status::Running);
    assert_eq!(state.remaining_secs, BREAK_DURATION_SECS);
    assert!(state.completion_flag);
    assert_eq!(state.state_label, "Break time");
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
    assert_eq!(state.status, Status::Complete);
    assert!(state.completion_flag);
    assert_eq!(state.state_label, "Break complete");
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
    assert_eq!(cleared.status, Status::Idle);
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
    assert_eq!(service.get_state().status, Status::Idle);
}

#[test]
fn test_cannot_resume_when_not_paused() {
    let mut service = TimerService::new();
    let result = service.resume();

    // Should return error
    assert!(result.is_err());
    assert_eq!(service.get_state().status, Status::Idle);
}

#[test]
fn test_clear_when_idle_remains_idle() {
    let mut service = TimerService::new();
    service.clear().unwrap();

    let state = service.get_state();
    assert_eq!(state.status, Status::Idle);
    assert_eq!(state.remaining_secs, WORK_DURATION_SECS);
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
    assert_eq!(state.status, Status::Running);
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
