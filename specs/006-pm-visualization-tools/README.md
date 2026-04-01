---
status: archived
created: '2025-11-01'
tags:
  - enhancement
  - cli
  - visualization
  - pm-tools
priority: medium
depends_on:
  - 20251101/002-structured-frontmatter
completed: '2025-11-01'
---

# pm-visualization-tools

> **Status**: ✅ Complete · **Priority**: Medium · **Created**: 2025-11-01 · **Tags**: enhancement, cli, visualization, pm-tools

## Overview

Once specs have structured frontmatter (status, tags, priority, dependencies), we can build powerful PM/project visibility tools. These CLI commands transform LeanSpec from a simple spec tool into a lightweight project management system - all while staying file-first and lean.

**Goal**: Give teams visibility into their work without requiring heavy PM tools like Jira or Linear. Everything stays in git, searchable and version-controlled.

## Design

### Commands Overview

```bash
harnspec stats              # Quick summary stats
harnspec board              # Kanban view by status
harnspec timeline           # Creation/completion over time
harnspec gantt              # Timeline with dependencies
harnspec deps <spec>        # Dependency graph
harnspec search <query>     # Full-text + metadata search
```

### 1. `harnspec stats`

Show aggregate statistics across all specs.

**Output:**

```
📊 Spec Statistics

Status:
  📅 Planned:      8
  🚧 In Progress:  5
  ✅ Complete:    12
  📦 Archived:    23

Priority:
  🔴 High:         3
  🟡 Medium:       7
  🟢 Low:          3

Tags (top 5):
  feature:        11
  api:             6
  refactor:        4
  breaking-change: 3
  security:        2

Total Specs: 48
```

**Options:**

- `--tag=api` - Filter stats by tag
- `--assignee=alice` - Filter by assignee
- `--json` - Output as JSON for scripting

### 2. `harnspec board`

Kanban-style board view grouped by status.

**Output:**

```
📋 Spec Board

┌─ 📅 Planned (8) ───────────────────────────────────────┐
│ 20251101/003-pm-visualization-tools                     │
│   tags: [enhancement, cli]  priority: medium            │
│                                                          │
│ 20251031/002-template-system-redesign                   │
│   tags: [refactor]  priority: low                       │
└──────────────────────────────────────────────────────────┘

┌─ 🚧 In Progress (5) ───────────────────────────────────┐
│ 20251101/002-structured-frontmatter                     │
│   tags: [enhancement]  priority: high  assignee: alice  │
│                                                          │
│ 20251031/001-typescript-cli-migration                   │
│   tags: [refactor]  priority: medium                    │
└──────────────────────────────────────────────────────────┘

┌─ ✅ Complete (12) ──────────────────────────────────────┐
│ (collapsed, use --show-complete to expand)              │
└──────────────────────────────────────────────────────────┘
```

**Options:**

- `--show-complete` - Expand complete column
- `--tag=api` - Filter by tag
- `--assignee=alice` - Filter by assignee

### 3. `harnspec timeline`

Visualize spec creation and completion over time.

**Output:**

```
📈 Spec Timeline (Last 30 Days)

2025-10-31 ████████ 8 created  ██ 2 completed
2025-11-01 ███ 3 created  █████ 5 completed

Created by Status:
  Oct 2025: 15 specs
  Nov 2025: 3 specs

Completion Rate:
  Last 7 days:  7 specs completed
  Last 30 days: 18 specs completed
```

**Options:**

- `--days=90` - Show last N days
- `--by-tag` - Group by tag
- `--by-assignee` - Group by assignee

### 4. `harnspec gantt`

Timeline view with dependencies (requires `depends_on` and optional `due` dates).

**Output:**

```
📅 Gantt Chart

Nov 1    Nov 8    Nov 15   Nov 22
|--------|--------|--------|--------|

20251101/002-structured-frontmatter
■■■■■■■■□□□□□□□□                   (in-progress, due: Nov 8)

20251101/003-pm-visualization-tools
        ↳ depends on 002
        □□□□□□□□□□□□□□□□           (planned, due: Nov 22)

20251031/001-typescript-cli-migration
■■■■■■■■■■■■■■■■                   (complete)
```

