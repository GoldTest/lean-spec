---
status: planned
created: 2025-12-05
priority: low
tags:
- analytics
- velocity
- ux
- mcp
- cli
- v0.3.0
created_at: 2025-12-05T07:44:57.547Z
updated_at: 2026-02-02T08:14:22.688657002Z
---
# AI-Era Analytics & Velocity Redesign

> **Status**: 📅 Planned · **Priority**: High · **Created**: 2025-12-05

## Problem Statement

Current stats/analytics have critical issues for AI-accelerated development:

1. **Oversimplified metrics** - Counts by status/priority don't inform decisions
2. **Wrong time granularity** - Days/weeks meaningless when AI completes specs in hours/minutes
3. **Missing agile classics** - No cumulative flow, cycle time distributions, or predictability metrics
4. **No actionable insights** - Stats show "what" but not "so what" or "now what"

### Before: Current Stats Output

```
📊 Spec Stats
  Total Specs     45
  Planned         12
  In Progress      8
  Complete        25
  
🚀 Velocity Summary
  Avg Cycle Time   3.2 days   # Meaningless for AI coding
  Throughput       5/week     # Too coarse
  WIP              8 specs
```

### After: AI-Era Analytics

```
⚡ AI Velocity Dashboard
  
📈 Spec Flow (Last 7 Days)
  ┌────────────────────────────────┐
  │ █████████████████████  Created │ 12
  │ ████████████████       Active  │ 8
  │ ███████████████████    Done    │ 15
  └────────────────────────────────┘
  
⏱️  Cycle Time Distribution
  < 1 hour   ████████████ 8 specs (32%)  ← AI-assisted
  1-4 hours  ██████ 4 specs (16%)
  4-24 hours ████ 3 specs (12%)
  1-3 days   ██████████ 7 specs (28%)    ← Complex/blocked
  > 3 days   ███ 3 specs (12%)           ⚠️ Investigate
  
  Median: 2.4 hours | P90: 18 hours
  
🔄 Flow Efficiency
  Active Time:  4.2 hours (avg)
  Wait Time:    12.8 hours (avg)
  Efficiency:   25% ← Bottleneck in review/planning
```

## Design

### 1. Time Granularity Shift

**Current**: Days as smallest unit
**New**: Minutes/hours for sub-day, days for multi-day

```typescript
interface TimeMetrics {
  // Smart formatting based on duration
  format(ms: number): string; // "23m", "2.4h", "3.2d"
  
  // Granularity-aware calculations
  cycleTime: {
    minutes: number;      // Raw value
    formatted: string;    // "2h 34m" or "1.5 days"
    bucket: TimeBucket;   // 'sub-hour' | 'hours' | 'day' | 'multi-day'
  };
}

type TimeBucket = 
  | 'instant'    // < 15 minutes (trivial/test specs)
  | 'quick'      // 15m - 1h (ideal AI-assisted)
  | 'session'    // 1h - 4h (focused work session)
  | 'half-day'   // 4h - 12h (complex feature)
  | 'day'        // 12h - 24h
  | 'multi-day'  // 1-3 days (investigation needed)
  | 'stuck';     // > 3 days (definitely blocked)
```

### 2. Agile Flow Metrics

#### Cumulative Flow Diagram (CFD) Data

```typescript
interface CumulativeFlowData {
  // Time series data for CFD visualization
  series: {
    date: string;
    planned: number;
    inProgress: number;
    complete: number;
    archived: number;
  }[];
  
  // Derived metrics
  averageLeadTime: number;
  averageWIP: number;
  throughputTrend: TrendDirection;
  
  // Flow health indicators
  wipLimit: number | null;          // Configured limit
  wipViolations: number;            // Days over limit
  flowEfficiency: number;           // Active/total time %
}
```

#### Lead Time vs Cycle Time

```typescript
interface FlowTimes {
  // Lead Time: Idea → Done (includes wait time)
  leadTime: {
    total: Duration;
    breakdown: {
      planning: Duration;    // Created → In-Progress
      execution: Duration;   // In-Progress (active work)
      waiting: Duration;     // Blocked, in review, etc.
    };
  };
  
  // Cycle Time: Work Started → Done
  cycleTime: {
    total: Duration;
    activeTime: Duration;    // Actual work time
    efficiency: number;      // activeTime / total
  };
}
```

