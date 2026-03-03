---
status: planned
created: 2026-03-03
priority: critical
tags:
- ai
- chat
- tools
- codebase
- workspace
depends_on:
- 322-tool-call-ux-redesign
parent: 333-agentic-ai-chat-tool-ecosystem
created_at: 2026-03-03T08:00:39.472119Z
updated_at: 2026-03-03T08:00:39.472119Z
---

# AI Chat Codebase Tools: Read, Write, and Search Project Files

## Overview

LeanSpec's AI chat has 10 spec-management tools but **zero codebase tools**. The AI cannot read a source file, search for a symbol, or make an edit — it must dispatch everything to a runner session. This forces users into heavyweight runner sessions for simple tasks (fix a typo, update an import, add a config line) that Copilot/Claude/Cursor handle inline.

**Goal**: Add a core set of codebase tools to the AI chat tool registry so it can perform direct read/write operations on project files. Runner sessions remain the right choice for complex, multi-file, long-horizon tasks — but simple, targeted changes should happen within the chat conversation itself.

## Requirements

### Read Tools

- [ ] `read_file` — Read file contents with optional line range (`startLine`/`endLine`), project-scoped, path traversal protection
- [ ] `list_directory` — List directory entries (files, dirs, sizes, gitignore status), same as existing HTTP endpoint
- [ ] `search_files` — Find files by name/path glob pattern (maps to existing file search endpoint)
- [ ] `grep` — Text/regex search across project files with match context (line numbers, surrounding lines). New HTTP endpoint required

### Write Tools

- [ ] `write_file` — Create a new file or overwrite an existing file. Requires explicit `path` and `content`
- [ ] `edit_file` — Apply targeted string replacements on an existing file (similar to spec `update` replacements pattern: `oldString`/`newString` with match mode). Supports multiple replacements in one call
- [ ] `delete_file` — Delete a file. Requires confirmation parameter (`confirm: true`) to prevent accidental deletion

### Infrastructure

- [ ] Register all new tools in `leanspec-core/src/ai_native/tools/registry.rs` using the existing `make_tool<I>` + `schemars` pattern
- [ ] Add input structs for each tool in `inputs.rs` following existing `camelCase` serde conventions
- [ ] Add `grep` HTTP endpoint to `leanspec-http` (the other read tools already have HTTP backing)
- [ ] Add `write_file`, `edit_file`, `delete_file` HTTP endpoints to `leanspec-http`
- [ ] All file operations scoped to project root with `resolve_safe_path` traversal protection
- [ ] Update system prompt to describe expanded capabilities and when to use tools vs runners
- [ ] Add tool result UI components in `tool-result-registry.tsx` for new tool types (file content viewer, search results list, edit diff summary)

## Non-Goals

- **Full terminal/shell execution** — No arbitrary command execution in chat. That's a separate concern with significant security implications
- **LSP integration** — No go-to-definition, rename-symbol, or type checking. Use runners that have IDE capabilities for that
- **Git operations** — No commit, branch, push. Users do that themselves or via runner sessions
- **Large file operations** — Keep 1MB read limit. Binary files rejected. For large refactors, use runners
- **Replacing runners** — Runners are *better* for multi-step, multi-file, long-running tasks with full IDE context. These tools handle quick, targeted actions

## Design

### Tool Routing Strategy: Chat Tools vs Runners

```
User Request → AI decides routing:

  Simple/targeted:                    Complex/long-horizon:
  ┌─────────────────────┐            ┌─────────────────────┐
  │ "Fix the typo in    │            │ "Implement spec 042  │
  │  config.ts line 15" │            │  end-to-end"         │
  │                     │            │                      │
  │  → read_file        │            │  → run_subagent /    │
  │  → edit_file        │            │    run_session       │
  │  → done in chat     │            │    (full runner)     │
  └─────────────────────┘            └─────────────────────┘
```

System prompt guidance for the AI:
- **Use chat tools** for: reading files, understanding code, searching patterns, fixing small bugs, updating configs, adding/editing individual sections
- **Use runners** for: implementing full specs, multi-file refactors, running tests, tasks requiring build/compile context, anything needing >5 file edits

### Security

All write operations follow the same security model as existing spec tools:
- Path traversal protection via `resolve_safe_path` (canonicalize + prefix check)
- Operations scoped to project root directory
- No execution of file contents
- `delete_file` requires explicit `confirm: true` parameter
- File size limits maintained (1MB read, reasonable write limits)

### HTTP API Endpoints (New)

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/api/projects/{id}/grep` | Text/regex search across files |
| `POST` | `/api/projects/{id}/files/write` | Create or overwrite a file |
| `PATCH` | `/api/projects/{id}/files/edit` | Apply string replacements |
| `DELETE` | `/api/projects/{id}/file` | Delete a file |

### Tool Input Schemas

```rust
// read_file — leverages existing GET /api/projects/{id}/file
pub struct ReadFileInput {
    pub project_id: Option<String>,
    pub path: String,          // relative to project root
    pub start_line: Option<u32>,
    pub end_line: Option<u32>,
}