**Options:**

- `--weeks=8` - Show N weeks
- `--show-complete` - Include completed specs
- `--critical-path` - Highlight critical path

### 5. `harnspec deps <spec>`

Show dependency graph for a specific spec.

**Output:**

```
📦 Dependencies for 20251101/003-pm-visualization-tools

Depends On:
  → 20251101/002-structured-frontmatter [in-progress]

Required By:
  (none)

Related:
  ⟷ 20251031/001-typescript-cli-migration [complete]

Dependency Chain:
  003-pm-visualization-tools
    └─ 002-structured-frontmatter
         └─ 001-typescript-cli-migration ✓
```

**Options:**

- `--depth=3` - Show N levels deep
- `--graph` - ASCII graph visualization
- `--json` - Output as JSON

### 6. `harnspec search <query>`

Full-text search with metadata filters.

**Output:**

```bash
$ harnspec search "api" --status=planned --priority=high

🔍 Found 2 specs matching "api"

20251031/002-api-v2-migration
  Status: planned  Priority: high  Tags: [api, breaking-change]
  Match: "...redesign the API to support GraphQL..."

20251101/004-api-authentication
  Status: planned  Priority: high  Tags: [api, security]
  Match: "...implement OAuth2 for API access..."
```

**Options:**

- `--status=<status>` - Filter by status
- `--tag=<tag>` - Filter by tag
- `--priority=<priority>` - Filter by priority
- `--assignee=<user>` - Filter by assignee

### Implementation Notes

**Tech Stack:**

- `gray-matter` - Parse frontmatter
- `chalk` - Colors (already used)
- `ink` + `ink-box` - React-based TUI for interactive board
- `ink-text-input` - Input component for search/filtering
- `ink-select-input` - Selection component for navigation
- `marked` or `markdown-it` - Extract text for search

**Interactive vs Static Mode:**

- Default commands output static text (fast, scriptable)
- Add `--interactive` or `-i` flag for TUI mode
- Example: `harnspec board --interactive` launches Ink TUI

**Performance:**

- Cache parsed frontmatter to avoid re-reading files
- For large repos (100+ specs), consider indexing
- Lazy load spec content for search (frontmatter first)

**Output Format:**

- Default: colorful terminal output
- `--interactive` / `-i`: Interactive TUI with Ink
- `--json` flag for scripting/integration
- `--markdown` flag for documentation

## Plan

- [ ] Implement `harnspec stats` command
- [ ] Implement `harnspec board` command (static output)
- [ ] Implement `harnspec timeline` command
- [ ] Implement `harnspec deps` command with graph visualization
- [ ] Implement `harnspec gantt` command
- [ ] Implement `harnspec search` command with full-text + metadata
- [ ] Add caching layer for better performance
- [ ] Add `--json` and `--markdown` output formats
- [ ] Implement interactive TUI mode with Ink
  - [ ] Interactive board with keyboard navigation
  - [ ] Real-time filtering and search
  - [ ] Spec detail view
  - [ ] Status update inline (move between columns)
- [ ] Update documentation with examples

## Test

- [ ] `harnspec stats` shows correct counts across all specs
- [ ] `harnspec board` groups specs by status correctly
- [ ] `harnspec timeline` aggregates by date accurately
- [ ] `harnspec deps` resolves dependency chains correctly
- [ ] `harnspec gantt` displays timeline with proper dependencies
- [ ] `harnspec search` finds specs by content and metadata
- [ ] Commands work with 100+ specs (performance)
- [ ] `--json` output is valid and parseable
- [ ] Color output respects NO_COLOR environment variable
- [ ] Gracefully handles specs without optional fields
- [ ] Interactive mode (`--interactive`) launches TUI correctly
- [ ] TUI keyboard navigation works (arrows, tab, enter)
- [ ] TUI updates persist to files (status changes write to frontmatter)
- [ ] TUI handles terminal resize gracefully

## Notes

### Why Build This?

Many teams avoid heavy PM tools but still need visibility. By building on structured frontmatter, we can provide:

