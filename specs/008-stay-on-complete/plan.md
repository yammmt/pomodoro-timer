# Implementation Plan: Stay on Completed Session

**Branch**: `008-stay-on-complete` | **Date**: 2026-01-14 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Remove automatic session transitions when timers reach 00:00. When a work session completes, the timer stays in work mode showing 00:00 instead of auto-switching to break. When a break session completes, it stays in break mode. Users manually control session transitions by clicking the Work/Break buttons. The implementation modifies the `handle_completion()` method in `timer.rs` to stay in the current phase rather than switching phases, while preserving the completion indication behavior.

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

✓ **PASS** – Modification to `handle_completion()` method follows existing Rust conventions. Changes are minimal and localized to completion logic. No new unsafe code. Unit tests will be added to verify stay-on-complete behavior.

### Testing Standards

✓ **PASS** – Unit tests will be added in `timer/tests.rs` to cover: work session completion stays in work mode, break session completion stays in break mode, manual transitions after completion, Start button functionality after completion, and completion flag behavior.

### User Experience Consistency

✓ **PASS** – Feature gives users explicit control over session transitions, aligning with desktop app conventions. Completion indications remain unchanged. Behavior is consistent across both work and break sessions. No breaking changes to existing UI interactions.

### Performance Requirements

✓ **PASS** – Changes to `handle_completion()` are O(1) with no additional allocations. No performance degradation expected. Removing auto-transition logic actually simplifies the code path slightly.

### Simplicity

✓ **PASS** – Feature simplifies behavior by removing automatic phase switching. No new state fields required. No new dependencies. Implementation is a straightforward modification of existing completion handler. YAGNI satisfied: removes unwanted auto-switch behavior without adding complexity.

**Final Status**: GATE PASSED (Post-Design) – All Constitution principles maintained. Ready for Phase 2 implementation.

## Project Structure

### Documentation (this feature)

```text
specs/008-stay-on-complete/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command) - NOT NEEDED for this feature
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# This single project
src-tauri/
├── capabilities/
├── icons/
└── src/
    ├── lib.rs          # IPC commands (no changes needed)
    ├── main.rs         # Entry point (no changes needed)
    └── timer.rs        # Timer logic (modify handle_completion)
        └── tests.rs    # Unit tests (add new test cases)

src/
├── index.html          # UI structure (no changes needed)
└── main.ts             # Frontend logic (no changes needed)
```

**Structure Decision**: Single project structure maintained. All changes localized to the `handle_completion()` method in `timer.rs`. No frontend changes required since the existing Work/Break buttons (from feature 006) already provide manual session switching. No new IPC commands needed. Feature is a pure backend behavior modification.

## Complexity Tracking

No Constitution violations identified. All decisions aligned with core principles (simplicity, code quality, testing standards). The feature actually reduces complexity by removing automatic phase transitions.
