---
status: planned
created: 2026-03-07
priority: high
tags:
- memory
- ai-agents
- architecture
- mcp
- core
created_at: 2026-03-07T02:39:24.545079Z
updated_at: 2026-03-07T02:39:24.545079Z
---

# Project-Scoped Memory for AI Agents

## Overview

AI agents lose project-specific knowledge between sessions. LeanSpec should provide a **project-scoped memory layer** that persists learnings, decisions, patterns, and facts across agent sessions — complementing specs (intent) with operational memory (experience).

Inspired by OpenClaw's memory architecture (Markdown files + semantic search + hybrid retrieval), but adapted for LeanSpec's project-centric, multi-agent model where memory belongs to the **project**, not individual agents.

## Problem

1. **Knowledge evaporation**: Agents rediscover the same patterns, conventions, and pitfalls every session
2. **No shared memory**: When multiple agents (Copilot, Claude, Cursor) work on the same project, each starts cold
3. **Specs ≠ memory**: Specs capture *intent* (what to build), but not *experience* (how things work, what failed, conventions discovered)
4. **Runner-specific silos**: GitHub Copilot's `/memories/repo/` and Claude's `CLAUDE.md` are runner-scoped — knowledge doesn't transfer between agents

## Design

### Memory Scopes

| Scope | Purpose | Storage | Injected Into |
|-------|---------|---------|---------------|
| **Project** | Conventions, architecture facts, verified commands | `.lean-spec/memory/` | All agent sessions in project |
| **Session** | Task-specific working context, in-progress notes | Ephemeral / session-local | Current session only |

**Key distinction from OpenClaw**: Memory is project-scoped (shared across agents), not agent-scoped (siloed per agent identity).

### File Layout

```
.lean-spec/
  memory/
    index.md            # Curated project knowledge (auto-loaded)
    conventions.md      # Coding patterns, naming, style
    architecture.md     # System design facts, component relationships
    operations.md       # Verified build/test/deploy commands
    decisions.md        # Key decisions and rationale
    daily/
      YYYY-MM-DD.md     # Daily append-only logs (auto-pruned)
```

### Memory Types

1. **Curated memory** (`index.md`, topic files): Durable, reviewed facts. Injected into agent context on every turn. Token-budgeted.
2. **Daily logs** (`daily/*.md`): Append-only session notes. Searchable but not auto-injected. Auto-pruned after configurable retention period.

### Retrieval Mechanisms

- **Auto-injection**: `index.md` contents included in system prompt context (token-budgeted, configurable cap)
- **Search**: `lean-spec memory search "query"` — keyword + fuzzy match over all memory files
- **Read**: `lean-spec memory read <file>` — targeted file access
- **MCP tools**: `memory_search`, `memory_read`, `memory_write` exposed via MCP server

### Write Mechanisms

- **MCP tool**: `memory_write` — agents write memories during/after sessions
- **CLI**: `lean-spec memory add "fact" --file conventions.md`
- **Auto-capture**: Optional hook that prompts agent to flush durable notes before session ends (inspired by OpenClaw's memory flush before compaction)

### Configuration

```json
// .lean-spec/config.json
{
  "memory": {
    "enabled": true,
    "autoInjectTokenBudget": 2000,
    "dailyRetentionDays": 30,
    "autoFlush": true
  }
}
```

### Integration with Runner Instruction Files

LeanSpec memory complements, not replaces, runner-specific files:

| Source | Scope | LeanSpec Role |
|--------|-------|---------------|
| `CLAUDE.md` | Claude only | LeanSpec can sync curated memory → CLAUDE.md |
| `/memories/repo/` | Copilot only | LeanSpec can export memory as Copilot repo memories |
| `AGENTS.md` | All runners | Static instructions (not memory) |
| `.lean-spec/memory/` | All runners | Dynamic, evolving project knowledge |

### Relationship to Spec 159

Spec 159 defines LeanSpec as the "memory layer" for agent orchestration. This spec implements the **concrete memory storage and retrieval system** that enables that vision. Specs remain for *intent*; memory is for *experience*.

## Non-Goals

- Agent-scoped memory (per-agent identity isolation) — use runner-native features
- Embedding/vector search — start with keyword/fuzzy, add semantic later if needed
- Cloud sync of memory — handled by existing cloud sync infrastructure
- Replacing AGENTS.md or runner instruction files

## Requirements

- [ ] Define memory file format (Markdown with optional YAML frontmatter for metadata)
- [ ] Implement `memory/` directory initialization in `lean-spec init`
- [ ] Add `memory_search` MCP tool for semantic/keyword search over memory files
- [ ] Add `memory_read` MCP tool for targeted file access
- [ ] Add `memory_write` MCP tool for agents to persist learnings
- [ ] Add CLI commands: `lean-spec memory add|search|read|list`
- [ ] Auto-inject `index.md` into MCP context with configurable token budget
- [ ] Support daily log files with auto-pruning
- [ ] Add `memory` section to `.lean-spec/config.json` schema
- [ ] Document memory workflow in AGENTS.md template

## Acceptance Criteria

- Agent can write a fact via MCP and retrieve it in a new session
- `index.md` contents appear in agent context without explicit tool call
- Multiple runners (Copilot, Claude) can read the same project memory
- Memory search returns relevant results across all memory files
- Daily logs auto-prune after retention period
- Token budget for auto-injection is respected

## Notes

### OpenClaw Architecture Takeaways

| OpenClaw Feature | LeanSpec Adaptation |
|------------------|---------------------|
| `MEMORY.md` (curated long-term) | `index.md` — auto-injected curated knowledge |
| `memory/YYYY-MM-DD.md` (daily logs) | `daily/YYYY-MM-DD.md` — same pattern |
| Per-agent SQLite vector index | Deferred — start with keyword/fuzzy search |
| Hybrid search (BM25 + vectors) | Start with full-text, add vectors in v2 |
| Memory flush before compaction | Auto-flush hook before session end |
| Agent-scoped workspace isolation | Project-scoped sharing across agents |

### Key Differentiator
OpenClaw's memory is **agent-scoped** (each agent has its own workspace and memory). LeanSpec's memory is **project-scoped** (all agents working on the project share the same memory). This reflects LeanSpec's position as a project-level tool, not an agent runtime.