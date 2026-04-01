---
status: complete
created: '2025-11-17'
tags:
  - docs
  - ux
  - beginner-friendly
priority: high
created_at: '2025-11-17T07:51:56.155Z'
updated_at: '2025-11-26T02:35:47.462Z'
transitions:
  - status: in-progress
    at: '2025-11-17T08:35:10.599Z'
  - status: complete
    at: '2025-11-17T08:35:19.888Z'
completed_at: '2025-11-17T08:35:19.888Z'
completed: '2025-11-17'
---

# Documentation: Beginner-First Reorganization

> **Status**: ✅ Complete · **Priority**: High · **Created**: 2025-11-17 · **Tags**: docs, ux, beginner-friendly

**Project**: harnspec  
**Team**: Core Development

## Overview

**Problem**: Current docs site structure mixes beginner and advanced content, making it harder for new users to find the right information. Advanced topics in "Core Concepts" create cognitive load, and case studies lack intuitive organization.

**Solution**: Reorganize docs with beginner-first approach:

1. Separate "Advanced Topics" section for deep theory
2. Individual terminology pages with clear explanations
3. Streamline "Understanding LeanSpec" for beginners only
4. Refocus "Tutorials" on AI-assisted spec writing
5. Promote case studies to top-level with better naming
6. Link to web app (harnspec.gitub.io) instead of GitHub

**Why now**: Following specs 88-92 migrations, documentation content is accurate but structure needs optimization for user experience.

## Design

### 1. Advanced Topics Separation

**Current state**: "Core Concepts" contains:

- Terminology ✅ (beginner-friendly)
- Understanding LeanSpec ✅ (beginner-friendly)
- First Principles 🔴 (advanced)
- Context Engineering 🔴 (advanced)
- AI Agent Memory 🔴 (advanced)
- Philosophy & Mindset 🔴 (advanced)
- Limits and Tradeoffs 🔴 (advanced)

**New structure**:

```
Guide
├── Introduction
│   ├── Overview
│   ├── Getting Started
│   ├── Comparison
│   └── Migration
├── Tutorials (AI-focused)
│   ├── Writing Your First Spec with AI
│   ├── AI-Assisted Feature Development
│   ├── Managing Multiple Specs with AI
│   └── Team Collaboration with AI Agents
├── Core Concepts (beginner-only)
│   ├── What is LeanSpec?
│   ├── Understanding Specs
│   └── Terminology
│       ├── Spec
│       ├── Status
│       ├── SDD Workflow
│       ├── Sub-specs
│       ├── Dependencies
│       └── Tags & Priority
├── Usage
│   ├── Essential Usage
│   ├── Project Management
│   ├── Advanced Features
│   └── AI-Assisted Workflows
├── Advanced Topics (NEW!)
│   ├── First Principles
│   ├── Context Engineering
│   ├── AI Agent Memory
│   ├── Philosophy & Mindset
│   └── Limits and Tradeoffs
├── Roadmap
└── FAQ

Real-World Examples (top-level, NEW!)
├── Overview
├── Simple Feature
├── Complex Feature
├── Refactoring Project
└── Cross-Team Coordination

Reference
├── CLI
├── Configuration
├── Frontmatter
└── MCP Server
```

### 2. Terminology as Individual Pages

**Current**: Single `terminology.md` with all terms listed.

**New**: Each term gets its own page under `Core Concepts/Terminology/`:

- `spec.mdx` - What is a spec? Clear definition, examples
- `status.mdx` - Spec lifecycle states
- `sdd-workflow.mdx` - Spec-Driven Development explained
- `sub-specs.mdx` - When and how to split
- `dependencies.mdx` - Relationships between specs
- `tags-priority.mdx` - Organization and prioritization

**Benefits**:

- Easier to find specific concepts
- More space for explanations and examples
- Better SEO (one concept per URL)
- Can add visual diagrams per concept

### 3. Understanding LeanSpec Optimization

**Current**: `understanding.md` mixes beginner explanations with advanced concepts.

**New**: Split into two:

- `Core Concepts/what-is-leanspec.mdx` - Beginner-only
  - What problems does LeanSpec solve?
  - How is it different?
  - When should I use it?
  - Quick mental model
