---
status: complete
created: 2025-12-10
priority: medium
tags:
- ui
- validation
- tokens
- feature
depends_on:
- 035-live-specs-showcase
- 018-spec-validation
- 069-token-counting-utils
parent: 168-leanspec-orchestration-platform
created_at: 2025-12-10T06:39:09.954Z
updated_at: 2026-02-02T07:38:23.632420303Z
transitions:
- status: in-progress
  at: 2026-01-29T01:44:36.360075409Z
- status: planned
  at: 2026-01-30T01:45:55.142481Z
---
# UI Display of Tokens and Validation Results

> **Status**: 🗓️ Planned · **Priority**: Medium · **Created**: 2025-12-10 · **Tags**: ui, validation, tokens, feature
>
> **Depends On**: 035-live-specs-showcase, 018-spec-validation, 069-token-counting-utils

## Overview

Add token counts and validation results information to the LeanSpec web UI to provide immediate visibility into spec quality metrics. Currently, users must run CLI commands (`harnspec tokens`, `harnspec validate`) to see this information, but having it displayed directly in the UI would improve workflow efficiency and help maintain spec quality standards.

**Why now?**

- Token counts and validation status are critical quality indicators
- Users currently need to switch between CLI and web UI
- Improves adherence to Context Economy principles by making metrics visible
- Supports the AI-first workflow by showing spec readiness for LLM consumption

## Design

### Visual Design System

**Color Coding (Aligned with Existing Badge System):**

| Threshold | Token Range | Background | Text | Dark Background | Dark Text |
|-----------|-------------|------------|------|-----------------|-----------|
| Optimal | <2K | `bg-green-100` | `text-green-800` | `dark:bg-green-900/30` | `dark:text-green-400` |
| Good | 2-3.5K | `bg-blue-100` | `text-blue-800` | `dark:bg-blue-900/30` | `dark:text-blue-400` |
| Warning | 3.5-5K | `bg-orange-100` | `text-orange-800` | `dark:bg-orange-900/30` | `dark:text-orange-400` |
| Critical | >5K | `bg-red-100` | `text-red-800` | `dark:bg-red-900/30` | `dark:text-red-400` |

**Validation Status Icons:**

- Pass: `CheckCircle2` (green, same as `status=complete`)
- Warning: `AlertTriangle` (orange, same as `priority=high`)  
- Fail: `XCircle` (red, same as `priority=critical`)
- Loading: `Loader2` with `animate-spin` class

### Component Specifications

#### TokenButton Component (Clickable)

**Props Interface:**

```typescript
interface TokenButtonProps {
  count: number;
  status: 'optimal' | 'good' | 'warning' | 'critical';
  breakdown?: TokenBreakdown;  // Opens in dialog on click
  onClick?: () => void;        // Opens TokenDetailsDialog
  showIcon?: boolean;          // Show FileText icon (default: true)
  size?: 'sm' | 'md';          // sm: h-5 for lists, md: h-6 for detail
  className?: string;
}
```

**Visual Specifications:**

```
Base:       Button variant with threshold color background
Height:     h-5 (20px) for lists, h-6 (24px) for detail
Padding:    px-2 py-0.5
Font:       text-xs font-medium
Format:     "2.4k" for >999, "456" for <1000
Icon:       FileText from lucide-react, h-3.5 w-3.5
Gap:        gap-1.5 between icon and text
Hover:      Slight brightness increase (hover:brightness-110)
Cursor:     cursor-pointer
Transition: transition-all duration-200
```

**Examples:**

```tsx
// List view - clickable button
<TokenButton count={2450} status="good" onClick={() => openTokenDialog(spec)} />
// Renders: [📄 2.4k] blue button, clickable

// Detail view - larger clickable button
<TokenButton count={2450} status="good" size="md" onClick={openDialog} />
// Renders: [📄 2.4k tokens] blue button, clickable

// Critical - red warning button
<TokenButton count={5200} status="critical" onClick={openDialog} />
// Renders: [📄 5.2k] red button, clickable
```

