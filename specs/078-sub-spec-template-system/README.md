---
status: archived
created: 2025-11-13
priority: medium
tags:
- templates
- cli
- dx
- ai-first
- sub-specs
depends_on:
- 012-sub-spec-files
- 073-template-engine-agents-md
created_at: 2025-11-13T13:31:48.324Z
updated_at: 2026-01-16T06:54:54.295050Z
transitions:
- status: archived
  at: 2026-01-16T06:54:54.295050Z
---

# sub-spec-template-system

> **Status**: 🗓️ Planned · **Priority**: Medium · **Created**: 2025-11-13 · **Tags**: templates, cli, dx, ai-first, sub-specs

**Project**: harnspec  
**Team**: Core Development

## Overview

**Problem**: Current `.harnspec/templates/` system only generates main spec file (README.md), but specs often need sub-files like DESIGN.md, TESTING.md, IMPLEMENTATION.md, etc.

**Current Limitations**:

- Template system generates only README.md
- Users manually create sub-spec files after initial creation
- No support for optional vs required sub-specs
- AI agents and automation tools need programmatic sub-spec creation

**Goal**: Enable template system to generate optional sub-spec files using:

- Declarative flags (`--design`, `--testing`) for AI/automation
- File conventions (`.opt`, `.req`) for template authors
- Interactive mode (`--with-subs`) for human users (opt-in)

**Origin**: Split from spec 073 Phase 2 (AGENTS.md template engine complete, sub-spec system separate concern)

## Design

### Proposed Template Structure

```
.harnspec/
└── templates/
    ├── default/
    │   ├── README.md         # Main spec template (always generated)
    │   ├── DESIGN.md.opt     # Optional sub-spec
    │   └── TESTING.md.opt    # Optional sub-spec
    └── api/
        ├── README.md
        ├── API.md.req        # Required sub-spec (always included)
        └── SCHEMAS.md.opt    # Optional sub-spec
```

### File Conventions

- **`{name}.md`** - Always generated (main spec)
- **`{name}.md.opt`** - Optional, generated only if requested via flag or prompt
- **`{name}.md.req`** - Required, always generated for this template

### Usage Patterns (AI-First Design)

**Primary Interface: Explicit Flags** (optimized for AI agents)

```bash
# Generate with specific sub-specs
harnspec create my-feature --design --testing
# → Creates README.md + DESIGN.md + TESTING.md

# No flags = README.md only (default)
harnspec create simple-fix
```

**Secondary Interface: Interactive Mode** (opt-in for humans)

```bash
harnspec create my-feature --with-subs
# Prompts user to select which optional sub-specs to include
? Include optional sub-specs? (space to select)
  [ ] DESIGN.md
  [x] TESTING.md
  [ ] IMPLEMENTATION.md
```

**Rationale**: AI agents use CLI programmatically and can't handle interactive prompts. Flags provide declarative, non-blocking interface.

### Config Schema Enhancement

```json
{
  "templates": {
    "default": {
      "main": "README.md",
      "optional": ["DESIGN.md", "TESTING.md", "IMPLEMENTATION.md"],
      "required": [],
      "flags": {
        "--design": "DESIGN.md",
        "--testing": "TESTING.md",
        "--implementation": "IMPLEMENTATION.md"
      }
    },
    "api": {
      "main": "README.md",
      "optional": ["SCHEMAS.md"],
      "required": ["API.md"],
      "flags": {
        "--schemas": "SCHEMAS.md"
      }
    }
  }
}
```

### Technical Implementation

**Extend `packages/cli/src/commands/creator.ts`**:

1. Parse sub-spec flags from command line
2. Detect `.opt` and `.req` files in template directory
3. Filter files based on flags and conventions
4. Apply variable substitution to all generated files
5. Fall back to interactive mode if `--with-subs` flag present

**Template Resolution Logic**:

```typescript
// Pseudo-code
function resolveTemplateFiles(templateDir, flags, interactive) {
  const files = [];
  
  // Always include main file
  files.push('README.md');
  
  // Add required sub-specs
  files.push(...glob('*.req').map(f => f.replace('.req', '')));
  
  // Add optional sub-specs based on flags
  if (flags.design) files.push('DESIGN.md');
  if (flags.testing) files.push('TESTING.md');
  
  // Or prompt if interactive mode
  if (interactive) {
    const selected = await promptForSubSpecs(glob('*.opt'));
    files.push(...selected);
  }
  
  return files;
}
```

## Plan

### Phase 1: Design & Planning

- [ ] Define `.opt` / `.req` file convention spec
- [ ] Design config schema for template metadata and flag mappings
- [ ] Design flag-based interface (primary) + interactive mode (opt-in)
- [ ] Document sub-spec template authoring guide

### Phase 2: Core Implementation