- Zero-setup project visibility
- Git-based, version-controlled work tracking
- No external dependencies or SaaS subscriptions
- Works offline, fast, no database needed

### Progressive Enhancement

Users can adopt these tools gradually:

1. Start with basic `harnspec list`
2. Add frontmatter → unlock `harnspec stats`
3. Add dependencies → unlock `harnspec deps` and `harnspec gantt`
4. Add due dates → unlock timeline planning

### Inspiration

- GitHub Projects (but file-based)
- Linear (but without the database)
- Taskwarrior (but for specs, not tasks)
- Gitmoji/commitizen (structured data in git)
- K9s (Kubernetes TUI) - great example of Ink power

### Interactive TUI with Ink

Build an interactive board using Ink (React for CLI):

**Command:**

```bash
harnspec board --interactive
# or shorthand
harnspec board -i
```

**TUI Features:**

```
┌─ LeanSpec Board ─────────────────────────────────────────┐
│ Filter: [api___] (type to filter) Press / to focus       │
├──────────────────────────────────────────────────────────┤
│                                                           │
│ ┌─ 📅 Planned (3) ────────────────────────────────────┐ │
│ │ › 003-pm-visualization-tools                         │ │
│ │   tags: [enhancement, cli]  priority: medium         │ │
│ │                                                       │ │
│ │   002-api-redesign                                    │ │
│ │   tags: [api, feature]  priority: high               │ │
│ └───────────────────────────────────────────────────────┘ │
│                                                           │
│ ┌─ 🚧 In Progress (2) ────────────────────────────────┐ │
│ │   002-structured-frontmatter                         │ │
│ │   tags: [enhancement]  priority: high                │ │
│ └───────────────────────────────────────────────────────┘ │
│                                                           │
└──────────────────────────────────────────────────────────┘
│ ↑/↓: Navigate  →/←: Move Status  Enter: Details  q: Quit │
```

**Interactions:**

- `↑/↓` or `j/k` - Navigate between specs
- `→/←` or `l/h` - Move spec to next/prev status column
- `Enter` - Open spec detail view (shows full Overview + Plan)
- `/` - Focus search/filter input
- `t` - Filter by tag (opens tag selector)
- `p` - Filter by priority
- `a` - Filter by assignee
- `r` - Refresh data
- `q` - Quit

**Detail View:**

```
┌─ 20251101/003-pm-visualization-tools ────────────────────┐
│ Status: planned                                           │
│ Created: 2025-11-01                                       │
│ Tags: enhancement, cli, visualization                     │
│ Priority: medium                                          │
│ Depends on: 002-structured-frontmatter                    │
│                                                           │
│ Overview:                                                 │
│ Build powerful PM/project visibility tools...             │
│                                                           │
│ Plan:                                                     │
│ ☐ Implement harnspec stats command                          │
│ ☐ Implement harnspec board command                          │
│ ...                                                       │
└──────────────────────────────────────────────────────────┘
│ s: Change Status  Esc: Back  e: Edit in $EDITOR          │
```

**Status Change:**

- When moving spec with `→/←`, update frontmatter automatically
- Also update `updated` timestamp
- If moving to complete, set `completed` date

**Implementation Structure:**

```typescript
// components/Board.tsx
import React from 'react';
import { Box, Text, useInput } from 'ink';

export const Board = ({ specs, onStatusChange }) => {
  // Kanban columns with keyboard navigation
  // Use ink-select-input for selection
};

// components/SpecDetail.tsx
export const SpecDetail = ({ spec }) => {
  // Full spec view with metadata and content
};

// commands/board-interactive.ts
import { render } from 'ink';
import { Board } from '../components/Board';

export const boardInteractive = async (options) => {
  const specs = await loadSpecs();
  render(<Board specs={specs} />);
};
```

**Benefits:**

- Fast, keyboard-driven workflow
- No context switching (stay in terminal)
- Real-time updates visible immediately
- Git diffs show exactly what changed
- Works over SSH (unlike web UIs)

**Keep It Optional:**

- Static output remains default (fast, scriptable)
- Interactive mode is opt-in (`-i` flag)
- Ink is optional dependency (peer dependency?)
