---
status: archived
created: 2025-11-13
priority: high
tags:
- process
- maintenance
- workflow
- best-practices
created_at: 2025-11-13T13:08:46.014Z
updated_at: 2026-01-16T06:55:02.832213Z
transitions:
- status: archived
  at: 2026-01-16T06:55:02.832213Z
---

# archiving-strategy

> **Status**: 🗓️ Planned · **Priority**: High · **Created**: 2025-11-13 · **Tags**: process, maintenance, workflow, best-practices

**Project**: harnspec  
**Team**: Core Development

## Overview

**Problem**: No clear strategy for when to archive completed specs, leading to:

- Bulk archiving attempts that catch incomplete work (e.g., spec 073 Phase 2 not done)
- Uncertainty about which completed specs should stay active vs archived
- Risk of archiving specs that are still referenced or contain living documentation
- Cluttered active workspace with many completed specs (29 completed specs currently)

**Current State**:

- We have `harnspec archive` command that moves specs to `archived/` directory
- Specs marked "complete" but unclear when they should be archived
- No documented criteria or workflow for archiving decisions
- Dependencies (`depends_on`, `related`) might reference archived specs

**Goal**: Define clear archiving strategy with:

1. **Criteria**: When a completed spec should be archived
2. **Process**: How to safely archive specs
3. **Exceptions**: When to keep completed specs active
4. **Tooling**: Commands/checks to help with archiving decisions

## Design

### Archiving Criteria

**Archive a completed spec when ALL of:**

1. ✅ Status is "complete"
2. ✅ Work is fully implemented (no pending phases/tasks)
3. ✅ No longer needs frequent reference
4. ✅ Not "living documentation" (e.g., philosophy/foundation docs)
5. ✅ Age > 30 days since completion (optional cooling-off period)

**Keep completed specs active if ANY of:**

- 🔁 Referenced frequently for context
- 📚 Contains living documentation (philosophy, core concepts)
- 🔥 Completed within last 30 days (recent work)
- 🔗 Multiple active specs depend on it
- 🎯 Part of current release cycle

### Spec Categories

**Safe to Archive (Historical):**

- Old launch milestones (v0.1, v0.2 launches)
- One-time refactors/migrations (fully complete)
- Completed documentation improvements (not living docs)
- Infrastructure changes (fully implemented)

**Keep Active (Reference):**

- Foundation specs (048, 049, 066 - first principles)
- Core feature documentation (if frequently referenced)
- Current release work (v0.3.0 related)
- Recent completions (< 30 days)

**Review Carefully:**

- Multi-phase specs (ensure all phases done)
- Specs with dependencies (check if dependents need them)
- Template/tooling specs (might be referenced)

### Archiving Workflow

**Pre-Archive Checklist:**

```bash
# 1. Verify spec is truly complete
harnspec view <spec>  # Check plan/tasks

# 2. Check dependencies
harnspec deps <spec>  # See what depends on it

# 3. Check age
harnspec view <spec> --json | jq '.completed_at'

# 4. Search for references in active specs
harnspec search "<spec-name>"
```

**Archive Command:**

```bash
harnspec archive <spec>
```

**Batch Archiving (with verification):**

```bash
# List candidates (completed > 30 days)
harnspec list --status complete

# Review each before archiving
# Archive individually after verification
```

### Tooling Enhancements

**Potential CLI improvements:**

1. `harnspec archive --candidates` - List archiving candidates with reasons
2. `harnspec archive --check <spec>` - Dry-run with safety checks
3. `harnspec deps <spec> --check-archived` - Warn if dependencies archived
4. Filter by completion age: `harnspec list --completed-before 2025-10-01`

**Safety Features:**

- Warn if archiving spec with active dependents
- Show references in active specs before archiving
- Dry-run mode to preview archive decision

## Plan

### Phase 1: Document Strategy

- [ ] Define archiving criteria (in this spec)
- [ ] Document workflow in AGENTS.md
- [ ] Create archiving best practices guide
- [ ] Add to official documentation

### Phase 2: Tooling Support

