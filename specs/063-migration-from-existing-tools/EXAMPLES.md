# Migration Examples: Metadata & Folder Organization

The key migration challenges vary by source tool:

1. **Metadata/Frontmatter** (ALL sources) - Use `harnspec backfill`
2. **Folder organization** (OpenSpec, ADR) - Reorganize into `specs/###-name/`
3. **spec-kit** - Already compatible! Just needs frontmatter

---

## Example 1: spec-kit → LeanSpec (Easiest Migration)

### Source Structure (spec-kit)

**Source path**: `.specify/specs/` (not `specs/`!)

```
.specify/
└── specs/
    ├── 001-task-management/
    │   ├── spec.md               # Feature specification
    │   ├── plan.md               # Implementation plan
    │   ├── tasks.md              # Task list
    │   └── contracts/
    │       └── api.yml
    ├── 002-user-authentication/
    │   ├── spec.md
    │   └── plan.md
    └── 003-notifications/
        └── spec.md
```

### Target Structure (LeanSpec)

```
specs/
├── 001-task-management/      # ✅ Already compatible!
│   ├── README.md             # Rename: spec.md → README.md
│   ├── plan.md               # ✅ Keep as-is (sub-spec)
│   ├── tasks.md              # ✅ Keep as-is (sub-spec)
│   └── contracts/            # ✅ Keep as-is
│       └── api.yml
├── 002-user-authentication/  # ✅ Already compatible!
│   ├── README.md
│   └── plan.md
└── 003-notifications/        # ✅ Already compatible!
    └── README.md
```

### Migration Process

**Folder reorganization**: Move from `.specify/specs/` to `specs/` and rename spec.md → README.md

```bash
# Move and rename in one go
mv .specify/specs specs/
find specs -name 'spec.md' -execdir mv {} README.md \;
```

**Metadata generation** (PRIMARY CHALLENGE): Use `harnspec backfill`

```bash
# Generate frontmatter from git history
harnspec backfill --assignee --all

# This extracts:
# - created_at: from first git commit
# - updated_at: from last git commit
# - completed_at: from status change to complete
# - assignee: from git author
# - status: inferred from git/content (defaults to 'planned')
# - priority: defaults to 'medium' (can manually adjust after)
# - tags: extracted from folder name or defaults (can manually adjust after)
```

**Result**: Each spec now has frontmatter:

```yaml
---
status: complete
created_at: '2024-03-15T10:23:45Z'
updated_at: '2024-11-08T14:30:12Z'
completed_at: '2024-03-20T16:45:00Z'
assignee: Alice Chen
priority: high
tags:
  - product
  - mvp
---
```

**Key Point**: spec-kit already has the right folder structure! Migration is mostly about metadata.

### Source Structure (OpenSpec)

```
openspec/
├── specs/              # Current state
│   ├── auth/spec.md
│   ├── api-gateway/spec.md
│   └── user-management/spec.md
└── changes/archive/    # Completed changes
    └── 2024-11-15-oauth-integration/
```

### Target Structure (LeanSpec)

```
specs/
├── 001-user-authentication/README.md
├── 002-api-gateway/README.md
└── 003-user-management/README.md
```

### Migration Process

**Folder reorganization**: Merge specs/ + changes/archive/

```bash
# Copy specs, merge archived changes, renumber, rename spec.md → README.md
cp -r openspec/specs/* specs/
mv specs/auth specs/001-user-authentication
mv specs/api-gateway specs/002-api-gateway
find specs -name 'spec.md' -execdir mv {} README.md \;
```

**Metadata**: `harnspec backfill --assignee --all`
---

## Example 2: OpenSpec → LeanSpec (Moderate Complexity)

### Source Structure (spec-kit)

```
specs/
├── 001-task-management/
│   ├── spec.md               # Feature specification
│   ├── plan.md               # Implementation plan
│   ├── tasks.md              # Task list
│   ├── research.md           # Technical research
│   ├── data-model.md         # Data models
│   └── contracts/            # API contracts
│       ├── tasks-api.yml
│       └── projects-api.yml
├── 002-user-authentication/
│   ├── spec.md
│   ├── plan.md
│   └── tasks.md
└── 003-notifications/
    ├── spec.md
    └── plan.md
```

### Target Structure (LeanSpec)

```
specs/
├── 001-task-management/
│   ├── README.md             # Main spec (from spec.md)
│   ├── IMPLEMENTATION.md     # Optional: from plan.md + tasks.md
│   ├── DESIGN.md             # Optional: from data-model.md
│   └── contracts/            # Preserved as-is
│       ├── tasks-api.yml
│       └── projects-api.yml
├── 002-user-authentication/
│   ├── README.md
│   └── IMPLEMENTATION.md
└── 003-notifications/
    └── README.md
```

### Migration Process

**AI Analysis:**

- spec-kit already uses sequential numbering—preserve it
- Multiple files per feature—decide: merge or keep as sub-specs
- Contracts folder—preserve structure
- Already has feature branches—extract metadata

