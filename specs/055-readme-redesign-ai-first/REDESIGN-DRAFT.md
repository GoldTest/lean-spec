# LeanSpec README Redesign - Draft Version

> **Note**: This is a proposed redesign based on deep analysis.  
> **See**: [ANALYSIS-PART1.md](ANALYSIS-PART1.md) & [ANALYSIS-PART2.md](ANALYSIS-PART2.md) for full rationale | [CHANGES.md](CHANGES.md) for improvements summary

---

# LeanSpec

<p align="center">
  <img src="docs-site/static/img/logo-with-bg.svg" alt="LeanSpec Logo" width="120" height="120">
</p>

<p align="center">
  <a href="https://www.npmjs.com/package/harnspec"><img src="https://img.shields.io/npm/v/harnspec.svg" alt="npm version"></a>
  <a href="https://www.npmjs.com/package/harnspec"><img src="https://img.shields.io/npm/dm/harnspec.svg" alt="npm downloads"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
</p>

---

## Your specs are too big for AI to read

Traditional specs overflow AI context windows. Your AI agent can't help because it can't fit the full context.

```diff
- 2,000-line RFC → "Context too large" → AI hallucinates
+ 287-line LeanSpec → Fits perfectly → AI implements correctly
```

**LeanSpec: Spec-Driven Development for human + AI collaboration.**

Specs under 300 lines. Intent-focused. Machine-readable. The SDD methodology designed for the AI era.

<p align="center">
  <a href="#try-it-now-5-minutes"><strong>Try It Now (5 Minutes) →</strong></a> •
  <a href="https://harnspec.gitub.io"><strong>Documentation</strong></a> •
  <a href="https://harnspec.gitub.io/docs/examples"><strong>Examples</strong></a>
</p>

---

## The Problem with Traditional SDD

### Scenario 1: The Context Overflow 🔴

You paste your 2,000-line RFC into Cursor. **"Context too large."** AI can't help. You're back to manual implementation.

### Scenario 2: The Stale Spec 📄

Your team has beautiful specs. None of them match the current code. Nobody updates them because it's too painful. They're dead on arrival.

### Scenario 3: The Process Paralysis ⚖️

You tried heavyweight RFCs—too slow, nobody maintains them. You tried "just code"—AI agents get lost, team misaligns. Where's the middle ground?

**LeanSpec solves this:**

- ✅ Specs fit in AI context windows (<300 lines)
- ✅ Light enough to actually maintain (5 min updates)
- ✅ Structured enough for AI agents to act on
- ✅ Flexible enough to grow with your team

---

## How It Works

### A LeanSpec in Action

Here's a real spec from this project (287 lines):

```yaml
---
status: in-progress
created: 2025-11-01
tags: [cli, dx]
priority: high
---

# Unified Dashboard

## Overview
Combine `harnspec board` and `harnspec stats` into a single, comprehensive
project health view. Give users instant insight into project status,
bottlenecks, and team velocity.

## Design
- Board view (Kanban columns)
- Key metrics (completion rate, avg spec size)
- Bottleneck detection (specs >400 lines, stale specs)
- Health score (0-100)

## Plan
1. Merge board + stats logic
2. Add health scoring algorithm
3. Implement bottleneck detection
4. Add color-coded indicators

## Success Criteria
- Shows full project state in <5 seconds
- Identifies bottlenecks automatically
- Used daily by team leads
```

**Notice:**

- ✅ Under 300 lines (fits in AI context)
- ✅ Intent is clear ("what" and "why")
- ✅ Implementation details are minimal
- ✅ Both human and AI can understand
- ✅ Structured metadata (status, tags, priority)

---

## Built on First Principles

LeanSpec isn't arbitrary rules—it's derived from fundamental constraints of working with AI.

### 🧠 Context Economy

**Specs <300 lines → Fit in working memory**

- Physics: AI context windows are bounded (~20K effective tokens)
- Biology: Human working memory is limited (7±2 items)
- Economics: Large contexts cost more time and money
- **Result**: Keep specs under 300 lines, split complex features into sub-specs

### ✂️ Signal-to-Noise Maximization

**Every word informs decisions → Or it's cut**

- Every sentence must answer: "What decision does this inform?"
- Cut obvious statements, inferable content, "maybe future" speculation
- Keep decision rationale, constraints, success criteria
- **Result**: Dense, actionable specs that respect reader attention

### 📈 Progressive Disclosure

**Add structure only when you feel pain → Start simple**

- Solo dev: Just `status` + `created`
- Small team: Add `tags` + `priority`
- Enterprise: Add custom fields as needed
- **Result**: Structure adapts to team size, not the other way around

### 🎯 Intent Over Implementation

**Capture "why" → Let "how" emerge**

- Must have: Problem, intent, success criteria
- Should have: Design rationale, trade-offs
- Could have: Implementation details (these change)
- **Result**: Specs stay relevant as implementation evolves

### 🌉 Bridge the Gap

**Both humans AND AI must understand → Clear structure + natural language**

- For humans: Overview, context, rationale
- For AI: Unambiguous requirements, structured metadata, examples
- Both can parse and reason about specs
- **Result**: Specs that enable human-AI collaboration

---

**These aren't preferences—they're constraints.** Physics (context windows), biology (working memory), and economics (token costs) dictate what works.