- `Core Concepts/understanding-specs.mdx` - Working with specs
  - Anatomy of a spec
  - Reading vs writing specs
  - Evolution during development
  
**Move to Advanced Topics**:

- Deep theory about why specs work
- Cognitive science connections
- Advanced workflow patterns

### 4. AI-First Tutorials

**Current**: Tutorials show manual spec writing.

**New**: Focus on AI-assisted workflows using VS Code + GitHub Copilot Agent Mode:

1. **Writing Your First Spec with AI**
   - User provides intent: "I want to add user authentication"
   - Copilot Agent Mode expands into proper spec
   - User reviews and refines
   - Demo video/screenshots

2. **AI-Assisted Feature Development**
   - Full SDD workflow with AI
   - AI writes spec from requirements
   - AI implements from spec
   - User validates and guides

3. **Managing Multiple Specs with AI**
   - AI helps find related specs
   - AI suggests dependencies
   - AI updates status tracking

4. **Team Collaboration with AI Agents**
   - AI agents read team specs
   - AI proposes changes
   - Human review and approval

**Format**: Each tutorial includes:

- Video walkthrough (Loom/YouTube)
- Step-by-step screenshots
- Code snippets with Copilot prompts
- Expected AI responses
- Common issues and fixes

### 5. Case Studies → Real-World Examples

**Rename**: "Case Studies" → "Real-World Examples" (more approachable than academic "case studies")

**Promote to top-level**: Move from under "Guide" to its own top-level section in sidebar.

**Add overview page**: Enhanced version of current `index.mdx` with:

- Visual overview (grid/cards)
- Filter by complexity (simple/complex/refactor/cross-team)
- Time to complete estimates
- Learning objectives

**Link to web app**: Replace GitHub links with web app links:

- ❌ Old: `github.com/codervisor/harnspec/tree/main/specs/071-...`
- ✅ New: `harnspec.gitub.io/specs/071`

**Benefits**:

- Better reading experience (web app has syntax highlighting, navigation)
- Consistent with product ecosystem
- Easier to maintain (one canonical link)

### 6. Sidebar Structure Changes

**File**: `docs-site/sidebars.ts`

```typescript
const sidebars: SidebarsConfig = {
  guideSidebar: [
    {
      type: 'category',
      label: 'Introduction',
      items: ['guide/index', 'guide/getting-started', 'comparison', 'guide/migration'],
    },
    {
      type: 'category',
      label: 'Tutorials',
      items: [
        'tutorials/writing-first-spec-with-ai',
        'tutorials/ai-assisted-feature-development',
        'tutorials/managing-multiple-specs-with-ai',
        'tutorials/team-collaboration-ai-agents',
      ],
    },
    {
      type: 'category',
      label: 'Core Concepts',
      items: [
        'guide/what-is-leanspec',
        'guide/understanding-specs',
        {
          type: 'category',
          label: 'Terminology',
          items: [
            'guide/terminology/spec',
            'guide/terminology/status',
            'guide/terminology/sdd-workflow',
            'guide/terminology/sub-specs',
            'guide/terminology/dependencies',
            'guide/terminology/tags-priority',
          ],
        },
      ],
    },
    {
      type: 'category',
      label: 'Usage',
      items: [
        // existing structure unchanged
      ],
    },
    {
      type: 'category',
      label: 'Advanced Topics',
      items: [
        'advanced/first-principles',
        'advanced/context-engineering',
        'advanced/ai-agent-memory',
        'advanced/philosophy',
        'advanced/limits-and-tradeoffs',
      ],
    },
    'roadmap',
    'faq',
  ],
  examplesSidebar: [
    {
      type: 'category',
      label: 'Real-World Examples',
      items: [
        'examples/index',
        'examples/simple-feature-token-validation',
        'examples/complex-feature-web-sync',
        'examples/refactoring-monorepo-core',
        'examples/cross-team-official-launch',
      ],
    },
  ],
  referenceSidebar: [
    'reference/cli',
    'reference/config',
    'reference/frontmatter',
    'reference/mcp-server',
  ],
};
```

**Config update**: `docusaurus.config.ts` - Add "Examples" to navbar alongside "Guide" and "Reference".

## Plan

### Phase 1: Structure Setup

