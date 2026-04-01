# Design: Unified Analytics & Dashboard

## Architecture

### File Structure

```
src/commands/
  ├── dashboard.ts       # NEW: Comprehensive overview
  ├── stats.ts          # ENHANCED: Merge timeline logic
  ├── list.ts           # KEEP: Detailed browsing
  ├── board.ts          # KEEP: Kanban view
  ├── gantt.ts          # KEEP: Timeline planning
  ├── deps.ts           # KEEP: Dependencies
  └── timeline.ts       # DEPRECATE → merge into stats.ts
```

### Code Reuse

- Extract shared visualization helpers to `utils/vis.ts`:
  - `createBar(count, max, width, char)` - reusable bar charts
  - `formatMetric(label, value, color)` - consistent metric display
  - `renderSection(title, content)` - section formatting
- Dashboard composes high-level summaries from other commands
- Stats command owns all analytics logic (including timeline)

### Performance

- Dashboard: Single `loadAllSpecs()` for all sections
- Smart caching between dashboard and other commands
- Lazy rendering (skip empty sections)
- Target: < 300ms for 100 specs

## Part 0: Timestamp Tracking (Foundation)

### Problem

Current tracking only stores dates (YYYY-MM-DD), not timestamps:

- Can't calculate precise cycle times
- Can't distinguish specs completed same day
- Loses granularity for velocity analysis

### Solution

Add ISO 8601 timestamp fields alongside date fields

### Frontmatter Schema Update

```yaml
---
status: in-progress
created: '2025-11-04'           # Keep for human readability
created_at: '2025-11-04T14:30:00Z'  # NEW: Precise timestamp
updated: '2025-11-04'           # Keep for human readability  
updated_at: '2025-11-04T16:45:00Z'  # NEW: Precise timestamp
completed_at: '2025-11-05T10:15:00Z'  # NEW: When status changed to complete

# Status transition history (optional, for advanced velocity)
transitions:
  - status: planned
    at: '2025-11-04T14:30:00Z'
  - status: in-progress
    at: '2025-11-04T15:00:00Z'
  - status: complete
    at: '2025-11-05T10:15:00Z'
---
```

### Migration Strategy

- Add new `*_at` fields alongside existing date fields
- Auto-generate timestamps on spec creation/updates
- Existing specs: infer timestamps from dates (use midnight UTC)
- Keep date fields for backward compatibility
- Make timestamps optional (graceful degradation)

### Implementation

```typescript
// In frontmatter.ts
export interface SpecFrontmatter {
  // Existing date fields (keep for compatibility)
  created: string;        // YYYY-MM-DD
  updated?: string;       // YYYY-MM-DD
  completed?: string;     // YYYY-MM-DD
  
  // NEW: Precise timestamps
  created_at?: string;    // ISO 8601
  updated_at?: string;    // ISO 8601
  completed_at?: string;  // ISO 8601
  
  // NEW: Status transition history (optional)
  transitions?: Array<{
    status: SpecStatus;
    at: string;  // ISO 8601
  }>;
}

// Auto-generate timestamps on updates
export function enrichWithTimestamps(
  data: SpecFrontmatter, 
  previousData?: SpecFrontmatter
): void {
  const now = new Date().toISOString();
  
  // Set created_at if missing
  if (!data.created_at) {
    data.created_at = data.created 
      ? `${data.created}T00:00:00Z`  // Infer from date
      : now;
  }
  
  // Update updated_at on any change
  if (previousData) {
    data.updated_at = now;
  }
  
  // Set completed_at when status changes to complete
  if (data.status === 'complete' && 
      previousData?.status !== 'complete' &&
      !data.completed_at) {
    data.completed_at = now;
    data.completed = new Date().toISOString().split('T')[0];
  }
  
  // Track transition (optional)
  if (previousData && data.status !== previousData.status) {
    if (!data.transitions) data.transitions = [];
    data.transitions.push({
      status: data.status,
      at: now
    });
  }
}
```

