# Data Model

## TimerState (existing, updated)

- **status**: `workReady | breakReady | running | paused | complete | overtimePaused`
- **overtimeSecs**: `number | null` (already present)
- **overtimePausedSecs**: `number | null`

## State Transitions (new)

- `complete` → `overtimePaused` on Pause
- `overtimePaused` → `complete` on Resume