- [ ] Create `docs/advanced/` directory
- [ ] Create `docs/examples/` directory (rename from case-studies)
- [ ] Create `docs/guide/terminology/` directory with individual term pages
- [ ] Update `sidebars.ts` with new structure
- [ ] Update `docusaurus.config.ts` navbar

### Phase 2: Content Migration & Creation

- [ ] Move advanced topics to `docs/advanced/`:
  - [ ] `first-principles.mdx`
  - [ ] `context-engineering.mdx`
  - [ ] `ai-agent-memory.mdx`
  - [ ] `philosophy.mdx`
  - [ ] `limits-and-tradeoffs.mdx`
- [ ] Split `terminology.md` into individual pages:
  - [ ] `terminology/spec.mdx`
  - [ ] `terminology/status.mdx`
  - [ ] `terminology/sdd-workflow.mdx`
  - [ ] `terminology/sub-specs.mdx`
  - [ ] `terminology/dependencies.mdx`
  - [ ] `terminology/tags-priority.mdx`
- [ ] Optimize `understanding.mdx` → `what-is-leanspec.mdx` + `understanding-specs.mdx`
- [ ] Rename case-studies → examples, update all links

### Phase 3: Tutorials Rewrite (AI-focused)

- [ ] Rewrite tutorial 1: Writing first spec with AI (+ video)
- [ ] Rewrite tutorial 2: AI-assisted feature development (+ video)
- [ ] Rewrite tutorial 3: Managing multiple specs with AI
- [ ] Rewrite tutorial 4: Team collaboration with AI agents

### Phase 4: Examples Enhancement

- [ ] Update `examples/index.mdx` with visual overview
- [ ] Replace all GitHub links with harnspec.gitub.io links
- [ ] Add complexity filters/tags
- [ ] Add time-to-complete estimates
- [ ] Add learning objectives to each example

### Phase 5: Polish & Validation

- [ ] Update all internal cross-references
- [ ] Run `npm run build` to verify
- [ ] Test navigation flows (beginner → intermediate → advanced)
- [ ] Update AGENTS.md with new structure
- [ ] Get feedback from fresh users

## Test

**Build validation**:

- [ ] `cd docs-site && npm run build` succeeds
- [ ] No broken links (`npm run build` catches broken internal links)
- [ ] All pages render correctly in dev mode

**User experience validation**:

- [ ] New user can find "Getting Started" easily
- [ ] Beginner doesn't encounter advanced theory prematurely
- [ ] Terminology concepts are easy to understand
- [ ] Tutorials demonstrate AI-assisted workflows clearly
- [ ] Examples link to web app instead of GitHub

**Content validation**:

- [ ] All moved content maintains frontmatter
- [ ] SEO metadata updated for new structure
- [ ] Search functionality works with new paths
- [ ] Sidebar navigation is intuitive

**AI agent validation**:

- [ ] AGENTS.md references correct doc paths
- [ ] AI agents can find beginner vs advanced content
- [ ] Tutorial instructions work with Copilot Agent Mode

## Notes

### Migration Strategy

**Low risk**: Content already accurate (thanks to specs 88-92), we're just reorganizing structure.

**Approach**:

1. Create new directories first
2. Copy files to new locations (don't delete originals yet)
3. Update all links and references
4. Test thoroughly
5. Delete old files only after verification

### Naming Decisions

**"Real-World Examples" vs "Case Studies"**:

- "Examples" is more approachable
- "Real-World" signals practical, not academic
- Matches common developer docs patterns (React, Vue, etc.)

**"Advanced Topics" vs "Deep Dive" or "Theory"**:

- "Advanced Topics" is clear and standard
- Sets expectation: "Not for beginners"
- Matches user mental model

### Future Enhancements (Out of Scope)

- Interactive tutorials (embedded editor)
- Video series for each section
- Community-contributed examples
- Multi-language support (already in progress with spec 64)

### Related Specs

- Spec 088: Core Concepts rewrite (content)
- Spec 089: AI Agent Memory content
- Spec 090: Limits and Tradeoffs content
- Spec 091: Philosophy content
- Spec 092: Docs site submodule migration (infrastructure)

This spec focuses on **structure and UX**, not content accuracy (already done).
