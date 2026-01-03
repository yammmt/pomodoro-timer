# Feature Specification: Manual Break Start

**Feature Branch**: `004-manual-break-start`  
**Created**: 2026-01-03  
**Status**: Draft  
**Input**: User description: "Change timer state transition. When work session completes, don't automatically start a break session: the timer just ring a chime and shows the break time. In a break session, the user should push Start button to enable count down, same as a work session."

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.
  
  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - Manual break initiation after work session (Priority: P1)

As a desktop user, when my 25-minute work session completes, I want the timer to ring a chime and display the break time (5:00) without automatically starting the countdown, so I can decide when to actually begin my break based on my current task state.

**Why this priority**: This is the core behavior change that gives users control over their break timing, allowing them to complete a thought or reach a stopping point before starting the break countdown.

**Independent Test**: Start a work session, let it complete to 00:00, verify that a chime sounds, the display shows 5:00, but the countdown does not start automatically. Pressing Start should then begin the 5-minute break countdown.

**Acceptance Scenarios**:

1. **Given** a work timer reaches 00:00, **When** it completes, **Then** a chime sounds, the display shows "5:00" indicating break time, the timer state indicates "break ready", but the countdown does NOT start automatically.
2. **Given** the timer is in "break ready" state showing 5:00, **When** the user presses Start, **Then** the 5-minute break countdown begins and decrements normally.
3. **Given** the timer completed a work session and shows 5:00 break time, **When** the user does not press Start for an extended period, **Then** the display continues to show 5:00 without any countdown or timeout.

---

### User Story 2 - Pause and resume break session (Priority: P2)

As a user during a break, I want to pause and resume my break countdown just like I can during work sessions, so I have consistent control over all timer states.

**Why this priority**: Consistent behavior across work and break sessions improves usability and meets user expectations for control.

**Independent Test**: Start a break countdown, press Pause, verify countdown stops, then press Resume and verify countdown continues from the paused time.

**Acceptance Scenarios**:

1. **Given** a running break timer, **When** the user presses Pause, **Then** the countdown stops and the remaining time is preserved.
2. **Given** a paused break timer, **When** the user presses Resume, **Then** the countdown continues from the preserved remaining time without any loss of seconds.

---

### User Story 3 - Break completion and cycle restart (Priority: P2)

As a user, when my break completes, I want the timer to ring a chime and display the work time (25:00) ready for the next cycle, so I know it's time to return to work and can start the next session when ready.

**Why this priority**: Completes the full work-break cycle with consistent behavior, preparing users for the next work session.

**Independent Test**: Start and complete a break session, verify that a chime sounds at 00:00, the display shows 25:00, and the timer is ready for the next work session without auto-starting.

**Acceptance Scenarios**:

1. **Given** a break timer reaches 00:00, **When** it completes, **Then** a chime sounds, the display shows "25:00" indicating work time, the timer state indicates "work ready", but the countdown does NOT start automatically.
2. **Given** the timer completed a break and shows 25:00 work time, **When** the user presses Start, **Then** a new 25-minute work countdown begins.

---

### User Story 4 - Clear during break ready state (Priority: P3)

As a user, I want to press Clear while in the "break ready" state (showing 5:00 after work completion) to skip the break entirely and return to work mode (25:00), so I can continue working if I don't need a break.

**Why this priority**: Provides flexibility for users who want to skip breaks or restart their workflow.

**Independent Test**: Complete a work session to reach "break ready" state, press Clear, verify the display shows 25:00 work mode without starting a countdown.

**Acceptance Scenarios**:

1. **Given** the timer is in "break ready" state showing 5:00, **When** the user presses Clear, **Then** the display resets to 25:00 work mode and the timer state indicates "work ready".
2. **Given** the timer is reset to work mode after clearing from break ready, **When** the user presses Start, **Then** a new 25-minute work countdown begins normally.

---

### Edge Cases

- Clear pressed while a break countdown is running should stop the timer and reset to 25:00 work mode.
  - Yes
- Pressing Start multiple times while already in a running break countdown should be ignored (consistent with current behavior for work sessions).
  - Yes
- Chime should play at work completion (transition to break ready) and break completion (transition to work ready).
  - Yes
- App minimizing or losing focus during "break ready" or "work ready" states should maintain those states without timeout or auto-starting.
  - Yes

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST ring a chime when a work session reaches 00:00.
- **FR-002**: System MUST transition to "break ready" state when a work session completes, displaying 5:00 without starting the countdown.
- **FR-003**: System MUST require the user to press Start to begin the break countdown from the "break ready" state.
- **FR-004**: System MUST support Pause and Resume controls during a running break countdown, preserving remaining time accurately.
- **FR-005**: System MUST ring a chime when a break session reaches 00:00.
- **FR-006**: System MUST transition to "work ready" state when a break session completes, displaying 25:00 without starting the countdown.
- **FR-007**: System MUST allow Clear to reset from "break ready" state (showing 5:00) to "work ready" state (showing 25:00) without starting any countdown.
- **FR-008**: System MUST allow Clear to reset from a running or paused break countdown to "work ready" state (showing 25:00).
- **FR-009**: System MUST prevent duplicate timer starts by ignoring Start button presses while a countdown is already running.
- **FR-010**: System MUST maintain "break ready" and "work ready" states indefinitely until the user takes action (Start or Clear).

### Key Entities

- **Timer Session**: Represents a countdown instance with attributes: type (work or break), configured duration (25 or 5 minutes), remaining time, and state (work_ready, work_running, work_paused, break_ready, break_running, break_paused).
- **Session Transition**: Represents the completion event that triggers a chime and state change (work_running → break_ready or break_running → work_ready) without automatic countdown start.

### Assumptions

- Chime functionality is already implemented or will be implemented as part of this feature (audio notification at session completion).
- The distinction between "ready" states (break_ready, work_ready) and "running" states (break_running, work_running) needs to be clearly represented in the UI to inform users they must press Start.
- Users are expected to initiate breaks and work sessions manually; no automatic transitions to running states occur.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: When a work session completes, users hear a chime and see 5:00 displayed within 1 second, with no automatic countdown starting.
- **SC-002**: Users can remain in "break ready" state showing 5:00 for any duration without the timer auto-starting or timing out.
- **SC-003**: Pressing Start from "break ready" state begins the 5-minute break countdown within 1 second.
- **SC-004**: When a break session completes, users hear a chime and see 25:00 displayed within 1 second, with no automatic countdown starting.
- **SC-005**: Break countdown pause and resume operations preserve remaining time within ±1 second accuracy.
- **SC-006**: 90% of users can complete a full work-break cycle (including manual break start) without confusion in usability testing.