📖 [Deep dive: First Principles Guide →](https://harnspec.gitub.io/docs/guide/first-principles)

---

## Features Designed for AI-First Development

<table>
<tr>
<td width="50%" valign="top">

### 🤖 AI-Native Integration

Works seamlessly with:

- **Cursor** / **Copilot** - Give agents full context
- **Aider** - Structured specs for autonomous coding
- **Claude** via MCP - Direct spec access in conversations

Clear specs = better code generation. Simple.

</td>
<td width="50%" valign="top">

### 📊 Built-in Workflow Tools

Track progress without leaving the terminal:

```bash
$ harnspec board
┌──────────────────────────────────┐
│ 📋 PLANNED │ 🚧 IN PROGRESS │ ✅ DONE │
│ • api-v2   │ • user-auth    │ • cli  │
│ • dashboard│ • db-migration │ • docs │
└──────────────────────────────────┘
```

</td>
</tr>
<tr>
<td width="50%" valign="top">

### 🎨 Flexible Structure

Start minimal, add complexity progressively:

```yaml
# Day 1: Solo dev
status: planned

# Week 2: Small team
status: in-progress
tags: [api, feature]
priority: high

# Month 3: Enterprise
assignee: alice
epic: PROJ-123
sprint: 2025-Q4-S3
```

Custom fields supported. Adapts to your workflow.

</td>
<td width="50%" valign="top">

### ⚡ Actually Maintainable

**Why specs get stale:** They're too painful to update.

**LeanSpec solution:**

- Short specs = 5 min updates
- CLI tools = easy viewing/editing
- Structured format = AI can help update
- Living docs = evolve with code

**Result:** Specs that stay in sync.

</td>
</tr>
</table>

---

## Try It Now (5 Minutes)

```bash
# Install
npm install -g harnspec

# Initialize in your project
cd your-project
harnspec init

# Create your first spec
harnspec create user-authentication

# View it
harnspec view user-authentication

# See the Kanban board
harnspec board
```

**What you'll discover:**

- ✅ Creating a spec takes <2 minutes
- ✅ Structure is clear, not constraining  
- ✅ You can start simple, add fields later
- ✅ AI agents can immediately work with it

**Next steps:**

- 📘 [Full CLI Reference](https://harnspec.gitub.io/docs/cli-reference) - All commands with examples
- 🎨 [Choose a Template](https://harnspec.gitub.io/docs/templates) - Minimal, standard, or enterprise
- 🤖 [AI Agent Setup](../../AGENTS.md) - Configure Cursor, Claude, Aider

---

## Who's Using LeanSpec

### AI-First Development Teams

Give agents clear context without context window overload. Works with Cursor, Copilot, Aider, Claude.

### Scaling Startups

One methodology from solo dev → team → enterprise. Add structure progressively as you grow.

### Teams Outgrowing "Just Code"

Need structure for AI agents and team alignment, but heavyweight processes are too slow.

### Developers Building AI Agents

MCP-native specs for autonomous coding workflows. Structured input format agents can parse reliably.

---

## We Practice What We Preach

<table>
<tr>
<td align="center" width="25%">
<strong>54</strong><br>
Specs
</td>
<td align="center" width="25%">
<strong>287</strong><br>
Avg lines
</td>
<td align="center" width="25%">
<strong>0</strong><br>
Over 400 lines
</td>
<td align="center" width="25%">
<strong>547</strong><br>
Updates in 6mo
</td>
</tr>
</table>

**LeanSpec is built using LeanSpec.** Every feature, refactor, and design decision has a spec. All specs follow the first principles. We dogfood our own methodology.

→ [Browse our specs](https://github.com/codervisor/harnspec/tree/main/specs)

---

## When to Use (and Skip) Specs

| Use LeanSpec When: | Skip It When: |
|---------------------|---------------|
| ✅ Features span multiple files/components | ❌ Trivial bug fixes |
| ✅ Architecture decisions need alignment | ❌ Self-explanatory refactors |
| ✅ Guiding AI agents on complex features | ❌ Pure API reference (use code comments) |
| ✅ Design rationale should be documented | ❌ Quick experiments |
| ✅ Team needs to coordinate work | ❌ Changes are obvious |

**Philosophy:** Write specs when they add clarity. Skip them when they don't.

---

## Learn More

### 📚 Documentation

- [Getting Started Guide](https://harnspec.gitub.io/docs/getting-started) - Complete setup walkthrough
- [First Principles](https://harnspec.gitub.io/docs/guide/first-principles) - The philosophy behind LeanSpec
- [CLI Reference](https://harnspec.gitub.io/docs/cli-reference) - All commands with examples

### 🛠️ Integrations

- [AI Agent Configuration](../../AGENTS.md) - Cursor, Copilot, Aider setup
- [MCP Server](../../docs/MCP-SERVER.md) - Claude Desktop integration
- [VS Code Extension](https://harnspec.gitub.io/docs/tools/vscode) - Enhanced editor support

### 🎓 Guides

- [Custom Fields](https://harnspec.gitub.io/docs/guide/custom-fields) - Adapt to your workflow
- [Sub-Specs](https://harnspec.gitub.io/docs/guide/sub-specs) - Manage complex features
- [Folder Structure](https://harnspec.gitub.io/docs/guide/folder-structure) - Organize your specs

### 🤝 Community

- [GitHub Issues](https://github.com/codervisor/harnspec/issues) - Report bugs or request features
- [Contributing Guide](../../CONTRIBUTING.md) - Join the project
- [Examples](https://harnspec.gitub.io/docs/examples) - Real-world usage patterns

---

## License

MIT - See [LICENSE](LICENSE)

---

<p align="center">
  <strong>Built for developers who want SDD without the overhead.</strong><br>
  Keep your specs short. Keep them clear. Keep them useful.
</p>

---

> **See**: [CHANGES.md](CHANGES.md) for detailed comparison with current README and implementation strategy.