- [ ] Extend `creator.ts` to handle sub-spec templates
- [ ] Implement file discovery (`.opt`, `.req` conventions)
- [ ] Add explicit sub-spec flags (`--design`, `--testing`, `--implementation`)
- [ ] Implement file filtering logic based on flags
- [ ] Update template resolution and variable substitution

### Phase 3: Interactive Mode

- [ ] Add `--with-subs` flag for interactive mode
- [ ] Implement interactive prompt for sub-spec selection
- [ ] Handle multi-select UI for optional sub-specs

### Phase 4: Template Updates

- [ ] Create example templates with sub-specs
- [ ] Update existing templates with `.opt` convention
- [ ] Add template metadata to config.json files
- [ ] Update default template with common sub-specs

### Phase 5: Documentation & Testing

- [ ] Update CLI documentation for sub-spec flags
- [ ] Create sub-spec template authoring guide
- [ ] Update AGENTS.md with new commands
- [ ] Add integration tests for all scenarios
- [ ] Test AI agent workflows

## Test

### Test Scenarios

**Basic Sub-Spec Generation**:

- [ ] `harnspec create feat --design` generates README.md + DESIGN.md
- [ ] `harnspec create feat --design --testing` generates README.md + DESIGN.md + TESTING.md
- [ ] `harnspec create feat` (no flags) generates only README.md

**Interactive Mode**:

- [ ] `harnspec create feat --with-subs` shows interactive prompt
- [ ] Multi-select allows choosing multiple optional sub-specs
- [ ] Selected sub-specs are generated correctly

**Required Sub-Specs**:

- [ ] Templates with `.req` files always generate those files
- [ ] Required files included even without flags
- [ ] Example: API template always includes API.md

**Variable Substitution**:

- [ ] Variables work correctly in sub-spec templates
- [ ] Spec name, date, and custom variables populated
- [ ] Frontmatter generated correctly in sub-specs

**Edge Cases**:

- [ ] Invalid flag ignored gracefully
- [ ] Template without sub-specs works as before
- [ ] Mixed `.opt` and `.req` files handled correctly

### AI Agent Test Protocol

```bash
# AI agent determines feature needs DESIGN + TESTING
# Should invoke: 
harnspec create feature --design --testing

# Not this (requires interaction):
harnspec create feature --with-subs
```

### Success Criteria

- [ ] AI agents can create specs with sub-specs programmatically
- [ ] No breaking changes to existing `harnspec create` behavior
- [ ] Interactive mode provides good UX for human users
- [ ] Template authors can easily add sub-spec support

## Notes

### Why AI-First Design?

In AI-human co-op spec writing mode:

- **AI agents primarily use the CLI**, not humans
- **Interactive prompts block automation** - AI can't respond to prompts effectively
- **Flags are declarative** - AI can determine needed sub-specs and invoke with explicit flags
- **Humans can opt-in** - `--with-subs` flag preserves interactive experience when desired

Example AI workflow:

```
AI analyzes task → determines needs DESIGN + TESTING sub-specs
→ runs: harnspec create feature --design --testing
→ no interaction needed, continues working
```

This aligns with 072-ai-agent-first-use-workflow principles: optimize for AI, accommodate humans.

### Related Specs

- **073-template-engine-agents-md**: Completed Phase 1 (AGENTS.md template engine), Phase 2 split into this spec
- **012-sub-spec-files**: Original sub-spec design and conventions (archived)
- **072-ai-agent-first-use-workflow**: AI-first design principles
- **074-content-at-creation**: Related but different - passing content at creation vs sub-file generation

### Split from Spec 073

Originally planned as Phase 2 of spec 073, split into separate spec because:

- Phase 1 (AGENTS.md template engine) is complete and independent
- Phase 2 (sub-spec template system) is a different feature with different scope
- Cleaner separation allows independent planning and archiving
- Phase 1 can be archived when appropriate while Phase 2 continues

### Open Questions

- Should we support nested sub-specs (e.g., `design/architecture.md`)?
- How to handle conflicts if sub-spec file already exists?
- Should flags be cumulative or last-wins?
- Should we add `--all-subs` flag to include all optional sub-specs?
- How to discover available sub-specs for a template?

### Alternative Approaches Considered

**1. Single `--subs` flag with comma-separated list**

```bash
harnspec create feat --subs=design,testing
```

- ✅ More concise
- ❌ Less discoverable (need to know names)
- ❌ Less idiomatic (most CLIs use separate flags)

**2. Always prompt for sub-specs**

```bash
harnspec create feat
? Include sub-specs? (Y/n)
```

- ❌ Blocks automation
- ❌ Annoying for quick creates
- ❌ Not AI-friendly

**3. Config file to specify defaults**

```json
{
  "defaults": {
    "subSpecs": ["DESIGN.md", "TESTING.md"]
  }
}
```

- ✅ Could work for power users
- ⚠️ Adds complexity
- 🤔 Could be added later as enhancement

**Decision**: Explicit flags as primary interface, interactive as opt-in secondary.
