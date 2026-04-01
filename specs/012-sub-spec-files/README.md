---
status: archived
created: '2025-11-01'
tags:
  - enhancement
  - cli
  - specs
  - organization
priority: medium
completed: '2025-11-02'
---

# Sub-Spec Files Management

> **Status**: ✅ Complete · **Priority**: Medium · **Created**: 2025-11-01 · **Tags**: enhancement, cli, specs, organization

## Implementation Status

**Phase 1 (Core Support) - ✅ Implemented (2025-11-06)**

- ✅ `loadSubFiles()` function implemented in `spec-loader.ts`
- ✅ `harnspec files <spec>` command implemented
- ✅ `harnspec view <spec>/SUBFILE.md` - View sub-spec files directly
- ✅ `harnspec open <spec>/SUBFILE.md` - Open sub-spec files in editor
- ✅ Sub-spec viewing with all output modes (formatted, --raw, --json)
- ✅ Full test coverage for sub-spec viewing
- ✅ Documentation updated (README.md, AGENTS.md, CLI help)

**Not Yet Implemented:**

- ⏸️ Phase 2: Enhanced `harnspec create` with `--files` option
- ⏸️ Phase 3: `harnspec validate` link checker
- ⏸️ Phase 4: `harnspec merge` command

**Key Feature:** You can now view and open sub-spec files using paths like:

```bash
harnspec view 045/IMPLEMENTATION.md
harnspec view my-spec/TESTING.md --raw
harnspec open 012/DESIGN.md
```

## Overview

Enable specs to contain multiple organized documents beyond just `README.md`. As specs grow in complexity, teams need to split content into focused sub-documents while maintaining discoverability and structure.

**Current State**: Each spec is a single `README.md` file. Some specs informally add extra files (e.g., `SUMMARY.md`, `TEST_SUMMARY.md`), but there's no standardized approach or tooling support.

**Goal**: Provide a lightweight, discoverable system for organizing multi-document specs that:

- Keeps the main `README.md` as the entry point (backwards compatible)
- Allows structured sub-documents with clear naming conventions
- Provides CLI commands to discover and work with sub-files
- Maintains the LeanSpec philosophy: only add structure when you feel the pain

## The Problem

Real-world scenarios where single-file specs break down:

1. **Test Documentation**: A feature spec with extensive test plans/results
   - `README.md` - Feature spec
   - `TESTING.md` - Test strategy and results
   - `TEST_SUMMARY.md` - Quick test summary

2. **API Specifications**: API design with detailed endpoints
   - `README.md` - API overview and design decisions
   - `ENDPOINTS.md` - Detailed endpoint specifications
   - `SCHEMAS.md` - Data models and validation rules
   - `EXAMPLES.md` - Request/response examples

3. **Architecture Decisions**: Complex system designs
   - `README.md` - Architecture overview
   - `DECISIONS.md` - ADR-style decision log
   - `DIAGRAMS.md` - Architecture diagrams and explanations
   - `MIGRATION.md` - Migration strategy from old to new

4. **Epic/Feature Sets**: Large features broken into phases
   - `README.md` - Epic overview
   - `PHASE1.md` - Phase 1 implementation details
   - `PHASE2.md` - Phase 2 implementation details
   - `ROLLOUT.md` - Deployment and rollout strategy

5. **Research & Prototyping**: Exploratory work with multiple approaches
   - `README.md` - Research goals and summary
   - `APPROACH_A.md` - First approach investigation
   - `APPROACH_B.md` - Alternative approach
   - `FINDINGS.md` - Comparative analysis and recommendation

## Design

### File Organization Principles

**1. README.md is Always the Entry Point**

- Every spec MUST have a `README.md`
- It contains the frontmatter (single source of truth for metadata)
- It provides overview and links to sub-documents
- Tooling always loads `README.md` first

**2. Naming Conventions**

Follow these patterns for discoverability:

```
specs/YYYYMMDD/NNN-spec-name/
├── README.md           # Main spec (required, has frontmatter)
├── TESTING.md          # Test-related content
├── API.md              # API specifications
├── ARCHITECTURE.md     # Architecture decisions
├── MIGRATION.md        # Migration strategy
├── PHASE{N}.md         # Phase-based breakdown
├── RESEARCH.md         # Research findings
├── DECISIONS.md        # Decision log (ADR-style)
├── EXAMPLES.md         # Code/usage examples
├── DIAGRAMS.md         # Visual diagrams
├── ROLLOUT.md          # Deployment plans
└── assets/             # Images, diagrams, etc.
    ├── diagram.png
    └── flow.svg
```