## Part 1: Unified Analytics Command

### Command Structure

Keep `harnspec stats` (backward compatible), enhanced with velocity modes:

```bash
harnspec stats              # Default: current stats (unchanged)
harnspec stats --timeline   # Add timeline section
harnspec stats --history    # Full historical view (current timeline command)
harnspec stats --velocity   # NEW: Cycle time & throughput analysis
harnspec stats --all        # Everything (stats + timeline + velocity)
```

### Velocity Section Output

```
📊 Velocity Metrics (Last 30 Days)

Cycle Time (Created → Completed)
  Average:  5.2 days  ████████░░░░░░░░░░  (target: 7 days)
  Median:   4.0 days
  P50-P95:  2-12 days

Stage Duration
  Planned:       2.1 days  ████░░░░░░░░░░░░░░░░
  In-Progress:   3.5 days  ██████░░░░░░░░░░░░░░
  
Throughput
  Last 7 days:   3 specs  ████████████░░░░░░░░  (up from 2)
  Last 30 days: 12 specs  ████████████████████  (target: 10)
  
Work in Progress
  Current WIP:   5 specs  (recommended: < 5)
  Average WIP:   4.2 specs

Velocity Trend
  Week 1:  ████░░  2 specs
  Week 2:  ████░░  2 specs  
  Week 3:  ██████  3 specs  ↑ improving
  Week 4:  ██████  3 specs
```

### Implementation Details

- Calculate cycle time: `completed_at - created_at`
- Track stage durations from transitions array
- Show percentiles (P50, P90, P95) for cycle time distribution
- Compare to targets (configurable in .harnspec/config.json)
- Show trends (last 4 weeks)

## Part 2: Dashboard Command (NOT IMPLEMENTED)

> **Decision**: Dashboard command was not implemented. Enhanced `stats` and `board` commands provide the needed functionality without adding CLI complexity.

### Original Design (for reference)

### Command

`harnspec` (no args) or `harnspec dashboard`

### Purpose

Quick project health overview combining:

- Summary metrics (from stats)
- Key activity indicators (from timeline)  
- Active work snapshot (from board)
- Smart insights (what needs attention)

### Dashboard Sections

```
╔══════════════════════════════════════════════════════════╗
║  LeanSpec Dashboard · harnspec                          ║
╚══════════════════════════════════════════════════════════╝

📊 Project Health
  Total: 42 specs · 15 in-progress · 20 complete · 5 planned · 2 archived
  Priority: 🔴 2 critical · 🟠 5 high · 🟡 10 medium · 🟢 3 low

⚠️  Needs Attention
  • 2 specs overdue (spec-001, spec-003)
  • 3 critical priority specs still planned

📈 Recent Activity (Last 14 Days)
  Created:   ████████░░░░░░  8 specs
  Completed: ████░░░░░░░░░░  4 specs
  Velocity:  2.8 specs/week ↑ trending up
  
⏳ In Progress (5)
  🔴 spec-042-mcp-error-handling     @alice  #bug #critical  (3d)
  🟠 spec-045-unified-dashboard      @bob    #ux #launch     (2d)
  🟡 spec-026-init-pattern           @alice  #feature       (8d ⚠️ long)
  
🏷️  Top Tags
  launch (12) · feature (8) · bug (5) · docs (4)

🚀 Velocity Summary
  Avg Cycle Time:  5.2 days (target: 7d)
  Throughput:      2.8 specs/week ↑
  WIP:             5 specs (healthy)

─────────────────────────────────────────────────────────────
💡 Commands: harnspec list | harnspec board | harnspec stats --velocity
```

### Smart Insights

- Show overdue specs first
- Highlight critical priority items
- Show specs assigned to user (if `--assignee` or git config)
- Suggest next actions

### Display Options

```bash
harnspec                      # Full dashboard
harnspec dashboard            # Explicit
harnspec --compact            # Minimal (just health + attention)
harnspec --expand-active      # Show all in-progress (not just top 5)
harnspec --json               # JSON for tooling
```