### 3. AI-Specific Insights

```typescript
interface AIVelocityInsights {
  // Pattern detection
  patterns: {
    aiAssistedRatio: number;      // % completed < 4 hours
    humanOnlyRatio: number;       // % completed > 1 day
    avgSpeedupFactor: number;     // vs. pre-AI baseline
  };
  
  // Bottleneck analysis
  bottlenecks: {
    stage: 'planning' | 'coding' | 'review' | 'testing';
    avgDelay: Duration;
    count: number;
    suggestion: string;
  }[];
  
  // Predictability
  predictability: {
    estimateAccuracy: number;     // If estimates tracked
    cycleTimeVariance: number;    // Lower = more predictable
    confidenceLevel: number;      // For completion forecasts
  };
}
```

### 4. New MCP Tools

```typescript
// Enhanced stats tool with structured output
interface StatsToolOutput {
  summary: ProjectSummary;
  flow: CumulativeFlowData;
  velocity: VelocityMetrics;
  insights: AIVelocityInsights;
  recommendations: ActionableRecommendation[];
}

// New analytics tool for specific queries
interface AnalyticsQuery {
  metric: 'cycle-time' | 'lead-time' | 'throughput' | 'wip' | 'flow';
  period: 'day' | 'week' | 'month' | 'quarter';
  groupBy?: 'tag' | 'priority' | 'assignee';
  compare?: 'previous-period' | 'baseline';
}
```

### 5. CLI Output Redesign

#### Default View: Executive Summary

```
harnspec stats
```

```
⚡ LeanSpec Analytics

📊 Flow Health: Good ✓
   WIP: 5 specs (limit: 8)
   Throughput: 12 specs/week ↑23%
   
⏱️  Speed Metrics
   Median Cycle: 2.4 hours
   AI-Assisted:  68% (< 4h completion)
   
⚠️  Attention (2)
   • 3 specs stuck > 3 days
   • High WIP on tag:api (6 specs)

💡 Run `harnspec stats --flow` for cumulative flow
   Run `harnspec stats --distribution` for cycle time breakdown
```

#### Flow View

```
harnspec stats --flow
```

```
📈 Cumulative Flow (30 days)

Created    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  87
In Progress ━━━━━━━━━━━━━━━━━━━━━━━━━━            5
Complete   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━   72
Archived   ━━━━━━━━━                             10

Flow Efficiency: 34%
├─ Active Work:   4.2h avg
├─ Wait/Blocked:  8.1h avg
└─ Planning:      1.2h avg

Bottleneck: review stage (+2.3h vs target)
```

#### Distribution View

```
harnspec stats --distribution
```

```
⏱️  Cycle Time Distribution (completed specs)

  < 30m    ████████████████ 16  (21%)  Trivial
  30m-1h   ██████████████ 14  (18%)  Quick wins
  1-4h     ████████████████████ 20  (26%)  AI-sweet-spot
  4-8h     ██████████ 10  (13%)  Session work
  8-24h    ██████ 6   (8%)   Day features
  1-3d     ████████ 8   (10%)  Complex
  > 3d     ███ 3    (4%)   ⚠️ Investigate

  Median: 1.8h | Mean: 6.2h | P90: 22h | P99: 4.2d
  
  Trend: Improving ↓ (median was 3.2h last month)
```

### 6. Structured Data for AI Agents

```typescript
// MCP tool returns structured, LLM-friendly output
{
  "summary": {
    "health": "good",
    "healthScore": 82,
    "activeSpecs": 5,
    "completedThisWeek": 12,
    "trend": "improving"
  },
  "recommendations": [
    {
      "priority": "high",
      "issue": "3 specs have been in-progress > 3 days",
      "specs": ["045-unified-dashboard", "052-api-redesign", "067-auth-flow"],
      "suggestion": "Review for blockers or scope creep",
      "action": "run `harnspec view 045` to investigate"
    }
  ],
  "metrics": {
    "cycleTime": {
      "median": { "value": 108, "unit": "minutes" },
      "p90": { "value": 22, "unit": "hours" },
      "distribution": { ... }
    }
  }
}
```

### 7. Web UI Redesign

