# IPC Commands Contract

## Commands

### `pause_timer`

- **When valid**: `status` is `running` or `complete`
- **Effect**:
  - If `running`: existing pause behavior
  - If `complete`: transitions to `overtimePaused` and freezes overtime display

### `resume_timer`

- **When valid**: `status` is `paused` or `overtimePaused`
- **Effect**:
  - If `paused`: existing resume behavior
  - If `overtimePaused`: transitions to `complete` and resumes overtime counting

### `get_state`

- **Returns**: `TimerState` with optional `overtimePausedSecs` when `status` is `overtimePaused`
