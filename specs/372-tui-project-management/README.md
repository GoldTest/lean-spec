---
status: planned
created: '2026-03-20'
tags:
  - cli
  - tui
  - ux
  - rust
  - ratatui
priority: high
created_at: '2026-03-20T10:30:00.000000+00:00'
---

# TUI Project Management

> **Status**: planned · **Priority**: high · **Created**: 2026-03-20

## Overview

The TUI currently operates on a single specs directory passed as an argument. It has no awareness of the project registry (`~/.lean-spec/projects.json`) that the web UI uses to manage multiple projects. The web UI provides a project switcher dropdown, a full project management page with CRUD operations, favorites, color coding, and GitHub import. The TUI has none of this — users must exit and relaunch with a different path to view another project's specs.

**Goal**: Bring project management to the TUI by integrating with the existing `ProjectRegistry` from `leanspec-core`, enabling multi-project switching and basic project management without leaving the terminal.

## Problems

1. **No multi-project support** — TUI is locked to one specs directory per session; switching requires relaunching
2. **No project awareness** — TUI doesn't read from the project registry, so users who manage multiple projects via the web UI get a disconnected experience in the terminal
3. **No project management** — can't add, rename, favorite, or remove projects from the TUI

## Design

### 1. Project Switcher

Add a project switcher accessible via `p` key from any view:

```
┌─ Switch Project ──────────────────────┐
│ Search: _                             │
│                                       │
│ ★ lean-spec          ~/projects/lean  │
│   acme-backend       ~/projects/acme  │
│   mobile-app         ~/projects/mob…  │
│   ◐ cloud-infra (gh) codervisor/inf…  │
│                                       │
│ [a]dd  [m]anage  [Enter] switch       │
└───────────────────────────────────────┘
```

**Behavior:**
- Opens as a centered overlay (popup) above the current view
- Lists all projects from `ProjectRegistry`, sorted: favorites first, then by `last_accessed` descending
- Each row shows: favorite star (`★`), project name, truncated path (or `owner/repo` for GitHub projects)
- Color indicator: left border or name colored with project's `color` field
- GitHub projects show `◐` icon prefix
- `j`/`k` or arrow keys to navigate, `Enter` to switch, `Esc` to cancel
- `/` activates inline search to filter projects by name
- Switching updates `last_accessed` in the registry and reloads specs into the TUI
- Status bar shows current project name after switching

**Integration with `ProjectRegistry`:**
- On TUI launch: if no `--specs-dir` argument, auto-load the most recently accessed project from the registry
- If `--specs-dir` is provided, use it directly (backward compatible)
- New `--project <name-or-id>` flag to launch directly into a named project

### 2. Project Management View

Add a dedicated project management view accessible via `P` (shift-p) or from the switcher via `m`:

```
┌─ Projects ──────────────────────────────────────┐
│                                                  │
│ ★ lean-spec            42 specs  96% complete    │
│   ~/projects/codervisor/lean-spec                │
│   Last accessed: 2 hours ago                     │
│                                                  │
│   acme-backend         18 specs  72% complete    │
│   ~/projects/acme-backend                        │
│   Last accessed: yesterday                       │
│                                                  │
│   mobile-app           31 specs  45% complete    │
│   ~/projects/mobile-app                          │
│   Last accessed: 3 days ago                      │
│                                                  │
│ [a]dd [r]ename [c]olor [f]avorite [d]elete [Esc] │
└──────────────────────────────────────────────────┘
```

**Project card details:**
- Project name with favorite star and color indicator
- Path (full, not truncated)
- Spec count and completion rate (percentage of specs with status `complete`)
- Last accessed relative timestamp
- Validation status icon: `✓` valid, `✗` invalid path, `?` unchecked

**Actions (on selected project):**
- `a` — **Add project**: prompts for directory path, auto-detects specs dir (checks `specs/`, `.lean-spec/specs/`, `doc/specs/`, `docs/specs/`)
- `r` — **Rename**: inline edit of project name
- `c` — **Change color**: cycle through preset colors or enter hex code
- `f` — **Toggle favorite**: star/unstar the project
- `d` — **Delete**: confirmation prompt, removes from registry (does NOT delete files)
- `v` — **Validate**: checks if project path still exists, updates status
- `Enter` — **Open**: switches to this project's specs view

**Add project flow:**
```
┌─ Add Project ─────────────────────────┐
│ Path: ~/projects/new-project_         │
│                                       │
│ Detected: specs/ (12 specs found)     │
│                                       │
│ [Enter] add  [Esc] cancel             │
└───────────────────────────────────────┘
```
- Text input for path with basic tab-completion (list directories)
- Auto-detects specs directory and shows preview of spec count
- Validates path exists before adding

### 3. Status Bar Integration

Update the TUI status bar to show the current project:

```
 lean-spec │ List │ 42 specs │ #369 │ ? help
```

- Project name appears at the left of the status bar
- Clicking the project name (mouse) opens the project switcher
- Color-coded to match the project's assigned color

### 4. Startup Behavior

Enhance TUI launch to be project-aware:

| Scenario | Behavior |
|----------|----------|
| `lean-spec tui` (no args) | Load most recently accessed project from registry |
| `lean-spec tui --specs-dir ./specs` | Use specified directory (existing behavior) |
| `lean-spec tui --project acme` | Load named project from registry |
| No projects in registry | Show "Add Project" prompt on first launch |
| Registry project path invalid | Show warning, offer to remove or re-point |

## Plan

- [ ] Import `ProjectRegistry` from `leanspec-core` into TUI module
- [ ] Add `--project` CLI flag to `tui` subcommand
- [ ] Update TUI startup to auto-load from registry when no `--specs-dir` given
- [ ] Implement project switcher popup widget (`tui/project_switcher.rs`)
- [ ] Add `p` keybinding to open project switcher from any view
- [ ] Implement project reload — swap `specs_dir`, reload specs, reset selection state
- [ ] Implement project management view (`tui/projects.rs`) with card layout
- [ ] Add project actions: add, rename, color, favorite, delete, validate
- [ ] Add path input widget with basic directory tab-completion
- [ ] Add auto-detect specs directory logic (reuse from `leanspec-core`)
- [ ] Show current project name in status bar with color
- [ ] Update `last_accessed` timestamp on project switch
- [ ] Handle edge cases: empty registry, invalid paths, first-launch flow

## Non-Goals

- GitHub import from TUI (complex OAuth/token flow — use web UI or CLI `import` command)
- Project creation wizard (just path input; use web UI for guided setup)
- Syncing project preferences (color, favorite) across machines (handled by cloud sync layer)
- Editing project `specs_dir` after creation (delete and re-add instead)

## Test

- [ ] `lean-spec tui` with no args loads the most recently accessed project
- [ ] `lean-spec tui --project acme` loads the named project
- [ ] `p` key opens project switcher overlay with all registered projects
- [ ] Switching projects reloads specs and updates the sidebar
- [ ] Favorites sort to the top of the project list
- [ ] Search in project switcher filters by project name
- [ ] `P` key opens project management view
- [ ] Add project with valid path succeeds and auto-detects specs directory
- [ ] Add project with invalid path shows error
- [ ] Rename project updates the registry
- [ ] Delete project shows confirmation and removes from registry (files untouched)
- [ ] Toggle favorite updates star indicator and re-sorts list
- [ ] Status bar shows current project name
- [ ] `last_accessed` updates when switching projects
- [ ] Empty registry shows first-launch add-project prompt
- [ ] Backward compatible: `--specs-dir` still works without registry