**3. Sub-Document Structure**

Each sub-document should follow this lightweight template:

```markdown
# {Document Title}

> Part of spec: [009-sub-spec-files](README.md)

## {Section}

Content here...

## Related Documents

- [Main Spec](README.md) - Overview and goals
- [Testing](TESTING.md) - Test strategy
```

**4. README.md Integration**

Main spec should link to sub-documents in a clear section:

```markdown
## Documentation Structure

This spec is organized into multiple documents:

- **[Testing Strategy](TESTING.md)** - Test plans and results
- **[API Specification](API.md)** - Detailed endpoint specs
- **[Migration Plan](MIGRATION.md)** - How to migrate from v1 to v2

## Quick Links

- 📋 [Full test results](TESTING.md#results)
- 🔗 [API endpoints](API.md#endpoints)
- 🚀 [Rollout timeline](MIGRATION.md#timeline)
```

### CLI Commands

#### 1. `harnspec create` Enhancement

Support creating specs with templates that include sub-files:

```bash
# Create spec with sub-files from template
harnspec create my-api-spec --template=api
# Creates: README.md, API.md, SCHEMAS.md, EXAMPLES.md

# Create spec with custom sub-files
harnspec create my-feature --files=TESTING.md,MIGRATION.md
# Creates: README.md, TESTING.md, MIGRATION.md

# Add sub-file to existing spec
harnspec add specs/20251101/001-my-spec/TESTING.md
# Creates TESTING.md with proper template and links
```

#### 2. `harnspec files` - List Sub-Documents

Show all files in a spec:

```bash
$ harnspec files specs/20251101/009-sub-spec-files

📄 Files in 009-sub-spec-files

Required:
  ✓ README.md              (2.4 KB)  Main spec

Documents:
  ✓ TESTING.md             (1.8 KB)  Test strategy and results
  ✓ API.md                 (3.2 KB)  API endpoints and schemas
  ✓ MIGRATION.md           (1.1 KB)  Migration plan

Assets:
  ✓ assets/diagram.png     (45 KB)   Architecture diagram
  ✓ assets/flow.svg        (8 KB)    User flow

Total: 6 files, 61.5 KB
```

**Options:**

- `--type=docs` - Show only markdown documents
- `--type=assets` - Show only assets
- `--tree` - Show as tree structure
- `--json` - Output as JSON

#### 3. `harnspec toc` - Generate Table of Contents

Auto-generate cross-document table of contents:

```bash
$ harnspec toc specs/20251101/009-sub-spec-files

# 009-sub-spec-files

## Overview
- [Main Spec](README.md#overview)
- [Design](README.md#design)

## Testing
- [Test Strategy](TESTING.md#strategy)
- [Test Results](TESTING.md#results)
- [Coverage Report](TESTING.md#coverage)

## API
- [Endpoints](API.md#endpoints)
  - [Create User](API.md#create-user)
  - [Update User](API.md#update-user)
- [Schemas](API.md#schemas)

## Migration
- [Timeline](MIGRATION.md#timeline)
- [Breaking Changes](MIGRATION.md#breaking-changes)
- [Rollback Plan](MIGRATION.md#rollback)
```

**Options:**

- `--depth=2` - Control heading depth
- `--output=TOC.md` - Write to file
- `--insert` - Insert into README.md

#### 4. `harnspec validate` - Check Spec Integrity

Validate spec structure and cross-references:

```bash
$ harnspec validate specs/20251101/009-sub-spec-files

✓ README.md exists and has valid frontmatter
✓ All sub-documents linked from README.md
✗ Broken link in TESTING.md: [See API](API.md#nonexistent)
✗ Orphaned file: DEPRECATED.md (not linked from any document)
⚠ Large asset: assets/video.mp4 (250 MB)

2 errors, 1 warning
```

**Checks:**

- README.md exists with valid frontmatter
- All sub-documents are referenced from README.md or other docs
- No broken internal links
- No orphaned files
- Asset size warnings (>10 MB)
- Naming convention compliance

**Options:**

- `--fix` - Auto-fix issues (add links, remove orphans)
- `--strict` - Fail on warnings
- `--check-external` - Validate external links too