#### ValidationButton Component (Clickable)

**Props Interface:**

```typescript
interface ValidationButtonProps {
  status: 'pass' | 'warn' | 'fail' | 'loading';
  errorCount?: number;         // Number of warnings/errors
  errors?: ValidationError[];  // Opens in dialog on click
  onClick?: () => void;        // Opens ValidationDialog
  size?: 'sm' | 'md';
}
```

**Visual Specifications:**

```
Base:       Button variant, ghost for pass, colored for warn/fail
Pass:       text-green-600 hover:bg-green-50 (ghost style)
Warning:    bg-orange-100 text-orange-800 (solid style)
Fail:       bg-red-100 text-red-800 (solid style)
Height:     h-5
Padding:    px-2
Icon:       CheckCircle2/AlertTriangle/XCircle, h-4 w-4
Count:      Badge overlay when errorCount > 0
Hover:      hover:brightness-110 for solid, hover:bg-opacity-80 for ghost
Cursor:     cursor-pointer
```

**Examples:**

```tsx
// Pass - subtle ghost button
<ValidationButton status="pass" onClick={openDialog} />
// Renders: [✓] green ghost button

// Warning - solid orange button with count
<ValidationButton status="warn" errorCount={3} onClick={openDialog} />
// Renders: [⚠ 3] orange solid button

// Fail - solid red button
<ValidationButton status="fail" errorCount={2} onClick={openDialog} />
// Renders: [✗ 2] red solid button
```

#### TokenDetailsDialog Component

**Props Interface:**

```typescript
interface TokenDetailsDialogProps {
  open: boolean;
  onClose: () => void;
  specName: string;
  tokenCount: number;
  status: 'optimal' | 'good' | 'warning' | 'critical';
  breakdown: TokenBreakdown;
  estimatedPerformance?: number;
}
```

**Dialog Layout:**

```
┌─ Context Economy: spec-name ───────────────────────────────┐
│                                                             │
│  [Hero Section]                                            │
│  ┌──────────────────────────────────────┐                  │
│  │ 📄 2,450 tokens                      │                  │
│  │ Status: Good (2K - 3.5K)            │                  │
│  └──────────────────────────────────────┘                  │
│                                                             │
│  [Progress Bar]                                            │
│  ████████████████████░░░░  80%                             │
│  0──────2K─────3.5K────5K                                  │
│                                                             │
│  [Breakdown]                                               │
│  ┌────────────────────────────────────┐                    │
│  │ Prose       ████████ 1,800 (73%)  │                    │
│  │ Code        ██       500 (20%)    │                    │
│  │ Frontmatter █        150 (7%)     │                    │
│  └────────────────────────────────────┘                    │
│                                                             │
│  [Performance Estimate]                                    │
│  ● Current effectiveness: ~92%                             │
│  ● Remaining: 1,550 tokens until warning                   │
│                                                             │
│              [Close]                                       │
└─────────────────────────────────────────────────────────────┘
```

**Visual Specs:**

- Dialog width: `w-[min(500px,90vw)]`
- Hero section: Large icon + count + status label
- Progress bar: h-3 with gradient or solid color
- Breakdown: Bar chart style with mini progress bars
- Performance: Bullet list with helpful context

#### ValidationDialog Component

**Props Interface:**

```typescript
interface ValidationDialogProps {
  open: boolean;
  onClose: () => void;
  specName: string;
  status: 'pass' | 'warn' | 'fail';
  errors: ValidationError[];
  lastChecked: string;
}
```

**Pass State Layout:**

```
┌─ Validation: spec-name ────────────────────────────────────┐
│                                                             │
│              ✓                                              │
│         All checks passed!                                  │
│                                                             │
│    This spec meets all LeanSpec quality standards.         │
│                                                             │
│    Last checked: 2 minutes ago                             │
│                                                             │
│              [Close]                                       │
└─────────────────────────────────────────────────────────────┘
```

**Issues State Layout:**

