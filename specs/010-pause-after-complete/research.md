# Research

## Decision: Represent paused overtime explicitly

- **Decision**: Add a paused-overtime representation in timer state (store frozen overtime seconds and expose a distinct paused status).
- **Rationale**: Allows overtime display to freeze without changing active session data, and keeps Resume behavior deterministic.
- **Alternatives considered**: Reusing existing `Paused` status for overtime (rejected due to ambiguity with active-session pause state).

## Decision: Enable Pause/Resume controls in completed state

- **Decision**: Update UI enablement rules so Pause is available when status is `complete`, and Resume is available when status is `overtimePaused`.
- **Rationale**: Matches the requirement to pause negative time after completion and keeps controls consistent across phases.
- **Alternatives considered**: Adding a separate overtime control (rejected to keep UI simple).
