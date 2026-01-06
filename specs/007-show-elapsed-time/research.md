# Research: Show Elapsed Time After Session Completion

**Date**: 2026-01-06  
**Feature**: [specs/007-show-elapsed-time/spec.md](specs/007-show-elapsed-time/spec.md)

## Findings

1) **Elapsed display color**  
- **Decision**: Use existing danger red `#ef4444` for elapsed text, with bold weight on dark background.  
- **Rationale**: Matches current Clear button palette, keeps UI consistent, and provides ~4.7:1 contrast against `#1a1a1a` background (meets WCAG AA for large text).  
- **Alternatives considered**: `#ff4d4f` (slightly brighter but harsher), `#f87171` (lower contrast on dark background).

2) **Elapsed continuity across app lifecycle**  
- **Decision**: Keep elapsed counting only while the app is running; on app restart, reset to ready state.  
- **Rationale**: Avoids adding persistence/storage; aligns with current countdown behavior and simplicity principle. Background/minimized windows still count using monotonic clock.  
- **Alternatives considered**: Persist completion timestamp and resume on launch (adds storage and edge cases); pause elapsed automatically on window blur (breaks user expectation to keep counting).

3) **Timekeeping and pause/resume model**  
- **Decision**: Track elapsed time with a dedicated accumulator using `Instant` + `elapsed_paused_secs`, separate from countdown `remaining_secs`. Run updates at 1 Hz via existing polling.  
- **Rationale**: Using monotonic `Instant` prevents drift; separating elapsed from countdown avoids negative remaining values and keeps start/pause/resume logic clear.  
- **Alternatives considered**: Reuse `remaining_secs` with negative values (complicates ready-state logic and clear button enablement); spawn background thread/ticker (unnecessary complexity for 1 Hz UI polling).