```
┌─ Validation Issues: spec-name ─────────────────────────────┐
│                                                             │
│              ⚠                                              │
│         3 Warnings Found                                    │
│                                                             │
│  ┌─ Warning 1 ─────────────────────────┐                   │
│  │ Missing Test section                │                   │
│  │                                     │                   │
│  │ Recommended: Add a Test section    │                   │
│  │ with specific test criteria.       │                   │
│  │                                     │                   │
│  │ [Quick Fix] [View in Spec →]       │                   │
│  └─────────────────────────────────────┘                   │
│                                                             │
│  ┌─ Warning 2 ─────────────────────────┐                   │
│  │ Title exceeds 80 characters         │                   │
│  │ Current: 85 characters              │                   │
│  └─────────────────────────────────────┘                   │
│                                                             │
│              [Close]                                       │
└─────────────────────────────────────────────────────────────┘
```

**Visual Specs:**

- Pass state: Centered icon + success message
- Issues state: Stacked cards with error details
- Error cards: border-l-4 with severity color
- Actions: "Quick Fix" button (if applicable) + "View in Spec" link
- Last checked: Small muted text at bottom

#### TokenProgressBar Component

**Visual Specifications:**

```
Container:    w-full h-2 bg-muted rounded-full overflow-hidden
Fill:         Color matches threshold (green/blue/orange/red)
Markers:      Absolute positioned ticks at 40%, 70%, 100%
Marker style: w-px h-3 bg-border absolute
Animation:    transition-all duration-500 ease-out
```

**Example:**

```tsx
<TokenProgressBar current={4200} max={5000} />
// Shows orange bar at 84% with ticks at 2K, 3.5K, 5K
```

### UI Placement & Layouts

#### Spec List View Enhancement

**Current Layout (ListView.tsx:24-45):**

```
┌────────────────────────────────────────────────────┐
│ [#123] Title                    [Status][Priority]│
│ spec-name                                           │
│ [tag1] [tag2]                                       │
└────────────────────────────────────────────────────┘
```

**Enhanced Layout:**

```
┌────────────────────────────────────────────────────────────┐
│ [#123] Title          [2.4k ↗][Status][Priority][✓ ↗]     │
│ spec-name                                                   │
│ [tag1] [tag2]                                               │
└────────────────────────────────────────────────────────────┘
```

**Positioning:**

- Token badge (clickable button) between title and status badge
- Validation button at far right (last in badge row)
- Both badges have `cursor-pointer` and hover state
- All badges use `h-5` height for consistency
- Gap: `gap-2` between badges

#### Spec Detail Page Enhancement

**Metadata Row Addition (after Line 422):**

```
Created: Jan 15 • Updated: Jan 20 (2 days ago) • Name: spec-name • Assignee: user
↓
Created: Jan 15 • Updated: Jan 20 (2 days ago) • [📄 2.4k tokens ↗] • [✓ Validation ↗]
```

**Right Sidebar (clean - no cards):**

- TOC only, no token/validation cards taking up space
- Click badges in header to open dialogs

#### Token Details Dialog (on click)

```
┌─ Context Economy ───────────────────────────────────────────┐
│                                                             │
│  ┌──────────────────────────────────────┐                  │
│  │ 📄                                    │                  │
│  │ 2,450 tokens                         │                  │
│  │ Status: Good                         │                  │
│  └──────────────────────────────────────┘                  │
│                                                             │
│  ████████████████████░░░░  80%                             │
│  0        2K       3.5K      5K                            │
│                                                             │
│  ─────────────────────────────────────                     │
│                                                             │
│  Breakdown:                                                 │
│  • Prose:       1,800 tokens (73%)                         │
│  • Code:          500 tokens (20%)                         │
│  • Frontmatter:   150 tokens (7%)                          │
│                                                             │
│  ─────────────────────────────────────                     │
│                                                             │
│  AI Performance Estimate:                                  │
│  ● Current: ~92% effectiveness                             │
│  ● At 5K: ~80% effectiveness                               │
│  ● Remaining: 1,550 tokens until warning                   │
│                                                             │
│              [Close]                                       │
└─────────────────────────────────────────────────────────────┘
```

