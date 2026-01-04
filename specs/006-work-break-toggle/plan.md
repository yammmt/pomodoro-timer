# Implementation Plan: Work/Break Mode Toggle

**Branch**: `006-work-break-toggle` | **Date**: 2026-01-04 | **Spec**: [specs/006-work-break-toggle/spec.md](specs/006-work-break-toggle/spec.md)
**Input**: Feature specification from `/specs/006-work-break-toggle/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Add Work/Break mode toggle buttons at the top of the desktop window to let users instantly switch between work (25 min) and break (5 min) sessions. Active button shows emphasized CSS styling. Switching sessions pauses the current timer without clearing remaining time. Tauri IPC commands route mode changes to the Rust timer backend, updating session type and display duration while preserving paused timer state across sessions.

## Technical Context

**Language/Version**: Rust 1.92  
**Primary Dependencies**: tauri 2.9  
**Storage**: N/A  
**Testing**: cargo test  
**Target Platform**: macOS and Linux Desktop (`ubuntu-latest`)  
**Project Type**: single  
**Performance Goals**: NO concrete plans  
**Constraints**: NO concrete plans  
**Scale/Scope**: one user per one app

## Constitution Check (Post-Design Validation)

*GATE: Verified after Phase 1 design. All checks passed; no violations identified.*

### Code Quality
✓ **PASS** – New `set_phase()` method follows Rust conventions. Idempotent design prevents state corruption. Unit tests included in `timer/tests.rs`. No unsafe code or unwrap() on public paths.

### Testing Standards
✓ **PASS** – Five unit test cases in `timer/tests.rs` cover: idempotent calls, time preservation across switches, running→paused transitions, phase isolation, and error handling. Frontend integration tests documented in quickstart.md.

### User Experience Consistency
✓ **PASS** – Two left-aligned buttons match existing UI simplicity. CSS `.active` class provides clear visual feedback. Behavior matches Pomodoro conventions (no auto-start, preserves paused time). Accessible keyboard navigation via standard button elements.

### Performance Requirements
✓ **PASS** – `set_phase()` is O(1) with no heap allocations for the toggle operation. Button click invokes IPC → backend update → state return; sub-100ms latency on modern systems. No threading or async complexity introduced.

### Simplicity
✓ **PASS** – Feature uses minimal state fields (2 new Optional<u32> fields). No dependency additions. No new storage, database, or external services. Pure functional state machine. YAGNI satisfied: solves only the work/break toggle requirement without feature creep.

**Final Status**: GATE PASSED (Re-evaluation) – Design maintains all Constitution principles. Ready for Phase 2 implementation.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# This single project
src-tauri/
├── capabilities/
├── icons/
└── src/

src/
└── main.rs
```

**Structure Decision**: Single project structure matches current app architecture. Mode toggle state stored in existing timer module. Frontend button component in TypeScript/Vite; backend session type stored in Rust timer state. No new storage or project splits needed; feature fits within existing modular design.

## Complexity Tracking

No Constitution violations identified. All decisions aligned with core principles (simplicity, code quality, testing standards).