- [ ] Add `--completed-before <date>` filter to `list` command
- [ ] Implement `harnspec archive --candidates` (suggests specs to archive)
- [ ] Add safety checks to `archive` command:
  - Warn if active specs depend on it
  - Show completion age
  - Check for incomplete tasks
- [ ] Add `--check` dry-run flag to `archive` command

### Phase 3: Clean Up Current Workspace

- [ ] Review 29 completed specs with new criteria
- [ ] Archive truly complete, historical specs
- [ ] Keep foundation/reference specs active
- [ ] Document decisions for future reference

### Phase 4: Ongoing Maintenance

- [ ] Add archiving to monthly/quarterly maintenance checklist
- [ ] Monitor completion rate and archive rate
- [ ] Refine criteria based on usage patterns

## Test

### Validation Criteria

- [ ] Archiving criteria clear and actionable
- [ ] Workflow documented and easy to follow
- [ ] Safety checks prevent accidental archiving of needed specs
- [ ] CLI commands help identify candidates automatically

### Test Scenarios

- [ ] Archive old completed spec (> 30 days, no dependents)
- [ ] Attempt to archive recently completed spec (< 30 days) - should warn
- [ ] Archive spec with active dependents - should show warning
- [ ] List archiving candidates - should match manual review
- [ ] Archive spec with incomplete phases - should detect and warn

### Success Metrics

- [ ] Zero accidental archives of needed specs
- [ ] < 5 minutes to decide if spec should be archived
- [ ] Clear rationale documented for each archiving decision
- [ ] Active workspace stays < 20 specs (excluding archived)

## Notes

### Real-World Example: Nov 13, 2025 Bulk Archive Attempt

**What Happened:**

- Attempted to archive all 29 completed specs at once
- Reverted due to concerns (spec 073 Phase 2 incomplete)
- Realized we needed clear archiving criteria

**Lessons Learned:**

1. Bulk archiving is risky without verification
2. "Complete" status doesn't always mean "fully done"
3. Some specs are reference documentation (shouldn't archive)
4. Need to check dependencies before archiving

**Specs We're Uncertain About:**

- 073: Phase 1 done, Phase 2 not started (split into separate spec?)
- 048, 049, 066: Foundation specs, referenced often (keep active?)
- 067, 069, 075: Recent core features (keep for context?)
- 043: Old launch milestone (safe to archive?)

### Dependencies and Archived Specs

**Question**: Is it OK for active specs to depend on archived specs?

**Answer**: Generally yes, since `depends_on` means "required work before starting":

- If dependency is complete and archived, that's expected
- `related` links can also point to archived specs (for context)
- The archive doesn't delete specs, just moves them

**However**, we should warn if:

- Archiving a spec that many active specs depend on (heavy reference usage)
- The spec contains living documentation that's frequently consulted

### Alternative Approaches Considered

**Auto-Archive After N Days:**

- ❌ Too aggressive, doesn't account for reference value
- ❌ Philosophy/foundation specs should never auto-archive
- ✅ Could use as suggestion/reminder, not automatic

**Status: "archived" vs Directory Move:**

- Current: Move to `archived/` directory
- Alternative: Keep in place, set `status: archived`
- ✅ Directory move is clearer, easier to browse
- ✅ Status can be "complete" (in archived/)

**Archive Levels (Hot/Cold):**

- Hot: Specs completed in last 90 days (keep active)
- Warm: Specs 90-180 days old (review for archiving)
- Cold: Specs > 180 days old (archive unless exception)
- 🤔 Adds complexity, may not be needed for current scale

### Open Questions

1. Should we split multi-phase specs when only Phase 1 is done?
2. How to handle "living documentation" specs (philosophy, guides)?
3. Should `harnspec board` show archived specs in a separate column?
4. Archive specs individually or in batches (monthly cleanup)?
5. Should we add `archived_at` timestamp to metadata?

### Related Specs

- 059-programmatic-spec-management: Programmatic access to specs (could help with archiving automation)
- 065-v03-planning: Current release planning (determines what's "current" vs historical)
