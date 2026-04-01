---
status: archived
created: '2025-12-10'
tags:
  - architecture
  - ai-agents
  - memory
  - integration
  - agent-relay
priority: critical
created_at: '2025-12-10T06:08:47.689Z'
depends_on:
  - 123-ai-coding-agent-integration
  - 158-persistent-agent-sessions
updated_at: '2025-12-21T14:46:00.620Z'
transitions:
  - status: archived
    at: '2025-12-21T14:46:00.620Z'
---

# LeanSpec as Memory Layer for AI Agent Orchestration

> **Status**: 📦 Archived · **Priority**: Critical · **Created**: 2025-12-10 · **Tags**: architecture, ai-agents, memory, integration, agent-relay

## Overview

**Core Thesis**: LeanSpec is the **memory and intent layer** for AI agents, not an orchestration engine. Agent orchestration engines like **agent-relay** handle execution, session management, and multi-agent coordination, while LeanSpec provides persistent context, specifications, and historical memory.

**Current State**: This architectural clarification is documented in specs but not yet reflected in system prompts (AGENTS.md) since agent-relay is still alpha. LeanSpec continues to provide basic agent dispatch functionality while the orchestration engine matures.

### The Architectural Separation

```
┌─────────────────────────────────────────────────────────────────┐
│                    CODERVISOR PLATFORM                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   ┌──────────────┐      ┌──────────────┐      ┌─────────────┐  │
│   │  LeanSpec    │ ───► │ agent-relay  │ ───► │   Devlog    │  │
│   │  (Memory)    │      │ (Execution)  │      │ (Observ.)   │  │
│   │              │      │              │      │             │  │
│   │ • Specs      │      │ • HQ Server  │      │ • Activity  │  │
│   │ • Intent     │      │ • Runners    │      │ • Metrics   │  │
│   │ • History    │      │ • Sessions   │      │ • Audit     │  │
│   │ • Context    │      │ • PTY Mgmt   │      │ • Analytics │  │
│   │ • Decisions  │      │ • Multi-Agent│      │             │  │
│   └──────────────┘      └──────────────┘      └─────────────┘  │
│                                                                 │
│   Intent → Execution → Observability → Insights → New Intent   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Why This Matters

**Problem**: Spec 123/158 conflate memory management with orchestration:

- ❌ LeanSpec shouldn't manage agent sessions (that's orchestration)
- ❌ LeanSpec shouldn't spawn runners/terminals (that's execution)
- ❌ LeanSpec shouldn't handle WebSocket connections (that's infrastructure)

**Solution**: Clear separation of concerns:

- ✅ **LeanSpec**: Persistent spec storage, intent capture, historical context
- ✅ **agent-relay**: Agent orchestration, session management, execution
- ✅ **Devlog**: Observability, audit trails, analytics

## Design

### LeanSpec's Role: Memory Layer

**What LeanSpec Provides**:

1. **Persistent Specifications**
   - Structured intent documents (markdown + frontmatter)
   - Dependency graphs between specs
   - Status tracking (planned → in-progress → complete)
   - Token-optimized for AI context windows

2. **Historical Context**
   - What was built and why
   - Design decisions and tradeoffs
   - Implementation learnings
   - Evolution of intent over time

3. **Query Interface for Agents**
   - Search specs by query/tags/status
   - View spec content and dependencies
   - Update spec status programmatically
   - Token counting for context budgeting

4. **MCP Integration** (spec 028)
   - Expose specs as MCP resources
   - Tools for spec CRUD operations
   - Context injection for AI agents

### agent-relay's Role: Orchestration Engine

**What agent-relay Provides** ([github.com/codervisor/agent-relay](https://github.com/codervisor/agent-relay)):

1. **Agent Execution Infrastructure**
   - HQ server (centralized control plane)
   - Runners (distributed execution)
   - PTY management (terminal streaming)
   - WebSocket protocol (real-time communication)

2. **Session Management**
   - Multi-agent coordination
   - Session lifecycle (start/pause/resume/stop)
   - Resource allocation
   - Concurrent execution

3. **Tool Integration**
   - Launch GitHub Copilot, Claude, Cursor, etc.
   - Execute arbitrary CLI tools
   - Stream terminal output
   - Handle interactive sessions

### Integration Architecture

```typescript
// agent-relay reads from LeanSpec
interface AgentTask {
  specPath: string;          // Which spec to implement
  agent: string;             // Which agent to use
  session: Session;          // agent-relay session
}

// Workflow:
// 1. User dispatches spec to agent-relay
const task = agentRelay.dispatch({
  spec: 'harnspec/specs/045-api-redesign',
  agent: 'claude',
  context: leanSpec.getContext('045')  // LeanSpec provides context
});