#### 5. `harnspec merge` - Combine Multi-Doc Spec

Merge all documents into single file for sharing:

```bash
$ harnspec merge specs/20251101/009-sub-spec-files

📦 Merging 009-sub-spec-files...

✓ README.md
✓ TESTING.md
✓ API.md
✓ MIGRATION.md

Created: specs/20251101/009-sub-spec-files-MERGED.md (8.5 KB)
```

Output structure:

```markdown
# 009-sub-spec-files (Merged)

> Merged from multiple documents on 2025-11-01

## Table of Contents
...

## Main Spec
[Content from README.md]

## Testing
[Content from TESTING.md]

## API
[Content from API.md]

## Migration
[Content from MIGRATION.md]
```

**Options:**

- `--output=file.md` - Custom output path
- `--include-assets` - Embed images as base64
- `--pdf` - Generate PDF (requires pandoc)
- `--exclude=DEPRECATED.md` - Skip specific files

### Template System Integration

Extend template system to support multi-file specs:

**New Template Structure:**

```
templates/api-spec/
├── config.json
├── README.md
├── spec-template.md         # Main README.md template
├── API.md                   # API endpoints template
├── SCHEMAS.md               # Data schemas template
├── EXAMPLES.md              # Examples template
└── files/
    └── AGENTS.md
```

**config.json Enhancement:**

```json
{
  "name": "api-spec",
  "description": "API specification with detailed endpoints",
  "subFiles": [
    {
      "name": "API.md",
      "description": "API endpoints and specifications",
      "required": false
    },
    {
      "name": "SCHEMAS.md",
      "description": "Data models and validation",
      "required": false
    },
    {
      "name": "EXAMPLES.md",
      "description": "Request/response examples",
      "required": true
    }
  ]
}
```

### Integration with Existing Commands

**Search Enhancement:**

```bash
# Search across all spec documents
harnspec search "authentication" --all-docs

# Search only in specific sub-document type
harnspec search "test" --in=TESTING.md
```

**Stats Enhancement:**

```bash
$ harnspec stats --verbose

📊 Spec Statistics

Specs: 48 total
Documents: 132 total (2.75 docs/spec avg)
  README.md:    48
  TESTING.md:   23
  API.md:       15
  MIGRATION.md: 12
  Other:        34
```

### Spec Loader Changes

Update `spec-loader.ts` to support sub-files:

```typescript
export interface SpecInfo {
  path: string;
  fullPath: string;
  filePath: string; // Main README.md
  name: string;
  date: string;
  frontmatter: SpecFrontmatter;
  content?: string;
  subFiles?: SubFileInfo[]; // New field
}

export interface SubFileInfo {
  name: string; // e.g., "TESTING.md"
  path: string; // Absolute path
  size: number; // File size in bytes
  type: 'document' | 'asset'; // Type classification
  content?: string; // Optional content
}

// New function to load sub-files
export async function loadSubFiles(
  specDir: string,
  options: { includeContent?: boolean } = {}
): Promise<SubFileInfo[]> {
  // Read all files in spec directory
  // Classify as document (.md) or asset (other)
  // Exclude README.md (main file)
  // Load content if requested
}

// Enhanced loadAllSpecs with sub-files
export async function loadAllSpecs(options: {
  includeArchived?: boolean;
  includeContent?: boolean;
  includeSubFiles?: boolean; // New option
  filter?: SpecFilterOptions;
}): Promise<SpecInfo[]> {
  // ... existing logic
  
  if (options.includeSubFiles) {
    specInfo.subFiles = await loadSubFiles(specDir, {
      includeContent: options.includeContent
    });
  }
}
```

## Use Cases

### Use Case 1: API Specification

**Scenario**: Team is designing a new REST API with 20+ endpoints.

**Structure**:

```
specs/20251101/010-user-api-v2/
├── README.md           # Overview, design decisions, goals
├── ENDPOINTS.md        # Detailed endpoint specifications
├── SCHEMAS.md          # Request/response schemas
├── AUTHENTICATION.md   # Auth strategy and flows
├── EXAMPLES.md         # Example requests/responses
├── MIGRATION.md        # Migration from v1
└── assets/
    └── api-flow.svg    # API flow diagram
```

**Workflow**:

```bash
# Create from template
harnspec create user-api-v2 --template=api-spec

# Work on different aspects
code specs/.../README.md        # High-level design
code specs/.../ENDPOINTS.md     # Endpoint details
code specs/.../AUTHENTICATION.md # Auth design

# Validate structure
harnspec validate specs/.../user-api-v2

# Generate unified TOC
harnspec toc specs/.../user-api-v2 --insert

# Share with stakeholders
harnspec merge specs/.../user-api-v2 --pdf
```

### Use Case 2: Complex Feature with Testing

**Scenario**: Building a payment system with extensive testing requirements.

**Structure**:

```
specs/20251101/011-payment-processing/
├── README.md           # Feature overview and goals
├── ARCHITECTURE.md     # System design
├── TESTING.md          # Test strategy
├── TEST_RESULTS.md     # Test execution results
├── SECURITY.md         # Security considerations
├── ROLLOUT.md          # Phased rollout plan
└── assets/
    ├── flow-diagram.png
    └── test-coverage.png
```

### Use Case 3: Research Spike

**Scenario**: Evaluating multiple approaches for caching strategy.

**Structure**:

```
specs/20251101/012-caching-strategy/
├── README.md           # Research goals and summary
├── REDIS.md            # Redis approach analysis
├── MEMCACHED.md        # Memcached approach analysis
├── IN_MEMORY.md        # In-memory approach analysis
├── COMPARISON.md       # Side-by-side comparison
├── RECOMMENDATION.md   # Final recommendation
└── assets/
    ├── redis-perf.png
    └── memcached-perf.png
```

**Workflow**:

```bash
# Create research spec
harnspec create caching-strategy

# Add sub-documents as you research
harnspec add specs/.../caching-strategy/REDIS.md
harnspec add specs/.../caching-strategy/MEMCACHED.md

# Generate comparison TOC
harnspec toc specs/.../caching-strategy

# Validate completeness
harnspec validate specs/.../caching-strategy
```

## Implementation Plan

### Phase 1: Core Support (v1.0)

- [ ] Update `SpecInfo` type to include `subFiles`
- [ ] Implement `loadSubFiles()` function
- [ ] Add `--include-sub-files` option to `harnspec list`
- [ ] Add `harnspec files <spec>` command
- [ ] Update templates to support sub-file definitions
- [ ] Document naming conventions

### Phase 2: Creation & Management (v1.1)

- [ ] Enhance `harnspec create` with `--files` option
- [ ] Add `harnspec add <spec>/<file>` command
- [ ] Create sub-file templates for common types
- [ ] Add API spec template with sub-files
- [ ] Update documentation

### Phase 3: Validation & Tools (v1.2)

- [ ] Implement `harnspec validate` command
- [ ] Add link checker for cross-references
- [ ] Add orphan file detection
- [ ] Implement `harnspec toc` command
- [ ] Add TOC auto-insertion

### Phase 4: Advanced Features (v2.0)

- [ ] Implement `harnspec merge` command
- [ ] Add PDF export support (via pandoc)
- [ ] Enhance search to include all sub-documents
- [ ] Add stats for sub-file distribution
- [ ] Implement asset optimization warnings

## Testing

### Unit Tests

- [ ] `loadSubFiles()` returns all markdown files except README.md
- [ ] `loadSubFiles()` classifies files correctly (docs vs assets)
- [ ] `loadSubFiles()` includes content when requested
- [ ] Sub-file loading works with archived specs
- [ ] Empty spec directory returns empty sub-files array

### Integration Tests

- [ ] `harnspec files` lists all sub-documents correctly
- [ ] `harnspec files --type=docs` filters to markdown only
- [ ] `harnspec files --tree` shows proper hierarchy
- [ ] `harnspec create --files=X,Y` creates all files
- [ ] `harnspec add` creates properly templated file
- [ ] `harnspec validate` detects broken links
- [ ] `harnspec validate` detects orphaned files
- [ ] `harnspec toc` generates correct TOC
- [ ] `harnspec toc --insert` updates README.md correctly
- [ ] `harnspec merge` combines all documents
- [ ] `harnspec search --all-docs` searches sub-files
- [ ] Templates with sub-files create all documents

### Edge Cases

- [ ] Spec with only README.md (no sub-files)
- [ ] Spec with 50+ sub-files (performance)
- [ ] Sub-file with circular references
- [ ] Non-markdown files in spec directory
- [ ] README.md not explicitly excluded from sub-files
- [ ] Symlinks in spec directory
- [ ] Sub-directories within spec folder