#### Validation Dialog (on click)

**Pass State:**

```
┌─ Validation ────────────────────────────────────────────────┐
│                                                             │
│           ✓                                                 │
│      All checks passed!                                     │
│                                                             │
│  This spec meets all LeanSpec quality standards.           │
│                                                             │
│  Last checked: 2 minutes ago                               │
│                                                             │
│              [Close]                                       │
└─────────────────────────────────────────────────────────────┘
```

**Warning/Error State:**

```
┌─ Validation Issues ─────────────────────────────────────────┐
│                                                             │
│           ⚠                                                 │
│      3 Warnings Found                                       │
│                                                             │
│  ┌────────────────────────────────────────────────────┐    │
│  │ ⚠ Missing Test section                             │    │
│  │   Recommended: Add a Test section with criteria    │    │
│  │                                                    │    │
│  │ [Quick Fix] [View in Spec →]                      │    │
│  └────────────────────────────────────────────────────┘    │
│                                                             │
│  ┌────────────────────────────────────────────────────┐    │
│  │ ⚠ Title exceeds 80 characters                      │    │
│  │   Current: 85 chars                                │    │
│  └────────────────────────────────────────────────────┘    │
│                                                             │
│  ┌────────────────────────────────────────────────────┐    │
│  │ ⚠ Incomplete checklist                             │    │
│  │   2 of 5 items checked                             │    │
│  └────────────────────────────────────────────────────┘    │
│                                                             │
│              [Close]                                       │
└─────────────────────────────────────────────────────────────┘
```

#### Stats Page Enhancement

**New Card (row 2, next to top tags):**

```
┌─ Token Distribution ─────────┐
│                               │
│  [Bar Chart]                 │
│                               │
│  ● Optimal: 12 specs         │
│  ● Good: 8 specs             │
│  ● Warning: 3 specs          │
│  ● Critical: 1 spec          │
│                               │
│  Total: 73,802 tokens        │
│  Average: 2,450/spec         │
└───────────────────────────────┘
```

### Interaction Patterns

**Click Behaviors (Primary):**

- **Token badge/button:** Opens Token Details Dialog with full breakdown
- **Validation button:** Opens Validation Dialog with issues or success state
- **Sort button (in header):** Toggle sort specs by token count asc/desc
- **Filter button (in header):** Toggle filter to show only specs with validation issues

**Hover Behaviors (Secondary):**

```
Token Badge Hover:
┌─────────────────────────────┐
│ Click to view details       │
└─────────────────────────────┘

Validation Button Hover:
┌─────────────────────────────┐
│ Click to view issues        │
└─────────────────────────────┘
```

**Dialog Interactions:**

- **Token Dialog:**
  - Progress bar shows position relative to thresholds
  - Breakdown items show percentage on hover
  - Close button or click outside to dismiss
  - Escape key closes dialog
  
- **Validation Dialog:**
  - Error items expandable (if multiple details)
  - "Quick Fix" button for auto-fixable issues
  - "View in Spec" navigates to relevant section
  - Close button or click outside to dismiss

**Loading States:**

```tsx
// Token badge loading (disabled state)
<Button variant="ghost" disabled className="h-5 px-2">
  <Loader2 className="h-3.5 w-3.5 animate-spin" />
  <span className="ml-1">...</span>
</Button>

// Validation badge loading (disabled state)
<Button variant="ghost" disabled className="h-5 px-2">
  <Loader2 className="h-3.5 w-3.5 animate-spin" />
</Button>
```

### Responsive Behavior

**Mobile (< 640px):**

- Token button shows icon only (📄) in list view
- Validation button shows icon only (✓/⚠/✗)
- Dialog is full-screen (h-full w-full)
- Breakdown shown as vertical list (no mini bars)
- Progress bar simplified (no threshold markers)

**Tablet (640px - 1024px):**

