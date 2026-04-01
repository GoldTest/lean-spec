---
status: planned
created: 2026-02-03
priority: medium
parent: 297-multi-tasking-ui-architecture
created_at: 2026-02-03T14:16:11.630159Z
updated_at: 2026-02-03T14:16:37.326442Z
---

# Multi-Session Dashboard

## Overview

A dedicated page for monitoring all AI coding sessions across projects simultaneously, providing a real-time control center for AI orchestration.

**Problem:**

- Sessions are scoped to individual projects - no unified view
- Users running AI agents on multiple specs/projects must switch contexts to monitor
- No way to see "all running agents" at a glance
- Miss session failures or completions when focused elsewhere
- Cannot prioritize which session needs attention

**Solution:**
New dashboard page showing all sessions across all registered projects with real-time status updates, filtering, and quick controls.

## Design

### Dashboard Layout

```
┌─────────────────────────────────────────────────────────────────┐
│ [Project ▼]  LeanSpec                                           │
├─────────────────────────────────────────────────────────────────┤
│ [📄 Sessions×]                                              [+] │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  AI Sessions Dashboard                    [All Projects ▼] [⟳]  │
│                                                                  │
│  ┌─── RUNNING (3) ──────────────────────────────────────────┐   │
│  │                                                           │   │
│  │ 🟢 Session #a1b2    Claude    harnspec/168    12m 34s   │   │
│  │    ████████████░░░░░░ 60%     127K tokens    [⏸] [⏹]     │   │
│  │                                                           │   │
│  │ 🟢 Session #c3d4    Copilot   my-app/042      3m 21s     │   │
│  │    ████░░░░░░░░░░░░░ 25%      34K tokens     [⏸] [⏹]     │   │
│  │                                                           │   │
│  │ 🟡 Session #e5f6    Claude    client/089      8m 45s     │   │
│  │    Waiting for user input...                  [▶] [⏹]     │   │
│  │                                                           │   │
│  └───────────────────────────────────────────────────────────┘   │
│                                                                  │
│  ┌─── RECENT (5) ───────────────────────────────────────────┐   │
│  │                                                           │   │
│  │ ✅ Session #g7h8    Claude    harnspec/244   15m   ✓    │   │
│  │    Completed 2 minutes ago                    [↻] [🗑]    │   │
│  │                                                           │   │
│  │ ❌ Session #i9j0    Claude    harnspec/287   8m    ✗    │   │
│  │    Failed: Test assertion error               [↻] [📋]    │   │
│  │                                                           │   │
│  └───────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Status Sections

| Section | Sessions Shown | Sort Order |
|---------|----------------|------------|
| Running | `running`, `paused` | Duration (longest first) |
| Pending | `pending`, `queued` | Created time (oldest first) |
| Recent | `completed`, `failed` (last 24h) | Ended time (newest first) |

### Session Card Components

**Running Session Card:**

```
┌─────────────────────────────────────────────────────────────────┐
│ 🟢 Session #a1b2c3d4                                            │
│ Tool: Claude Code  │  Spec: 168-orchestration  │  harnspec    │
├─────────────────────────────────────────────────────────────────┤
│ ████████████████░░░░░░░░░░░░ 60%                                │
│ Duration: 12m 34s  │  Tokens: 127,432 (~$0.85)  │  Iter: 3/10  │
├─────────────────────────────────────────────────────────────────┤
│ Latest: "Running tests for task 2..."                           │
├─────────────────────────────────────────────────────────────────┤
│ [⏸ Pause]  [⏹ Stop]  [📋 View Logs]  [📄 Open Spec]            │
└─────────────────────────────────────────────────────────────────┘
```

**Failed Session Card:**

```
┌─────────────────────────────────────────────────────────────────┐
│ ❌ Session #i9j0k1l2                                 8m │ 2h ago│
│ Tool: Claude  │  Spec: 287-agent-runners  │  harnspec         │
├─────────────────────────────────────────────────────────────────┤
│ Error: Test assertion failed in agent_test.rs:42                │
│ "Expected 'running' but got 'pending'"                          │
├─────────────────────────────────────────────────────────────────┤
│ [↻ Retry]  [📋 Copy Error]  [📄 View Logs]  [🗑 Dismiss]        │
└─────────────────────────────────────────────────────────────────┘
```

### Filtering & Controls

**Filter Bar:**

```
[All Projects ▼] [All Tools ▼] [All Status ▼] [🔍 Search...]  [⟳ Refresh]
```

**Filter Options:**

- **Project**: All / Specific project
- **Tool**: All / Claude / Copilot / Cursor / Codex / etc.
- **Status**: All / Running / Paused / Pending / Completed / Failed

**Auto-Refresh:**

- Default: Poll every 5 seconds
- WebSocket for instant updates when available
- Manual refresh button for immediate update

### Quick Actions

| Action | Icon | Description |
|--------|------|-------------|
| Pause | ⏸ | Pause running session (if supported) |
| Resume | ▶ | Resume paused session |
| Stop | ⏹ | Cancel/terminate session |
| Retry | ↻ | Restart failed session with same config |
| View Logs | 📋 | Open session detail in tab |
| Open Spec | 📄 | Open associated spec in tab |
| Dismiss | 🗑 | Remove from recent list |
| Copy Error | 📋 | Copy error message to clipboard |

### Real-Time Updates

**WebSocket Events:**

```typescript
type SessionEvent = 
  | { type: 'session.started'; session: Session }
  | { type: 'session.progress'; sessionId: string; progress: number }
  | { type: 'session.log'; sessionId: string; message: string }
  | { type: 'session.completed'; sessionId: string; result: 'success' | 'failure' }
  | { type: 'session.stopped'; sessionId: string };