## Non-Goals

- **Not building a wiki**: This is for spec organization, not general documentation
- **Not version control**: Git already handles versioning of all files
- **Not a CMS**: No database, no complex content management
- **Not automatic splitting**: No AI-powered document splitting (user decides structure)
- **Not enforcing structure**: Sub-files remain optional, use when needed

## Design Decisions

### Why Keep README.md as Entry Point?

**Decision**: Always require README.md with frontmatter.

**Rationale**:

- Backwards compatibility with existing specs
- Clear entry point for tools and humans
- Single source of truth for metadata
- Familiar convention (GitHub, GitLab, etc.)
- Frontmatter only in one place (avoid sync issues)

### Why Naming Conventions?

**Decision**: Recommend standard names (TESTING.md, API.md) but don't enforce.

**Rationale**:

- Discoverability: predictable names help navigation
- Tooling: standard names enable smart defaults
- Convention over configuration: reduce decision fatigue
- Flexibility: not enforced, just recommended

**Alternative Considered**: Free-form naming (rejected - too chaotic)

### Why Not Nested Directories?

**Decision**: Keep all documents at spec root level (flat structure).

**Rationale**:

- Simpler to discover and link
- Avoid over-organization
- Assets can live in `assets/` subdirectory
- Nested structure adds complexity without much benefit

**Alternative Considered**: Full directory hierarchy (rejected - violates LeanSpec minimalism)

### Why Not Automatic TOC in README.md?

**Decision**: Make TOC generation opt-in (`harnspec toc --insert`).

**Rationale**:

- Not all specs need TOC (simple ones don't benefit)
- Manual control over when/where TOC appears
- Avoid auto-generated content in version control
- Keep README.md human-authored by default

**Alternative Considered**: Auto-generate TOC on every change (rejected - too automatic)

## Migration Path

### For Existing Specs

Specs with informal sub-files (like `SUMMARY.md`) will continue working. New commands will simply recognize them.

**No breaking changes**:

- Existing specs with only README.md: no change
- Existing specs with extra files: now discoverable via `harnspec files`
- All existing commands continue working

**Gradual adoption**:

1. Start using recommended naming conventions
2. Add links in README.md to sub-documents
3. Use `harnspec validate` to check structure
4. Use `harnspec toc` to auto-generate navigation

## Documentation Updates

- [ ] Update README.md with sub-file examples
- [ ] Add "Organizing Large Specs" guide
- [ ] Update AGENTS.md with sub-file discovery guidance
- [ ] Document naming conventions
- [ ] Add template documentation for multi-file specs
- [ ] Create examples in `examples/` directory

## Related Specs

- `20251101/002-structured-frontmatter` - Frontmatter stays in README.md only
- `20251101/003-pm-visualization-tools` - Stats could show sub-file distribution
- `20251031/002-template-system-redesign` - Templates need sub-file support

## Questions

1. **Should sub-files have their own frontmatter?**
   - No - frontmatter only in README.md (single source of truth)
   - Sub-files can have simple header with link back to main spec

2. **How to handle conflicting content between files?**
   - README.md is authoritative for metadata and high-level info
   - Sub-files provide detailed content
   - Use validation to detect inconsistencies

3. **Should `harnspec search` include sub-files by default?**
   - Yes - search all content by default
   - Add `--main-only` flag to search only README.md

4. **Size limits for sub-files?**
   - No hard limits
   - Warning for assets >10 MB
   - Recommendation: keep docs <100 KB each

5. **Asset management?**
   - Keep in `assets/` subdirectory
   - Reference with relative paths: `![Diagram](assets/flow.svg)`
   - No special handling needed (git handles binary files)

## Success Metrics

- 20% of new specs use sub-files within 2 months
- Zero complaints about backward compatibility
- `harnspec files` command used regularly (top 5 commands)
- Templates with sub-files are popular choices
- Documentation receives positive feedback

## Future Considerations

- **Interactive TOC builder**: CLI wizard to help organize large specs
- **Template gallery**: Community-contributed multi-file templates
- **Link graph visualization**: See connections between documents
- **Content suggestions**: AI-powered recommendations for splitting large README.md
- **Export formats**: HTML, PDF, EPUB for sharing outside git
- **Diff tools**: Compare versions of multi-file specs

---

**Remember**: Sub-files are for complexity management. Most specs should stay simple with just README.md. Only add structure when you feel the pain.