**AI Execution:**

```bash
# Option 1: Keep sub-specs (complex features)
harnspec create task-management
# AI copies spec.md → README.md
# AI copies plan.md + tasks.md → IMPLEMENTATION.md (merged)
# AI copies data-model.md → DESIGN.md
# AI preserves contracts/ folder
harnspec update task-management --status complete --tags product,mvp

# Option 2: Single file (simple features)
harnspec create user-authentication
# AI merges spec.md + plan.md into single README.md
harnspec update user-authentication --status complete --tags auth,security

# Option 3: Minimal (very simple features)
harnspec create notifications
# AI copies spec.md → README.md only
harnspec update notifications --status in-progress --tags product
```

**Key Points:**

- Numbering preserved (001 → 001)
- Multi-file decision: merge vs sub-specs depends on complexity
- Content stays mostly intact
- Frontmatter added for metadata

---

## Example 3: ADR Collection → LeanSpec

### Source Structure (ADR)

### Source Structure (ADR)

```
docs/
└── adr/
    ├── 0001-use-microservices.md
    ├── 0042-event-sourcing-audit.md
    ├── 0105-graphql-api.md
    └── 0203-kubernetes-deployment.md
```

### Target Structure (LeanSpec)

```
specs/
├── 001-use-microservices/
│   └── README.md
├── 002-event-sourcing-audit/
│   └── README.md
├── 003-graphql-api/
│   └── README.md
└── 004-kubernetes-deployment/
    └── README.md
```

### Migration Process

**Folder reorganization**: Flat → hierarchy with renumbering

```bash
# Create folder for each ADR, renumber sequentially
mkdir -p specs/001-use-microservices
mv docs/adr/0001-use-microservices.md specs/001-use-microservices/README.md

mkdir -p specs/002-event-sourcing-audit
mv docs/adr/0042-event-sourcing-audit.md specs/002-event-sourcing-audit/README.md
# ... repeat for all ADRs
```

**Metadata**: `harnspec backfill --assignee --all`

---

## Summary: Migration Complexity by Source

### spec-kit (Easiest) ✅

- **Folder structure**: Already compatible! Just rename spec.md → README.md
- **Challenge**: Metadata/frontmatter only
- **Solution**: `harnspec backfill`
- **Time**: < 5 minutes for 20 specs

### OpenSpec (Moderate) ⚠️

- **Folder structure**: Merge specs/ + changes/archive/ directories
- **Challenge**: Folder merge + metadata
- **Solution**: Manual merge + `harnspec backfill`
- **Time**: 15-30 minutes for 20 specs

### ADR/RFC (Complex) 🔴

- **Folder structure**: Flat files → folder hierarchy
- **Challenge**: Complete reorganization + metadata
- **Solution**: Reorganize + `harnspec backfill`
- **Time**: 30-60 minutes for 20 specs

---

## Real Migration Work: Metadata Is the Challenge

### What Actually Needs Migration

1. **Frontmatter (ALL sources - PRIMARY CHALLENGE)**:
   - Extract timestamps from git: `harnspec backfill`
   - Infer status from content/history
   - Set priority (defaults to 'medium')
   - Extract/create tags
   - Get assignee from git author

2. **Folder structure (OpenSpec, ADR only)**:
   - OpenSpec: Merge two directories
   - ADR: Flat → hierarchy
   - spec-kit: Already compatible!

3. **Content (NEVER changes)**:
   - LeanSpec is flexible about content format
   - Keep existing writing style
   - No format conversion needed

### The `harnspec backfill` Command

This is the key tool for migration:

```bash
# Basic: Extract timestamps from git history
harnspec backfill

# With assignee from git author
harnspec backfill --assignee

# Full metadata extraction
harnspec backfill --all

# Preview before applying
harnspec backfill --dry-run
```

**What it extracts from git**:

- `created_at` - First commit timestamp
- `updated_at` - Last commit timestamp
- `completed_at` - When status changed to 'complete'
- `assignee` - First commit author (with `--assignee`)
- `transitions` - Full status change history (with `--transitions`)

**What you set manually after**:

- `priority` - Defaults to 'medium', adjust with `harnspec update --priority`
- `tags` - Defaults from folder names, adjust with `harnspec update --tags`
- `status` - Inferred from content/history, adjust if needed

See [spec 047-git-backfill-timestamps](../047-git-backfill-timestamps/) for complete `backfill` documentation.

---

## System Prompts: AGENTS.md Migration

Don't forget to migrate AI guidance files:

```
# Source tools often have:
openspec/AGENTS.md
.cursorrules
.github/copilot-instructions.md

# LeanSpec uses:
AGENTS.md (in project root)
```

**Migration strategy**:

1. Review existing AI guidance from source tool
2. Preserve project-specific conventions
3. Merge with LeanSpec AGENTS.md template
4. Update commands (openspec → harnspec)
5. Keep team workflows intact

This ensures AI agents maintain continuity during transition.