```

**Visual Feedback:**

- Progress bars animate smoothly
- New sessions slide in from top
- Completed sessions fade to "Recent" section
- Failed sessions highlight with red pulse

### Notifications Integration

When dashboard is not focused:

- Desktop notification on session complete/fail
- Badge on Sessions nav item: `🤖 Sessions (3)` → shows running count
- Sound notification option (configurable)

## Implementation

### Component Architecture

```typescript
// src/pages/sessions-dashboard.tsx
interface SessionsDashboardProps {
  initialSessions?: Session[];
}

// src/components/sessions/SessionCard.tsx
interface SessionCardProps {
  session: Session;
  onPause: () => void;
  onStop: () => void;
  onRetry: () => void;
  onViewLogs: () => void;
  onOpenSpec: () => void;
}

// src/components/sessions/SessionSection.tsx
interface SessionSectionProps {
  title: string;
  sessions: Session[];
  emptyMessage: string;
}

// src/components/sessions/DashboardFilters.tsx
interface DashboardFiltersProps {
  filters: DashboardFilters;
  onChange: (filters: DashboardFilters) => void;
  projects: Project[];
  tools: string[];
}
```

### Data Fetching

```typescript
// src/hooks/useAllSessions.ts
function useAllSessions(filters: DashboardFilters) {
  const { projects } = useProjects();
  
  // Aggregate sessions from all projects
  const allSessions = useQuery({
    queryKey: ['sessions', 'all', filters],
    queryFn: async () => {
      const sessions = await Promise.all(
        projects.map(p => fetchSessions(p.id, filters))
      );
      return sessions.flat();
    },
    refetchInterval: 5000, // Poll every 5s
  });
  
  return allSessions;
}
```

### WebSocket Integration

```typescript
// src/hooks/useSessionUpdates.ts
function useSessionUpdates(onUpdate: (event: SessionEvent) => void) {
  useEffect(() => {
    const ws = new WebSocket(getSessionsWSUrl());
    
    ws.onmessage = (event) => {
      const sessionEvent = JSON.parse(event.data);
      onUpdate(sessionEvent);
    };
    
    return () => ws.close();
  }, []);
}
```

### Route Integration

```typescript
// Add to router
{
  path: '/sessions-dashboard',
  element: <SessionsDashboard />,
}

// Or make accessible from existing /sessions route with toggle
```

## Plan

### Phase 1: Core Dashboard (Week 1)

- [ ] Create SessionsDashboard page component
- [ ] Implement SessionCard for running sessions
- [ ] Implement SessionCard for completed/failed sessions
- [ ] Add section grouping (Running, Recent)
- [ ] Basic project filter dropdown

### Phase 2: Multi-Project Integration (Week 1-2)

- [ ] Aggregate sessions from all registered projects
- [ ] Add project indicator to session cards
- [ ] Implement cross-project session fetching
- [ ] Add "Open Spec" action (switches project context)

### Phase 3: Real-Time Updates (Week 2)

- [ ] Implement polling for session status
- [ ] Add WebSocket support (when backend ready)
- [ ] Animate progress bar updates
- [ ] Handle session state transitions visually

### Phase 4: Quick Actions (Week 2-3)

- [ ] Implement Pause/Resume/Stop controls
- [ ] Add Retry for failed sessions
- [ ] Implement "View Logs" (open in tab)
- [ ] Add error copying and dismissal

### Phase 5: Polish (Week 3)

- [ ] Filter bar (project, tool, status)
- [ ] Search functionality
- [ ] Loading and empty states
- [ ] Desktop notifications integration
- [ ] Badge on nav item for running count

## Test

- [ ] Dashboard loads with sessions from all projects
- [ ] Running sessions show progress correctly
- [ ] Progress bars update in real-time
- [ ] Pause/Stop controls work
- [ ] Failed sessions show error details
- [ ] Retry restarts session correctly
- [ ] Filters narrow results
- [ ] "View Logs" opens session detail in tab
- [ ] "Open Spec" navigates and switches project
- [ ] Empty state shows when no sessions
- [ ] Auto-refresh updates every 5 seconds

## Notes

### Design Decisions

**Why a separate dashboard vs enhanced /sessions?**

- /sessions is project-scoped (fits current model)
- Dashboard is cross-project (different purpose)
- Can coexist - nav to dashboard or project sessions

**Why group by status vs time?**

- Running sessions need immediate visibility
- Status grouping aids prioritization
- Time-based within groups for context

**Why limit "Recent" to 24h?**

- Prevents infinite list growth
- Older sessions less actionable
- Can always go to project-level /sessions for history

### Dependencies

- 244-session-ui-enhancement (single session detail page)
- 239-ai-coding-session-management (backend APIs)
- 168-leanspec-orchestration-platform (session infrastructure)

### Future Enhancements

- Session comparison (compare logs of two sessions)
- Session templates (save successful session config)
- Bulk actions (stop all, pause all)
- Session scheduling (queue sessions for later)
- Cost tracking dashboard (token usage over time)
- Team view (sessions from all team members)

### Related Specs

- 297-multi-tasking-ui-architecture (parent umbrella)
- 298-browser-style-tabs (open session details in tabs)
- 244-session-ui-enhancement (individual session pages)
- 168-leanspec-orchestration-platform (orchestration context)