See **[UI-DESIGN.md](./UI-DESIGN.md)** for detailed UI specifications including:

- Dashboard overview with health score, velocity, WIP, throughput cards
- Cumulative Flow Chart (replacing pie charts)
- Cycle Time Histogram with AI-era time buckets
- Flow Efficiency breakdown
- Sparklines for trend visualization
- Insights Panel with actionable recommendations
- Period comparison view
- New routes: `/stats/flow`, `/stats/velocity`, `/stats/compare`

## Plan

### Phase 1: Core Infrastructure

- [ ] Add minute-level timestamp precision to frontmatter
- [ ] Create `Duration` type with smart formatting
- [ ] Add time bucket classification utility
- [ ] Update velocity.ts for sub-day granularity

### Phase 2: Flow Metrics

- [ ] Implement cumulative flow data collection
- [ ] Add lead time breakdown (planning/execution/waiting)
- [ ] Calculate flow efficiency metrics
- [ ] Track WIP over time for trend analysis

### Phase 3: Distribution Analytics

- [ ] Cycle time histogram/distribution
- [ ] Percentile calculations (P50, P75, P90, P95, P99)
- [ ] Time bucket analysis
- [ ] Comparative analysis (this period vs last)

### Phase 4: AI Insights

- [ ] Pattern detection (AI-assisted ratio)
- [ ] Bottleneck identification algorithm
- [ ] Predictability scoring
- [ ] Actionable recommendations engine

### Phase 5: CLI Redesign

- [ ] New default executive summary view
- [ ] `--flow` flag for cumulative flow
- [ ] `--distribution` flag for cycle time histogram
- [ ] `--compare` flag for period comparison

### Phase 6: MCP Enhancement

- [ ] Structured analytics tool output
- [ ] Query-based analytics API
- [ ] Recommendations in tool response
- [ ] Export formats (JSON, CSV)

### Phase 7-10: UI Implementation

See [UI-DESIGN.md](./UI-DESIGN.md) for detailed UI phases.

## Test

- [ ] Verify minute-level timestamps work correctly
- [ ] Test cycle time calculations with sub-hour durations
- [ ] Validate flow efficiency calculations
- [ ] Test recommendation engine with edge cases
- [ ] Verify CLI output formatting for various data sizes
- [ ] Test MCP tool structured output schema

See [UI-DESIGN.md](./UI-DESIGN.md) for UI-specific test criteria.

## Appendix: Metrics Reference

### AI-Era Velocity Benchmarks

| Metric | Poor | Fair | Good | Excellent |
|--------|------|------|------|-----------|
| Median Cycle Time | > 3d | 1-3d | 4-24h | < 4h |
| AI-Assisted Ratio | < 20% | 20-40% | 40-60% | > 60% |
| Flow Efficiency | < 20% | 20-35% | 35-50% | > 50% |
| Throughput Trend | ↓↓ | ↓ | → | ↑ |
| WIP Stability | > 150% limit | 100-150% | 80-100% | < 80% |

### Traditional Agile Metrics (Still Relevant)

| Metric | Description | Why It Matters |
|--------|-------------|----------------|
| Cumulative Flow | Stacked area chart of work states | Visual bottleneck detection |
| Little's Law | Lead Time = WIP / Throughput | Predictability foundation |
| Cycle Time Distribution | Histogram of completion times | Understand variability |
| Flow Efficiency | Active Time / Total Time | Reduce waste |
| Escaped Defects | Issues found post-complete | Quality indicator |

## Notes

### Open Questions

1. **Timestamp precision**: Store ISO with timezone, or Unix epoch ms?
2. **Baseline comparison**: Should we track pre-AI baseline for speedup metrics?
3. **Team vs solo**: How to handle multi-assignee specs in velocity?
4. **Historical data**: Backfill minute-level timestamps from git history?

### Rejected Alternatives

- **Story points**: Not meaningful for AI-assisted work (complexity ≠ time)
- **Burndown charts**: Sprint-based thinking doesn't fit continuous flow
- **Velocity in points/sprint**: AI makes this metric meaningless

### Future Considerations

- Integration with git commit frequency for active work detection
- PR cycle time integration
- AI agent session correlation (which agent, how many iterations)
