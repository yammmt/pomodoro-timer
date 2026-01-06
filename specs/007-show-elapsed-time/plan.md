# Implementation Plan: Show Elapsed Time After Session Completion

**Branch**: `007-show-elapsed-time` | **Date**: 2026-01-06 | **Spec**: [specs/007-show-elapsed-time/spec.md](specs/007-show-elapsed-time/spec.md)
**Input**: Feature specification from `/specs/007-show-elapsed-time/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Display elapsed time after a work/break session completes, showing a negative timer ("-MM:SS") in bright red. Backend tracks post-completion elapsed time with pause/resume/clear controls; frontend renders elapsed state distinctly and keeps existing clear workflow. Timer accuracy must remain within ±1s while polling at 1 Hz without adding new storage or services.

## Technical Context

**Language/Version**: Rust 1.92 (backend), TypeScript/Vite (frontend)  
**Primary Dependencies**: tauri 2.9, serde, @tauri-apps/api  
**Storage**: None (in-memory timer state only)  
**Testing**: cargo test (Rust); manual UI verification for elapsed display  
**Target Platform**: macOS and Linux Desktop (`ubuntu-latest`)  
**Project Type**: single  
**Performance Goals**: 1 Hz UI polling; user actions (pause/resume/clear) respond <100ms; elapsed drift ≤1s over 10 minutes  
**Constraints**: Small window, simple UI; elapsed text uses danger red `#ef4444`; keep Clear flow unchanged; avoid new storage/background services  
**Scale/Scope**: one user per one app  
**Resolved Behaviors**: Elapsed counts while app runs; resets to ready on app restart (no persistence).

## Constitution Check (Pre-Design Gate)

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Code Quality: PASS with plan to keep state changes in `timer.rs`, add unit tests for elapsed lifecycle, and avoid unsafe/unwrap on public paths.
- Testing Standards: PASS contingent on adding Rust unit tests for completion→elapsed, pause/resume/clear, and timer accuracy; no automated frontend tests planned.
- User Experience Consistency: PASS with reuse of existing buttons and modal; need accessible red for elapsed text (pending color choice).
- Performance Requirements: PASS with 1 Hz polling and monotonic `Instant` for elapsed; ensure no busy loops or extra threads.
- Simplicity: PASS by reusing existing IPC commands and in-memory state; no new storage or services. 

## Project Structure

### Documentation (this feature)

```text
specs/007-show-elapsed-time/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src-tauri/
├── capabilities/
├── icons/
└── src/

src/
└── main.ts
```

**Structure Decision**: Single-project Tauri app; elapsed timer logic lives in `src-tauri/src/timer.rs`, UI updates in `src/main.ts` + `src/index.html`. No new modules or packages required; reuse existing IPC surface and CSS.

## Complexity Tracking

No Constitution violations identified; table not required.

## Constitution Check (Post-Design Validation)

*GATE: Verified after Phase 1 design. All checks passed; no violations identified.*

### Code Quality
✓ **PASS** – Elapsed logic contained in `timer.rs`; new fields isolated. Start guard prevents ambiguous states. No new dependencies; avoid `unwrap` on IPC paths.

### Testing Standards
✓ **PASS** – Planned Rust unit tests cover completion→elapsed, pause/resume, clear reset, and start guard. Manual UI validation documented in quickstart.

### User Experience Consistency
✓ **PASS** – Reuses existing buttons and modal; elapsed text uses consistent danger red and bold for clarity; Start disabled to enforce clear workflow.

### Performance Requirements
✓ **PASS** – 1 Hz polling with monotonic `Instant`; no extra threads; elapsed math O(1). Button actions expected <100ms.

### Simplicity
✓ **PASS** – No storage, threads, or new commands; reuses existing IPC surface with added fields. YAGNI maintained.

**Final Status**: GATE PASSED – Proceed to Phase 2 implementation.
