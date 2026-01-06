# Quickstart: Show Elapsed Time After Session Completion

**Date**: 2026-01-06  
**Feature**: [specs/007-show-elapsed-time/spec.md](specs/007-show-elapsed-time/spec.md)  
**Data Model**: [data-model.md](data-model.md)  
**IPC Contracts**: [contracts/ipc-commands.md](contracts/ipc-commands.md)

---

## Overview

Implement post-completion elapsed display with pause/resume/clear controls. Backend tracks elapsed using a dedicated accumulator; frontend renders `-MM:SS` in red while status is `complete`. Start is disabled during elapsed to force explicit clear.

---

## Part 1: Rust Backend (`src-tauri/src/timer.rs`)

1) **Extend TimerService fields**

- Add `last_completed_phase: Option<Phase>`, `elapsed_started_instant: Option<Instant>`, `elapsed_paused_secs: u32`, `elapsed_running: bool`.
- Initialize in `new()` with `None/0/false`.

1) **Start elapsed on completion**

- In `handle_completion()`:
  - Capture `completed_phase` before switching.
  - Set `phase` to next session (work→break, break→work).
  - Set `status = Status::Complete` (instead of ready).
  - Set `last_completed_phase = Some(completed_phase)`.
  - Start elapsed clock: `elapsed_running = true`, `elapsed_paused_secs = 0`, `elapsed_started_instant = Some(Instant::now())`.
  - Keep `completion_flag = true` for the first `get_state` call.

1) **Update `get_state()`**

- Compute `elapsed_secs` when `status == Complete` as `elapsed_paused_secs + (now - elapsed_started_instant)` when running.
- Populate new TimerState fields: `elapsedSecs`, `elapsedRunning`, `lastCompletedPhase`.
- Clear `completion_flag` after surfacing once.

1) **Pause/Resume for elapsed**

- `pause()` should handle `status == Complete` by stopping elapsed: move delta into `elapsed_paused_secs`, clear `elapsed_started_instant`, set `elapsed_running = false`, update `state_label` (e.g., "Paused (elapsed)").
- `resume()` should handle `status == Complete` by setting `elapsed_started_instant = Some(Instant::now())`, `elapsed_running = true`, `state_label = "Elapsed running"`.

1) **Guard `start()`**

- Reject when `status == Complete` with a clear message (require `clear()` first). Start remains valid only for `workReady | breakReady`.

1) **Clear behavior**

- `clear()` resets elapsed fields, sets `completion_flag = false`, and sets status/remaining/duration to match current `phase` (already set to next session after completion).

1) **Unit tests (`src-tauri/src/timer/tests.rs`)**

- Completion sets status=Complete, starts elapsed, keeps `completion_flag` for one poll.
- Pause/resume elapsed preserves time correctly.
- Start rejected in Complete state.
- Clear resets to next phase ready and removes `elapsedSecs`.

---

## Part 2: IPC Layer (`src-tauri/src/lib.rs`)

- Ensure `get_state`, `start_timer`, `pause_timer`, `resume_timer`, `clear_timer` surface new `TimerState` fields via serde.
- Propagate new error string for start when status is Complete.

---

## Part 3: Frontend (`src/main.ts`, `src/index.html`)

1) **Display logic**

- If `state.status === 'complete' && state.elapsedSecs !== undefined`, render `-MM:SS` in `#timer-display` with red text (`#ef4444`) and bold weight.
- Otherwise render countdown `MM:SS`.

1) **Button states**

- Start disabled when `status === 'complete'`.
- Pause enabled when `status === 'running'` or (`status === 'complete' && state.elapsedRunning`).
- Resume enabled when (`status === 'paused'`) or (`status === 'complete' && !state.elapsedRunning`).
- Clear always enabled when elapsed is present.

1) **State label**

- Show completion label using `lastCompletedPhase` (e.g., "Work session completed").

1) **Chime**

- Keep existing chime triggered by `completionFlag` edge.

1) **Styling**

- Add CSS for elapsed red text and bold weight. Ensure contrast on `#1a1a1a` background.

---

## Part 4: Manual Validation Checklist

- Run `cargo test` inside `src-tauri` (after adding new unit tests).
- App flow:
  1. Start work session; wait for completion.
  2. Verify display shows `-00:01` in red, Start disabled, Pause/Resume/Clear visible.
  3. Pause elapsed: time freezes; Resume: time resumes.
  4. Clear: timer resets to break ready (05:00) with no elapsed display.
  5. Repeat with break completion → next work ready.

- Edge: allow app minimized during elapsed; ensure time continues (monotonic clock).