- Token button shows "2.4k" format
- Validation button shows icon + count badge
- Dialog: w-[90vw] max-w-lg
- Full breakdown with mini progress bars

**Desktop (> 1024px):**

- Token button shows full format "📄 2.4k tokens"
- Validation button shows icon + "Pass"/"3 Warnings"/"2 Errors"
- Dialog: w-[500px] centered
- Full visual breakdown with progress bars
- Performance estimates visible

### Threshold Indicators

**Badge Label Mapping:**

```typescript
const thresholdLabels = {
  optimal: { label: 'Optimal', color: 'green', icon: CheckCircle2 },
  good: { label: 'Good', color: 'blue', icon: CheckCircle2 },
  warning: { label: 'Warning', color: 'orange', icon: AlertTriangle },
  critical: { label: 'Critical', color: 'red', icon: AlertCircle }
};
```

**Performance Impact Estimates:**

```
Optimal (<2K):    ~100% baseline AI performance
Good (2-3.5K):    ~90-95% performance
Warning (3.5-5K): ~80-85% performance  
Critical (>5K):   ~65-80% performance
```

### Technical Implementation

**Data Sources:**

- Reuse existing `harnspec tokens` CLI via API (tiktoken, <50ms/spec)
- Integrate with `harnspec validate` command via API
- Calculate on-demand initially, no caching needed for v1

**API Additions:**

```typescript
// types/api.ts
interface Spec {
  tokenCount?: number;
  tokenStatus?: 'optimal' | 'good' | 'warning' | 'critical';
  tokenBreakdown?: {
    prose: number;
    code: number;
    frontmatter: number;
  };
  validationStatus?: 'pass' | 'warn' | 'fail';
  validationErrors?: ValidationError[];
}

interface ValidationError {
  message: string;
  line?: number;
  type: 'frontmatter' | 'content' | 'structure';
  severity: 'error' | 'warning';
}

// backend-adapter.ts
getSpecTokens(projectId: string, specName: string): Promise<TokenInfo>;
getSpecValidation(projectId: string, specName: string): Promise<ValidationResult>;
```

## Plan

### Phase 1: Backend API (1 day)

- [ ] Add `tokenCount`, `tokenStatus`, `validationStatus` fields to Spec type
- [ ] Create `GET /api/projects/{id}/specs/{name}/tokens` endpoint
- [ ] Create `GET /api/projects/{id}/specs/{name}/validation` endpoint
- [ ] Add methods to `backend-adapter.ts` interface

### Phase 2: Button & Dialog Components (2 days)

- [ ] Create `TokenButton` component (clickable button variant)
- [ ] Create `ValidationButton` component (clickable button variant)
- [ ] Create `TokenDetailsDialog` component (full breakdown)
- [ ] Create `ValidationDialog` component (issues or success state)
- [ ] Create `TokenProgressBar` component (for dialog)
- [ ] Add Storybook stories for all components
- [ ] Implement responsive variants (sm/md sizes for buttons)

### Phase 3: Page Integration (1-2 days)

- [ ] Add `TokenButton` to ListView (clickable, opens dialog)
- [ ] Add `ValidationButton` to ListView (clickable, opens dialog)
- [ ] Add sort button to ListView header (sort by tokens asc/desc)
- [ ] Add filter button to ListView toolbar (filter by validation status)
- [ ] Add `TokenButton` to SpecDetailPage metadata row
- [ ] Add `ValidationButton` to SpecDetailPage metadata row
- [ ] Integrate dialogs with page state management

### Phase 4: Stats Dashboard (1 day)

- [ ] Add Token Distribution chart to StatsPage
- [ ] Add aggregate token metrics (total, average)
- [ ] Add validation health overview section

### Phase 5: Polish (1 day)

- [ ] Add hover tooltips with detailed info
- [ ] Implement loading states (skeletons + spinners)
- [ ] Add error handling for failed token/validation fetches
- [ ] Add smooth transitions for progress bars
- [ ] Dark mode testing