// 2. agent-relay spawns runner with spec context
const session = agentRelay.createSession({
  runnerId: 'dev-machine-01',
  command: ['claude', 'implement'],
  context: {
    spec: leanSpec.view('045'),        // Full spec content
    dependencies: leanSpec.deps('045'),  // Upstream dependencies
    files: leanSpec.files('045')         // Sub-specs, assets
  }
});

// 3. Agent reads spec, executes work

// 4. Agent updates spec status via LeanSpec MCP
leanSpec.update('045', { status: 'in-progress' });

// 5. Devlog captures session activity
devlog.capture(session.id, {
  spec: '045',
  agent: 'claude',
  duration: '15m',
  filesChanged: ['api.ts', 'handlers.ts']
});

// 6. After completion, update spec
leanSpec.update('045', { status: 'complete' });
```

### Data Flow

```
┌────────────┐
│    User    │
└─────┬──────┘
      │ 1. Create spec
      ▼
┌────────────────┐
│   LeanSpec     │ ◄─── 6. Update status
│   (Memory)     │
└────────┬───────┘
         │ 2. Dispatch spec to agent
         ▼
┌────────────────┐
│  agent-relay   │
│ (Orchestration)│
└────────┬───────┘
         │ 3. Spawn runner
         ▼
┌────────────────┐
│    Runner      │
│  (Execution)   │
└────────┬───────┘
         │ 4. Execute work
         ▼
┌────────────────┐
│    Devlog      │ ◄─── 5. Capture activity
│ (Observability)│
└────────────────┘
```

## LeanSpec's Responsibilities

### ✅ What LeanSpec Should Do

1. **Spec Storage & Retrieval**
   - CRUD operations on spec files
   - Frontmatter parsing and validation
   - Dependency graph management
   - Full-text search

2. **Context Engineering**
   - Token counting
   - Sub-spec splitting
   - Template management
   - MCP resource exposure

3. **Intent Capture**
   - What to build (specifications)
   - Why decisions were made (design docs)
   - How it evolved (spec history)

4. **Query Interface**
   - CLI commands (`harnspec view`, `search`, `list`)
   - MCP tools (for AI agents)
   - Programmatic API (for integrations)

### ❌ What LeanSpec Should NOT Do

1. **Agent Orchestration**
   - ❌ Managing agent sessions
   - ❌ Spawning runners or processes
   - ❌ Coordinating multiple agents
   - ❌ Handling agent lifecycle

2. **Execution Infrastructure**
   - ❌ PTY management
   - ❌ WebSocket connections
   - ❌ Terminal streaming
   - ❌ Resource allocation

3. **Observability**
   - ❌ Logging agent activity
   - ❌ Capturing terminal output
   - ❌ Generating analytics
   - ❌ Audit trail management

**Guideline**: If it involves real-time process management, networking, or observability → agent-relay or Devlog. If it involves persistent intent/context → LeanSpec.

## Refactoring Spec 158

**Current spec 158** (Persistent AI Agent Sessions) assumes LeanSpec handles:

- Session state management ← agent-relay's job
- Phase-based workflows ← agent-relay's job
- Context continuity ← shared between LeanSpec (memory) and agent-relay (state)
- Session history ← Devlog's job

**Revised approach**:

1. **agent-relay** implements session persistence (spec 158 concepts)
2. **LeanSpec** provides read-only spec context to agent-relay
3. **Devlog** captures session activity and metrics

```typescript
// agent-relay session (implements spec 158 concepts)
interface AgentSession {
  id: string;
  specPath: string;           // Which LeanSpec spec
  agent: AgentType;
  status: 'active' | 'paused' | 'completed';
  currentPhase: string;
  
  // Context from LeanSpec
  specContext: {
    content: string;          // LeanSpec.view(spec)
    dependencies: string[];   // LeanSpec.deps(spec)
    tokens: number;           // LeanSpec.tokens(spec)
  };
  
  // Execution state (agent-relay)
  runnerId: string;
  worktree?: string;
  branch: string;
  
