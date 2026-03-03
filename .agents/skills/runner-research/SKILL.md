---
name: runner-research
description: Research AI agent runners/platforms (Claude Code, Copilot, Cursor, Windsurf, Codex, Kiro, Gemini, Amp, Goose, etc.) to track config format changes, new capabilities, deprecations, and emerging runners. Use when asked to "research runners", "check for runner updates", "audit runner compatibility", or when preparing to update the runner registry. Triggers include questions about runner config formats, new runner releases, or keeping LeanSpec runner support current.
---

# Runner Research

Research AI agent runners to keep LeanSpec's runner registry and configurations current as the ecosystem evolves rapidly.

## Workflow

### 1. Gather Current State

Read the runner registry source to understand what LeanSpec currently supports:

```
rust/leanspec-core/src/sessions/runner.rs  # RunnerRegistry::builtins()
```

Read the current runner catalog reference for known details:

- See [references/runners-catalog.md](references/runners-catalog.md) for the full catalog of tracked runners

### 2. Research Updates

For each runner category, use `web_search` to check for:

- **Config format changes** — New or renamed config files/dirs (e.g., `.cursorrules` → `.cursor/rules`)
- **CLI changes** — New commands, renamed binaries, new flags
- **New environment variables** — API key changes, new auth mechanisms
- **New capabilities** — MCP support, tool use, context protocol changes
- **Deprecations** — Removed features, sunset announcements
- **New runners** — Emerging agent platforms not yet tracked

Prioritize runners by adoption and rate of change:

1. **Tier 1** (high priority): Claude Code, Copilot, Cursor, Windsurf, Codex, Gemini
2. **Tier 2** (medium): Kiro, Amp, Aider, Goose, Continue, Roo Code
3. **Tier 3** (monitor): Droid, Kimi, Qodo, Trae, Qwen Code, OpenHands, Crush, CodeBuddy, Kilo, Augment

### 3. Compare & Identify Gaps

Cross-reference findings against the registry:

- Missing detection commands or config dirs
- Outdated env var names
- New runners worth adding
- Runners that have been discontinued

### 4. Report Findings

Produce output based on significance:

- **Minor updates** (typos, env var additions): Update `references/runners-catalog.md` directly
- **Major changes** (new config formats, new runners, breaking changes): Create a new spec via the leanspec-sdd skill describing what needs to change in the runner registry

### 5. Update the Catalog

After research, update `references/runners-catalog.md` with:

- New findings, dates checked, version numbers
- Any corrections to detection config
- New runners discovered

## Key Source Files

| File | Purpose |
|------|---------|
| `rust/leanspec-core/src/sessions/runner.rs` | Runner registry with detection config |
| `schemas/runners.json` | JSON schema for custom runner config |
| `packages/cli/templates/_shared/agents-components/` | AGENTS.md template components |
| `packages/cli/templates/*/AGENTS.md` | Generated AGENTS.md templates |

## Research Sources

Use `web_search` with queries like:

- `"{runner-name} CLI changelog 2025 2026"`
- `"{runner-name} agent config file format"`
- `"{runner-name} MCP support"`
- `"new AI coding agent CLI 2026"`
- `"site:github.com {runner-org}/{runner-repo} releases"`