## Test

### Visual Tests (Storybook)

- [ ] TokenBadge renders all 4 threshold colors correctly
- [ ] TokenBadge shows correct format (2.4k vs 456)
- [ ] TokenBadge responsive sizes (sm vs md)
- [ ] ValidationStatus icons match design system (CheckCircle2, AlertTriangle, XCircle)
- [ ] ValidationStatus count badge positioned correctly
- [ ] TokenProgressBar shows correct fill color and width
- [ ] Loading states render correctly (spinners, skeletons)
- [ ] Dark mode colors render correctly

### Component Unit Tests

- [ ] TokenBadge formats numbers correctly (>999 = X.Xk)
- [ ] TokenBadge applies correct threshold class
- [ ] ValidationStatus shows correct icon for each status
- [ ] ValidationStatus error count badge displays when >0
- [ ] TokenProgressBar calculates width percentage correctly

### Integration Tests

- [ ] Token button displays in ListView next to status badge
- [ ] Clicking token button opens TokenDetailsDialog
- [ ] TokenDetailsDialog shows correct breakdown data
- [ ] Validation button opens ValidationDialog on click
- [ ] ValidationDialog shows correct error details
- [ ] Sort button in header sorts specs by token count
- [ ] Filter button filters specs by validation status
- [ ] SpecDetailPage shows token and validation buttons in metadata row
- [ ] Dialogs close correctly (close button, outside click, escape key)

### E2E Tests

- [ ] Browse specs and see token buttons with correct colors
- [ ] Click token button opens dialog with full breakdown
- [ ] Token dialog shows progress bar and performance estimates
- [ ] View spec detail and click validation button
- [ ] Validation dialog shows error list with "View in Spec" links
- [ ] Pass state shows success message in validation dialog
- [ ] Dashboard shows Token Distribution chart
- [ ] Sort and filter buttons work correctly
- [ ] Responsive layout works on mobile/tablet/desktop
- [ ] Dialogs are accessible (keyboard navigation, screen readers)

### Accessibility Tests

- [ ] Token badges have aria-label with exact count
- [ ] Validation icons have aria-label with status
- [ ] Color is not the only indicator (icons + text)
- [ ] Tooltips are keyboard accessible

### Performance Tests

- [ ] Token calculation doesn't block page render (<50ms)
- [ ] Token badges load without layout shift
- [ ] Validation status updates don't cause re-render cascade
- [ ] Progress bar animation is smooth (60fps)

## Notes

### Dependencies

- Depends on: 035-live-specs-showcase (UI foundation)
- Depends on: 018-spec-validation (validation logic)
- Depends on: 069-token-counting-utils (token counting)

### Implementation Decisions

- **On-demand calculation**: Use tiktoken directly (<50ms/spec), no caching for v1
- **Validation refresh**: Check on page load + manual refresh button in dialog
- **Token breakdown**: Show in dialog only (clean UI, detailed on demand)
- **Dialogs**: Click badges to open detailed dialogs (no sidebar cards)
- **Sorting**: Separate sort button in table header (not on token badge)
- **Filtering**: Separate filter button in toolbar (not on validation badge)
- **Dialogs**: Use shadcn/ui Dialog component, consistent with existing dialogs (Timeline, Dependencies)

### Visual References

See existing components for styling patterns:

- `StatusBadge.tsx`: Badge sizing, colors, dark mode
- `PriorityBadge.tsx`: Icon + label layout  
- `ListView.tsx`: Spec list layout and spacing
- `SpecDetailPage.tsx`: Header metadata row styling
- `SpecTimeline` dialog: Dialog structure and sizing pattern
- `SpecDependencyGraph` dialog: Full-size dialog pattern

### Design Decisions

- Color coding: Green (optimal: <2k), Yellow (good: 2-3.5k), Orange (warning: 3.5-5k), Red (critical: >5k)
- Validation icons: ✅ Pass, ⚠️ Warnings, ❌ Fail
- Token display format: "2.1k tokens" with progress bar
