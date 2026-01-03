# Implementation Plan: Align Buttons in Single Row

**Branch**: `003-align-buttons` | **Date**: 2026-01-03 | **Spec**: [specs/003-align-buttons/spec.md](spec.md)
**Input**: Feature specification from `/specs/003-align-buttons/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Adjust the Pomodoro timer GUI to ensure all four action buttons (Start, Pause, Resume, Clear) are displayed in a single horizontal row without wrapping. The current implementation uses a flexbox container with `flex-wrap: wrap`, which can cause buttons to wrap on narrower viewports. The solution involves modifying the CSS `flex-wrap` property to `nowrap` and adjusting button padding/sizing to ensure all buttons fit on one line at default window dimensions without text truncation.

## Technical Context

**Language/Version**: HTML5/CSS3 + TypeScript  
**Primary Dependencies**: None (HTML/CSS only, TypeScript runtime already in place)  
**Storage**: N/A  
**Testing**: Visual inspection (GUI layout test)  
**Target Platform**: macOS and Linux Desktop  
**Project Type**: Single pomodoro timer application  
**Performance Goals**: No concrete plans  
**Constraints**: Maintain current button styling and colors; no functional changes  
**Scale/Scope**: One user per one app instance

**Current State**:

- Buttons are in a `.controls` div with `display: flex`, `flex-wrap: wrap`
- Gap between buttons is 10px
- Button padding is 12px 24px (vertical/horizontal)
- Container max-width is 400px
- Buttons currently can wrap if window is narrow

**Issue Identified**: The `flex-wrap: wrap` property allows buttons to wrap to multiple lines, violating FR-001 and FR-002 requirements.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Code Quality**: ✓ No new Rust code required. HTML/CSS changes follow existing style conventions.
- **Testing Standards**: ✓ Visual regression testing and manual testing sufficient for GUI layout changes.
- **User Experience Consistency**: ✓ Maintains existing button styling, colors, and interaction patterns. Layout-only change.
- **Performance Requirements**: ✓ No performance impact. Pure CSS layout adjustment.
- **Simplicity**: ✓ Single CSS property change (`flex-wrap: wrap` → `flex-wrap: nowrap`) maintains simplicity principle. No over-engineering.

**Gate Status**: ✅ **PASS** - No violations. All constitution principles satisfied.

## Project Structure

### Documentation (this feature)

```text
specs/003-align-buttons/
├── plan.md              # This file (implementation plan)
├── spec.md              # Feature specification
├── research.md          # Phase 0 (not needed for this feature)
├── data-model.md        # Phase 1 (not applicable - GUI only)
├── quickstart.md        # Phase 1 (not applicable)
└── checklists/
    └── requirements.md  # Quality checklist
```

### Source Code (repository root)

```text
src/
├── index.html           # [TO MODIFY] Button layout CSS
├── main.ts              # Existing timer logic (no changes needed)
└── assets/
    └── chime.html       # Existing asset (no changes)

src-tauri/
├── src/
│   ├── main.rs          # Rust backend (no changes needed)
│   ├── lib.rs           # Rust backend (no changes needed)
│   └── timer.rs         # Rust backend (no changes needed)
└── ... (no changes needed)
```

**Structure Decision**: This is a single-component GUI adjustment feature. Changes are minimal and localized to HTML/CSS in `src/index.html`. No database, backend, or Rust code changes required. The implementation is straightforward: modify the `.controls` flexbox CSS to prevent wrapping and adjust sizing as needed.

## Complexity Tracking

> **No violations detected. This section is omitted as all Constitution principles are satisfied.**

No complexity justifications needed. The solution is a single CSS property modification with no architectural complexity or trade-offs required.