### vs. Individual Commands

| Command | Purpose | When to Use |
|---------|---------|-------------|
| `harnspec` | Quick overview | Daily standup, "what's happening?" |
| `harnspec list` | Browse/search specs | Find specific spec, apply filters |
| `harnspec board` | Kanban workflow | Sprint planning, status changes |
| `harnspec gantt` | Timeline planning | Schedule work, see deadlines |
| `harnspec stats` | Deep analytics | Metrics review, team performance |

## Command Organization Philosophy

### PM Commands (keep separate - distinct workflows)

- `list` - Browse/search/filter specs
- `board` - Kanban workflow (status changes)
- `deps` - Dependency visualization
- `gantt` - Timeline planning (schedule work)

### Analytics Commands (consolidate - overlapping purpose)

- `stats` - Current metrics + historical trends
- `timeline` - Redundant with stats (merge in)

### Dashboard (new - quick overview)

- `harnspec` - Glanceable project health
- Entry point for daily use
- Directs to PM commands for detail

This organization makes sense because:

- PM commands have distinct UX patterns (kanban, graph, gantt chart)
- Analytics commands both show "numbers over time"
- Dashboard is a meta-view (doesn't replace PM commands)

## Why Merge Stats + Timeline?

### Current Redundancy

- Both load all specs
- Both show date-based trends
- Both have bar charts
- Both support same filters
- Both output similar visualizations

### Differences

- `stats` - emphasizes current state (status, priority, tags)
- `timeline` - emphasizes historical change (created/completed over time)

### Solution

Make `stats` the comprehensive analytics command:

- Default: current stats (backward compatible)
- `--timeline`: add timeline section
- `--history`: timeline-focused view

## Smart Insights Algorithm

"Needs Attention" section prioritizes:

1. **Overdue & Critical** - highest urgency

   ```typescript
   spec.frontmatter.due < today &&
   spec.frontmatter.status != 'complete' &&
   spec.frontmatter.priority == 'critical'
   ```

2. **Overdue & In-Progress** - likely blockers

   ```typescript
   spec.frontmatter.due < today &&
   spec.frontmatter.status == 'in-progress'
   ```

3. **Critical & Planned** - not started yet

   ```typescript
   spec.frontmatter.priority == 'critical' &&
   spec.frontmatter.status == 'planned'
   ```

4. **Long-running In-Progress** - potential stalls

   ```typescript
   spec.frontmatter.status == 'in-progress' &&
   daysSince(spec.frontmatter.updated) > 14
   ```

Show top 3-5 items max, then "and N more need attention"

## Why `harnspec` Should Default to Dashboard

### Current behavior

`harnspec` shows help

### Proposed

`harnspec` shows dashboard

### Reasoning

- Help still accessible via `harnspec --help`
- Dashboard is most frequently needed view
- Matches modern CLI patterns (gh, git status at root)
- Better new user experience (show, don't tell)
- OpenSpec uses `openspec view` as primary command

### User flow

```bash
cd my-project
harnspec                    # Quick overview (dashboard)
# See something interesting...
harnspec list --tag bug     # Drill down
harnspec board              # Change status
harnspec                    # Check dashboard again
```

## Velocity Configuration

Add to `.harnspec/config.json`:

```json
{
  "velocity": {
    "enabled": true,
    "targets": {
      "cycle_time_days": 7,
      "throughput_per_month": 10,
      "max_wip": 5
    },
    "alerts": {
      "long_running_days": 14,
      "overdue_critical": true
    },
    "tracking": {
      "use_timestamps": true,
      "track_transitions": true
    }
  }
}
```

### Defaults (if not configured)

- Cycle time target: 7 days
- Max WIP: 5 concurrent specs
- Long-running threshold: 14 days
- Timestamps: auto-enabled
- Transitions: optional (off by default)
