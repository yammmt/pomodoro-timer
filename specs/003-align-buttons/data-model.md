# UI Data Model: Button Layout Structure

**Feature**: 003-align-buttons  
**Date**: 2026-01-03  
**Scope**: GUI component structure for pomodoro timer controls

## Layout Components

### Container Structure

```
<body>
  └── <div class="container">
      ├── <div id="state-label">       [Status text: Ready/Working/Break]
      ├── <div id="timer-display">     [Timer countdown: MM:SS]
      └── <div class="controls">       [Button row - MODIFIED]
          ├── <button id="start-btn">
          ├── <button id="pause-btn">
          ├── <button id="resume-btn">
          └── <button id="clear-btn">
```

### Controls Component (Modified Element)

**Element**: `.controls` div  
**Type**: Flex container  
**Children**: 4 button elements  
**Properties**:

| Property | Current | Change | Purpose |
|----------|---------|--------|---------|
| `display` | `flex` | No change | Enable flexbox layout |
| `gap` | `10px` | No change | Spacing between buttons |
| `justify-content` | `center` | No change | Center buttons horizontally |
| `flex-wrap` | `wrap` | → `nowrap` | Prevent button wrapping |
| `flex-shrink` | N/A | Add `0` | Prevent flex compression |

### Button Elements (Unchanged)

**Elements**: `#start-btn`, `#pause-btn`, `#resume-btn`, `#clear-btn`

| Property | Value | Notes |
|----------|-------|-------|
| Padding | 12px 24px | Unchanged - maintains UX |
| Font-size | 16px | Unchanged - maintains UX |
| Border-radius | 6px | Unchanged - maintains UX |
| Colors | Various (green/orange/blue/red) | Unchanged - maintains UX |
| Width | Auto | Implicit - based on padding + text |
| Flex properties | None needed | Flex children by default |

## Styling Rules

### CSS Rules Modified

```css
.controls {
    display: flex;
    gap: 10px;
    justify-content: center;
    flex-wrap: nowrap;      /* PRIMARY CHANGE */
}
```

### CSS Rules Preserved

All button-specific rules remain unchanged:

- `#start-btn`, `#pause-btn`, `#resume-btn`, `#clear-btn` - colors, hover states
- `button:disabled` - opacity and cursor
- `button` - general padding, border, transition

## Accessibility

- No ARIA changes needed - existing labels are sufficient
- No semantic HTML changes - structure remains valid
- Layout change is pure presentation
- All buttons remain focusable and interactive

## Responsive Behavior

**At default window**: All buttons visible in single row  
**On window resize**: Buttons stay in single row (may overflow on very narrow windows < 300px, which is not a typical desktop scenario)  
**On button state change**: Layout stable - no reflow due to `flex-wrap: nowrap`

## Constraints & Assumptions

1. **Button text length**: Remains 4-6 characters (Start, Pause, Resume, Clear)
2. **Spacing**: 10px gap between buttons remains constant
3. **Container width**: 400px max-width provides sufficient space
4. **Button padding**: 12px 24px maintains consistent UX
5. **No new buttons**: Feature scope is 4 existing buttons only

## Testing Verification

**Visual checklist**:

- [ ] All 4 buttons visible on single line
- [ ] No text truncation
- [ ] Consistent spacing between buttons (10px)
- [ ] No layout shift when hovering
- [ ] No layout shift when toggling button disabled state
- [ ] No layout shift on window resize (within typical desktop range)

**Affected Files**:

- `src/index.html` - CSS modification only

**Not affected**:

- `src/main.ts` - No TypeScript changes
- `src-tauri/src/` - No Rust changes
- Other HTML files - Isolated change to index.html