  // Observability (logged to Devlog)
  interactions: Interaction[];
  filesChanged: string[];
}
```

## Plan

### Phase 1: Document Architecture ✅

- [x] Create spec 159 defining separation of concerns
- [x] Update spec 123 (AI Agent Integration) to clarify LeanSpec's role
- [x] Update spec 158 to reference agent-relay for session management
- [x] Document integration patterns

### Phase 2: Update AGENTS.md and Documentation (Deferred)

- [ ] Update root AGENTS.md to reference agent-relay for orchestration (deferred until agent-relay production-ready)
- [ ] Add integration guide to docs-site
- [ ] Create examples of LeanSpec + agent-relay workflows
- [ ] Document MCP integration patterns

**Note**: AGENTS.md updates deferred until agent-relay reaches production readiness. Current LeanSpec agent commands work for simple dispatch; agent-relay integration will be added when the orchestration engine is stable.

- [ ] Move orchestration concerns from spec 123 to agent-relay repo
- [ ] Keep MCP integration in LeanSpec (read-only context provider)
- [ ] Simplify `harnspec agent run` to dispatch to agent-relay

### Phase 3: Define Integration Points

- [ ] LeanSpec → agent-relay: Spec context API
- [ ] agent-relay → LeanSpec: Status update callbacks
- [ ] agent-relay → Devlog: Session activity streams
- [ ] Devlog → LeanSpec: Link events to specs

### Phase 5: Integration Points Implementation

- [ ] LeanSpec → agent-relay: Spec context API (MCP tools)
- [ ] agent-relay → LeanSpec: Status update callbacks (MCP tools)
- [ ] agent-relay → Devlog: Session activity streams
- [ ] Devlog → LeanSpec: Link events to specs

### Phase 6: Testing & Documentation

- [ ] Test LeanSpec + agent-relay integration end-to-end
- [ ] Document workflow examples
- [ ] Create video tutorials
- [ ] Update marketing materials (spec 136)

## Test

### Integration Tests

- [ ] agent-relay can read LeanSpec specs via MCP
- [ ] agent-relay can update LeanSpec status
- [ ] Devlog can link events to LeanSpec specs

### User Experience

- [ ] Single command workflow: `harnspec create` → `agent-relay dispatch`
- [ ] Seamless context flow: spec → agent → activity log
- [ ] Can use each tool independently

## Notes

### Why This Separation?

**1. Single Responsibility**

- LeanSpec = Intent/Memory (what and why)
- agent-relay = Execution (how and when)
- Devlog = Observability (what happened)

**2. Composability**

- Use LeanSpec without agent-relay (manual implementation)
- Use agent-relay without LeanSpec (ad-hoc tasks)
- Mix and match agents/tools

**3. Development Velocity**

- LeanSpec can evolve memory/context features independently
- agent-relay can optimize orchestration without breaking specs
- Devlog can enhance observability without touching either

**4. Open Ecosystem**

- Other orchestration engines can read LeanSpec specs
- Other spec formats can be used with agent-relay
- Observability can integrate with other platforms

### Comparison: Before vs After

| Concern | Before (spec 123/158) | After (spec 159) |
|---------|----------------------|------------------|
| Spec storage | LeanSpec | LeanSpec |
| Spec search | LeanSpec | LeanSpec |
| Session management | LeanSpec | agent-relay |
| Agent orchestration | LeanSpec | agent-relay |
| PTY/Terminal | LeanSpec | agent-relay |
| Activity logging | None | Devlog |
| Audit trails | None | Devlog |
| Intent capture | LeanSpec | LeanSpec |
| Historical context | LeanSpec | LeanSpec |

### Implementation Status

**agent-relay** (alpha, not production-ready):

- ✅ Reverse-connection architecture (Runners dial HQ)
- ✅ WebSocket protocol (PTY streaming)
- ✅ Terminal UI (xterm.js)
- ✅ Multi-runner support
- ⏳ Session persistence (spec 158 concepts)
- ⏳ LeanSpec integration
- ⏳ Phase-based workflows

**LeanSpec** (stable):

- ✅ Spec storage and CRUD
- ✅ MCP integration
- ✅ Search and discovery
- ✅ Basic agent dispatch (`harnspec agent run`)
- ⏳ agent-relay integration (deferred until agent-relay production-ready)

### Related Specs

**LeanSpec**:

- **028-mcp-server**: MCP integration for AI agents
- **123-ai-coding-agent-integration**: Agent dispatch (needs refactor)
- **158-persistent-agent-sessions**: Session concepts (move to agent-relay)
- **136-growth-marketing-strategy-v2**: Platform positioning

**agent-relay** (separate repo):

- **001-project-init**: Initial architecture
- **003-core-implementation**: HQ and Runner implementation
- **004-modern-web-ui**: Web dashboard
- *(New)* **persistent-sessions**: Implement LeanSpec spec 158 concepts

### Open Questions

1. **Should LeanSpec CLI keep `harnspec agent run`?**
   - Option A: Deprecated, redirect to agent-relay
   - Option B: Simple proxy that calls agent-relay
   - Option C: Remove entirely
   - **Recommendation**: Keep as simple dispatcher if agent-relay installed

2. **How does agent-relay update LeanSpec status?**
   - Option A: Direct file system write (if co-located)
   - Option B: MCP tool calls (if remote)
   - Option C: Webhook/API callbacks
   - **Recommendation**: MCP tools (most flexible)

3. **Session state storage location?**
   - Option A: `.leanspec/sessions/` (current spec 158 design)
   - Option B: `agent-relay/sessions/` (separate)
   - **Recommendation**: agent-relay manages its own state

4. **Context injection strategy?**
   - How much spec content does agent-relay cache?
   - When to re-query LeanSpec for updates?
   - **Recommendation**: Minimal cache, query on-demand