// grep — new endpoint
pub struct GrepInput {
    pub project_id: Option<String>,
    pub pattern: String,       // search pattern
    pub is_regex: Option<bool>,
    pub include_pattern: Option<String>, // glob filter (e.g. "*.rs")
    pub max_results: Option<u32>,
    pub context_lines: Option<u32>, // lines of context around matches
}

// write_file
pub struct WriteFileInput {
    pub project_id: Option<String>,
    pub path: String,
    pub content: String,
    pub create_directories: Option<bool>, // mkdir -p behavior
}

// edit_file — surgical replacements
pub struct EditFileInput {
    pub project_id: Option<String>,
    pub path: String,
    pub replacements: Vec<EditReplacement>,
    pub expected_content_hash: Option<String>, // optimistic concurrency
}
pub struct EditReplacement {
    pub old_string: String,
    pub new_string: String,
    pub match_mode: Option<String>, // unique|all|first
}

// delete_file
pub struct DeleteFileInput {
    pub project_id: Option<String>,
    pub path: String,
    pub confirm: bool, // must be true
}
```

### Leveraging Existing Infrastructure

Several pieces already exist and can be reused:

| Need | Existing Asset |
|------|---------------|
| File path security | `resolve_safe_path` in `leanspec-http/src/handlers/files.rs` |
| File reading | `read_project_file` handler (add line range support) |
| File listing | `list_project_files` handler |
| File name search | `search_project_files` handler |
| String replacements | `apply_replacements` in `leanspec-core` (used by spec `update`) |
| Optimistic concurrency | `hash_content` + `expectedContentHash` pattern from spec updates |
| Tool registration | `make_tool<I>` pattern in `registry.rs` |
| Tool result UI | `tool-result-registry.tsx` patterns from spec 322 |
| gitignore walking | `ignore` crate already in use for file browsing |

## Plan

### Phase 1: Read Tools (Connects to existing HTTP endpoints)
- [ ] Add `ReadFileInput`, `ListDirectoryInput`, `SearchFilesInput` structs to `inputs.rs`
- [ ] Register `read_file`, `list_directory`, `search_files` in `registry.rs` — each calls existing HTTP endpoints via `fetch_json`
- [ ] Add line-range support to `read_project_file` HTTP handler (query params `startLine`, `endLine`)
- [ ] Add tool result renderers in UI for file content and search results
- [ ] Update AI chat tool count test (`test_build_tools_produces_11_tools` → new count)

### Phase 2: Grep Tool (New HTTP endpoint)
- [ ] Add `POST /api/projects/{id}/grep` endpoint using the `ignore` crate walker + regex matching
- [ ] Add `GrepInput` struct and register `grep` tool in `registry.rs`
- [ ] Add grep results renderer in UI (file path + matched lines with context)

### Phase 3: Write Tools (New HTTP endpoints + tools)
- [ ] Add `POST /api/projects/{id}/files/write` endpoint
- [ ] Add `PATCH /api/projects/{id}/files/edit` endpoint reusing `apply_replacements` from `leanspec-core`
- [ ] Add `DELETE /api/projects/{id}/file` endpoint with `confirm` guard
- [ ] Add `WriteFileInput`, `EditFileInput`, `DeleteFileInput` structs
- [ ] Register `write_file`, `edit_file`, `delete_file` tools in `registry.rs`
- [ ] Add result renderers (diff display for edits, creation confirmation for writes)

### Phase 4: System Prompt and Polish
- [ ] Update system prompt to describe all new tools and routing guidance
- [ ] Add tools to MCP server (`leanspec-mcp`) for external AI tool consumers
- [ ] Update docs with new tool descriptions

## Acceptance Criteria

- [ ] AI chat can read any project file and reference its content in conversation
- [ ] AI chat can search for code patterns with grep and find relevant files
- [ ] AI chat can create new files and edit existing files without spawning a runner
- [ ] All file operations reject paths outside project root
- [ ] Edit operations use `oldString`/`newString` pattern with optimistic concurrency
- [ ] Existing spec management tools continue to work unchanged
- [ ] Tool results render properly in the chat UI

## Notes

- The `edit_file` tool reuses the proven `Replacement` / `apply_replacements` infrastructure from spec content editing — same `oldString`/`newString`/`matchMode` pattern users and AI already know
- Grep implementation can use the `ignore` crate (already a dependency) for gitignore-respecting file walking + the `regex` crate for pattern matching
- Write operations should return the new file's content hash so subsequent edits can use optimistic concurrency
- Consider adding a `max_write_size` config option (default ~500KB) to prevent the AI from writing enormous files