---
status: archived
created: '2025-11-03'
tags:
  - vscode
  - copilot
  - ai
  - ux
  - v0.2.0-optional
priority: medium
created_at: '2025-11-03T00:00:00Z'
updated_at: '2026-01-16T07:23:04.394Z'
depends_on:
  - 067-monorepo-core-extraction
  - 017-vscode-extension
  - 072-ai-agent-first-use-workflow
transitions:
  - status: archived
    at: '2026-01-16T07:23:04.394Z'
---

# GitHub Copilot Chat Slash Commands & Prompts

> **Status**: 📦 Archived · **Priority**: Medium · **Created**: 2025-11-03 · **Tags**: vscode, copilot, ai, ux, v0.2.0-optional

**Project**: harnspec  
**Team**: Core Development

## Overview

Add LeanSpec integration directly into VS Code via GitHub Copilot Chat using slash commands and chat participants. This enables developers to interact with specs naturally through Copilot chat without leaving their coding context.

**Why now?** Copilot Chat is where developers already ask questions and get guidance. Integrating LeanSpec here makes specs discoverable and actionable at the exact moment they're needed—when writing code.

## Design

**Implementation Options:**

**Option A: Chat Participant** (Recommended)

- Register `@harnspec` chat participant via VS Code extension
- Natural language interactions: `@harnspec what's the status of auth system?`
- Context-aware: can reference open files, workspace, current work

**Option B: Slash Commands**

- Register commands like `/harnspec-search`, `/harnspec-create`, `/harnspec-status`
- More structured but less conversational
- Fallback if participant API has limitations

**Key Features:**

- `/harnspec search <query>` or `@harnspec search for authentication specs`
- `/harnspec status` or `@harnspec show me what's in progress`
- `/harnspec create` or `@harnspec help me create a spec for this feature`
- `/harnspec read <spec>` or `@harnspec show me the API redesign spec`
- `/harnspec update <spec> --status <status>` or `@harnspec mark auth-system as complete`

**Context Integration:**

- Automatically detect related specs based on open files
- Suggest creating specs when Copilot detects new feature work
- Link spec references in code comments to full specs

**Architecture:**

- Extend existing VS Code extension (spec 006)
- Use VS Code Chat Participant API or Slash Command API
- Reuse LeanSpec CLI logic via TypeScript imports
- Return formatted markdown with links to specs

## Plan

- [ ] Research VS Code Chat Participant API vs Slash Command API
- [ ] Choose implementation approach (likely Chat Participant)
- [ ] Extend VS Code extension package
- [ ] Implement spec search with Copilot Chat
- [ ] Add spec status queries and visualization
- [ ] Enable spec creation through conversational flow
- [ ] Add spec reading/preview in chat
- [ ] Implement status updates
- [ ] Add context-aware spec suggestions
- [ ] Test with real development workflows
- [ ] Document in extension README and docs

## Test

- [ ] Chat participant/commands register successfully
- [ ] Search returns relevant results with correct formatting
- [ ] Status queries show accurate current state
- [ ] Create flow guides user through spec creation
- [ ] Read command displays spec content clearly
- [ ] Update commands modify specs correctly
- [ ] Context awareness suggests relevant specs
- [ ] Works alongside other Copilot features
- [ ] Handles workspaces without LeanSpec gracefully

## Notes

**GitHub Copilot Chat APIs:**

- Chat Participants: <https://code.visualstudio.com/api/extension-guides/chat>
- Language Model API for context

**Design Principles:**

- Conversational over command-line syntax
- Smart defaults based on workspace context
- Clear feedback and error messages
- Don't interrupt flow—augment it

**Related:**

- VS Code extension (spec 006) - base extension to extend
- MCP server (spec 019) - similar capabilities, different context (Claude Desktop vs VS Code)

**Open Questions:**

- Should this replace or complement the VS Code extension sidebar?
- Can we auto-generate spec suggestions from PR descriptions?
