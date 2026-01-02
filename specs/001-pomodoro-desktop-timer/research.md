# Phase 0 Research — Pomodoro Desktop Timer

## Timer accuracy (monotonic, Rust-side)
- Decision: Implement timer on Rust backend using monotonic `Instant` and `tokio::time::interval` to drive ticks and state transitions.
- Rationale: Monotonic clock avoids drift from system clock changes and is more accurate than browser `setInterval`; Rust side keeps a single source of truth.
- Alternatives considered: Frontend `setInterval` (too much drift, affected by tab throttling); spawning multiple timers (adds complexity and overlap risk).

## UI update and IPC strategy
- Decision: Expose Tauri commands for `start`, `pause`, `resume`, `clear`, and `get_state`; push periodic state via frontend polling at 1 Hz or on-demand fetch.
- Rationale: Simple command surface matches FRs, keeps IPC minimal, and allows deterministic state tests; 1 Hz refresh meets ±1s display requirement without overloading UI.
- Alternatives considered: Event streaming with custom emitters (more plumbing than needed for single timer); front-end-only state (risks overlap and drift).

## Completion indication
- Decision: Provide visual state change plus lightweight audible chime (bundled asset) on work/break completion, with a clear toggle in UI to mute if desired.
- Rationale: Meets FR-006 and SC-004 with minimal footprint; local asset avoids network dependency.
- Alternatives considered: OS-level notifications (adds permissions and platform differences); vibration/haptics (not available on desktop reliably).

## Cycle logic and safety
- Decision: Maintain finite state machine with states `idle_work`, `work_running`, `work_paused`, `break_running`, `break_paused`, `complete_break`; deny `start` when running and reset to `idle_work` on clear.
- Rationale: Explicit states prevent overlapping timers and simplify tests; maps directly to acceptance scenarios.
- Alternatives considered: Single boolean running flag (insufficient to express work/break modes and pause semantics); queued timers (unneeded for single-cycle scope).
