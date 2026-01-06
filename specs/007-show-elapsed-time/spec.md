# Feature Specification: Show Elapsed Time After Session Completion

**Feature Branch**: `007-show-elapsed-time`  
**Created**: 2026-01-06  
**Status**: Draft  
**Input**: User description: "The timer shows passed times after the work/break session has completed. This elapsed time is shown like "-01:23" (elapsed by 1 minute and 23 seconds) with a little bright red color. The user could pause, resume and clear this time. If the user want to clear elapsed time, he/she has to click the Clear button, same as the current revision."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - View Elapsed Time After Session Ends (Priority: P1)

When a work or break session completes, the user needs to see how much time has passed since completion so they can track their overtime or break duration.

**Why this priority**: This is the core value of the feature - displaying elapsed time is the fundamental requirement that users depend on to understand session completion status.

**Independent Test**: Can be fully tested by completing a session (work or break) and verifying the elapsed time display appears and updates correctly. This alone delivers the core feature value.

**Acceptance Scenarios**:

1. **Given** a Pomodoro work session is running, **When** the timer reaches 00:00, **Then** the display transitions to show "-00:01" (elapsed time format) in red color
2. **Given** an elapsed time is being displayed, **When** 1 minute and 23 seconds have passed since session completion, **Then** the display shows "-01:23"
3. **Given** a session has just completed, **When** the user views the timer, **Then** the elapsed time display is prominently visible with bright red color to distinguish it from active sessions
4. **Given** the app is displaying elapsed time, **When** the session originally completed 5 minutes ago, **Then** the elapsed display continues to show "-05:00" and increments in real-time

---

### User Story 2 - Pause Elapsed Time (Priority: P2)

The user may want to pause the elapsed time counter to freeze the display at a specific moment for reference or documentation purposes.

**Why this priority**: Pausing provides users control over the elapsed time display, useful when they want to document or review a specific elapsed duration without the counter continuously incrementing.

**Independent Test**: Can be fully tested by allowing a session to complete, elapsed time to display, then pausing it and verifying the counter stops incrementing. Provides independent pause functionality value.

**Acceptance Scenarios**:

1. **Given** elapsed time is displaying and incrementing, **When** the user clicks the Pause button, **Then** the elapsed counter stops and the current time is frozen on display
2. **Given** elapsed time is paused, **When** 10 seconds pass, **Then** the displayed elapsed time remains unchanged
3. **Given** elapsed time is paused, **When** the user views the UI, **Then** there is a visual or UI indicator showing the paused state (e.g., button state change or display styling)

---

### User Story 3 - Resume Elapsed Time Counter (Priority: P2)

After pausing the elapsed time, the user should be able to resume the counter to continue tracking time from where it was paused.

**Why this priority**: Resume capability provides complete control flow alongside pause - users can temporarily freeze the display and then continue tracking without losing the elapsed duration.

**Independent Test**: Can be fully tested by pausing elapsed time, then resuming it and verifying the counter continues from the paused value. Provides independent resume functionality.

**Acceptance Scenarios**:

1. **Given** elapsed time is paused at "-02:15", **When** the user clicks the Resume button, **Then** the elapsed counter resumes incrementing from "-02:15"
2. **Given** elapsed time is resumed, **When** 30 seconds pass, **Then** the display shows "-02:45"
3. **Given** elapsed time is resuming, **When** the user views the UI, **Then** the paused indicator is cleared and the counter appears active again

---

### User Story 4 - Clear Elapsed Time (Priority: P1)

The user should be able to clear the elapsed time display to reset the timer back to a ready state for the next session, using the same Clear button interaction pattern as the current revision.

**Why this priority**: Clearing is essential for moving to the next session - without it, users are stuck viewing the previous session's elapsed time. This is a core workflow action.

**Independent Test**: Can be fully tested by displaying elapsed time and clicking Clear, then verifying the elapsed time display is removed and the timer is ready for the next session. Delivers immediate productivity value.

**Acceptance Scenarios**:

1. **Given** elapsed time is displaying (e.g., "-01:23"), **When** the user clicks the Clear button, **Then** the elapsed time display is removed and the timer returns to its initial "ready" state
2. **Given** elapsed time is paused at "-05:00", **When** the user clicks the Clear button, **Then** the elapsed time is cleared regardless of paused state
3. **Given** the timer is in ready state after clearing, **When** the user starts a new work or break session, **Then** the new session begins normally with the active countdown timer

---

### Edge Cases

- What happens when the app is closed while elapsed time is displaying? (User expectations: elapsed time should be preserved or reset based on app state management)
  - Same as the count down: keep counting.
- How does the system handle elapsed time display if a user manually pauses and resumes a work/break session before it completes? (The elapsed time feature only activates after session completion)
  - Yes, this feature only activates after session has completed.
- What is the maximum displayable elapsed time? (e.g., after 24 hours - should wrap or maintain display)
  - Same as the current displayed area, that is, 99:59.
- Does elapsed time display appear for both work and break sessions equally? (Assumption: yes, both session types show elapsed time)
  - Yes, enabled in both sessions.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST display elapsed time in "-MM:SS" format (negative sign, minutes, seconds) after a work or break session completes
- **FR-002**: System MUST display elapsed time in bright red color to visually distinguish it from active countdown timers
- **FR-003**: Users MUST be able to pause the elapsed time counter, which halts the incrementing display
- **FR-004**: Users MUST be able to resume the paused elapsed time counter, continuing from where it was paused
- **FR-005**: Users MUST be able to clear the elapsed time display using the Clear button, returning the timer to ready state
- **FR-006**: System MUST continue incrementing elapsed time in real-time while in the elapsed display state (unless paused)
- **FR-007**: System MUST support elapsed time display for both work sessions and break sessions
- **FR-008**: Clear button action MUST work on paused elapsed time displays (no special state handling required)

### Key Entities

- **Elapsed Time State**: Represents the elapsed time counter after session completion, with attributes:
  - Duration (in seconds)
  - Display format ("-MM:SS")
  - Color styling (bright red)
  - Pause state (paused/active)
  - Session type context (work/break - for consistency in display)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can pause, resume, and clear elapsed time displays without confusion (measured by user testing: 95% task completion rate on first interaction)
- **SC-002**: Elapsed time display is immediately distinguishable from active countdown timers (measured by visual contrast: bright red color meets WCAG accessibility standards for contrast)
- **SC-003**: Pause/Resume/Clear operations respond to user input within 100ms (measured by responsiveness testing to ensure perceived instant feedback)
- **SC-004**: Elapsed time counter accurately tracks time to within 1 second over a 10-minute elapsed period (measured by timer accuracy testing)

## Assumptions

- The app already has session completion detection logic; this feature builds on that existing capability
- The "Clear button" interaction pattern referred to is already implemented in the current revision
- Pause and Resume functionality will use existing UI button patterns in the app
- Both work and break sessions follow identical elapsed time display and control behavior
- The bright red color will be defined in the app's existing color palette or theme system
- Elapsed time increments at 1-second intervals matching typical UI timer precision
