# Research: GUI Button Alignment

**Feature**: 003-align-buttons  
**Date**: 2026-01-03  
**Purpose**: Determine optimal CSS approach for single-row button layout

## Key Findings

### Decision 1: CSS Flexbox Modification
**Decision**: Change `flex-wrap: wrap` to `flex-wrap: nowrap` in `.controls` CSS class

**Rationale**:
- Current implementation already uses flexbox for button layout
- The `flex-wrap: wrap` property is the direct cause of button wrapping
- Removing `flex-wrap: wrap` or explicitly setting `flex-wrap: nowrap` prevents multi-line layout
- This is a minimal, targeted change with zero side effects

**Alternatives Considered**:
- CSS Grid with `grid-auto-flow: column` - Overkill for 4 buttons; flexbox already in place
- `white-space: nowrap` - Does not apply to flex items; flexbox property required
- Negative margins - Could work but adds unnecessary complexity
- Reducing button padding - While technically possible, violates constraint to maintain styling

**Evidence**: 
- Current code shows `.controls { display: flex; gap: 10px; justify-content: center; flex-wrap: wrap; }`
- Removing `flex-wrap: wrap` (or setting to `nowrap`) forces single-row layout
- Flexbox is W3C standard with excellent cross-browser support

### Decision 2: Button Sizing Strategy
**Decision**: Maintain current button padding (12px 24px). If needed, use `flex-shrink: 0` to prevent button compression.

**Rationale**:
- Current padding (12px 24px) provides adequate clickable surface area
- Button text is short (4-6 characters each)
- Container width (400px) should accommodate all four buttons with current sizing
- `flex-shrink: 0` ensures buttons don't compress under flex layout

**Calculation**:
```
4 buttons × ~100px (with padding/border) = ~400px
+ 3 gaps × 10px = 30px
= ~430px (slight overflow acceptable at 400px container)
```

**Alternatives Considered**:
- Reduce button padding - Violates styling consistency requirement
- Smaller font size - Violates styling consistency requirement
- Two-column layout - Violates FR-001 requirement
- Horizontal scroll - Violates accessibility standards

### Decision 3: Testing Approach
**Decision**: Visual regression testing + manual browser testing at standard window sizes

**Rationale**:
- GUI layout changes are inherently visual
- No functional code changes means no unit test requirements
- Manual testing at typical desktop dimensions (1920x1080, 1440x900, etc.) sufficient
- Browser DevTools responsive design mode ideal for validation

**Test Scenarios**:
1. Default app launch window - verify single-row layout
2. Hover states - verify no layout shift
3. Button state changes (enabled/disabled) - verify stability
4. Resize window - verify layout at various widths

### Decision 4: Browser/Platform Compatibility
**Decision**: Flexbox with standard W3C properties ensures compatibility across macOS and Linux

**Rationale**:
- Tauri targets modern Chromium-based webviews (WRY)
- Flexbox is fully supported in all modern browsers since 2015+
- No CSS vendor prefixes needed
- CSS changes are cross-platform compatible

## Implementation Summary

**File to modify**: `src/index.html`

**CSS Change**:
```css
.controls {
    display: flex;
    gap: 10px;
    justify-content: center;
    flex-wrap: nowrap;    /* Changed from: flex-wrap: wrap; */
    flex-shrink: 0;       /* Optional: prevents container shrinking */
}
```

**Impact Assessment**:
- Lines changed: 1-2
- Functions affected: None
- Tests affected: None
- Performance impact: None (pure CSS, no JavaScript)
- Breaking changes: None

**Deployment**: Single HTML file change, no build system impact
