# CLI Design

Command-line interface for the `harnspec validate` command.

**Note:** This spec originally proposed expanding `harnspec check`, but the implementation created a separate `harnspec validate` command for comprehensive quality validation, while keeping `harnspec check` focused on sequence conflicts.

## Basic Usage

### Current Implementation (v0.2.0)

```bash
# Validate all specs (runs: line count + frontmatter)
harnspec validate

# Validate specific specs
harnspec validate 018
harnspec validate 043 048 018          # Multiple specs

# Custom line limit
harnspec validate --max-lines 500
harnspec validate 018 --max-lines 300

# For sequence conflicts (separate command)
harnspec check
```

### Planned Enhancements (v0.3.0+)

```bash
# Validate specific aspects
harnspec validate --frontmatter        # Only frontmatter validation
harnspec validate --structure          # Only structure validation
harnspec validate --content            # Only content validation
harnspec validate --corruption         # Only corruption detection
harnspec validate --staleness          # Only staleness detection
harnspec validate --sub-specs          # Only sub-spec validation

# Combine validations
harnspec validate --frontmatter --structure
harnspec validate --sub-specs --structure  # Check sub-specs and main structure

# Skip certain checks
harnspec validate --no-staleness       # Skip staleness warnings

# Filter which specs to validate
harnspec validate --status=in-progress
harnspec validate --tag=api
```

## Output Options

```bash
# Output formatting
harnspec validate --format=json        # JSON output for CI
harnspec validate --quiet              # Brief output (errors only)
harnspec validate --verbose            # Detailed output with explanations

# Behavior options
harnspec validate --strict             # Fail on warnings (not just errors)
harnspec validate --fix                # Auto-fix issues where possible
```

## Command Evolution

### Current Implementation (v0.2.0+)

Two separate commands with distinct purposes:

```bash
harnspec check               # Fast sequence conflict detection
harnspec validate            # Comprehensive quality validation
harnspec validate [specs...] # Validate specific specs
```

### Planned Enhancements (v0.3.0+)

Expand `harnspec validate` with additional validation rules:

```bash
harnspec validate --all           # All validation rules
harnspec validate --frontmatter   # Frontmatter validation
harnspec validate --structure     # Structure validation
harnspec validate --corruption    # Corruption detection
```

## Console Output Format

### Current Output (v0.2.0)

**Actual output format:**

```
Validating specs...

Line Count:
  ✓ 018-spec-validation (255 lines)
  ⚠ 051-docs-system-prompt-principles (340 lines - approaching limit)
     → Consider simplification or splitting
  ✗ 048-spec-complexity-analysis (601 lines - exceeds limit!)
     → Spec exceeds 400 lines (601 lines)
     → Consider splitting into sub-specs using spec 012 pattern

Frontmatter:
  ✓ All 25 spec(s) passed

Results: 25 specs validated, 3 error(s), 6 warning(s)
```

**Features:**

- Groups results by validator type (Line Count, Frontmatter, etc.)
- Clear pass/warn/error indicators (✓ ⚠ ✗)
- Actionable suggestions for each issue
- Summary with counts

### Planned Output (v0.3.0+)

```
📋 Validating specs...

Line Count:
  ✓ 10 specs within limits
  ⚠ 1 spec approaching limit (300-400 lines)
  ✗ 1 spec exceeds limit (>400 lines)

Frontmatter:
  ✗ 1 spec has errors:
    - specs/044-spec-relationships-clarity/
      • Missing required field: created
      • Invalid status: "wip"

Structure:
  ✗ 1 spec has errors:
    - specs/044-spec-relationships-clarity/
      • Missing required section: ## Testing

Sub-Specs:
  ⚠ 2 warnings:
    - specs/018-spec-validation/
      ⚠ Sub-spec TESTING.md (421 lines) exceeds 400 line limit
      ⚠ Orphaned sub-spec: DEPRECATED.md (not linked from README.md)
  ✓ All other specs with sub-specs are valid

Corruption:
  ✗ 1 spec corrupted:
    - specs/018-spec-validation/
      • Duplicate section: "Auto-Fix Capability" (lines 245, 320)
      • Malformed code block (line 67)
      • Incomplete JSON (line 156)

Content:
  ⚠ 1 warning:
    - specs/043-official-launch-02/
      ⚠ In progress for 45 days

Results: 8/12 passed, 2 warnings, 4 errors

Note: For sequence conflicts, run `harnspec check`
```

### Quiet Output

```
✗ 2 specs with errors
```

### Verbose Output

```
📋 Validating specs...

Line Count:
  ✓ 10 specs within limits
  
  Checked 12 specs total
  - 10 specs under 300 lines (ideal)
  - 1 spec 300-400 lines (warning zone)
  - 1 spec over 400 lines (should split)

Frontmatter:
  ✗ 1 spec has errors:
  
    specs/044-spec-relationships-clarity/
      • Missing required field: created
        → Fix: Add 'created: YYYY-MM-DD' to frontmatter
      
      • Invalid status: "wip"
        → Valid values: planned, in-progress, complete, archived
        → Fix: Change status to one of the valid values

... (more detailed explanations)
```

## JSON Output Format

For CI/CD integration:

```json
{
  "summary": {
    "total": 12,
    "passed": 10,
    "failed": 2,
    "warnings": 1,
    "checks": {
      "sequences": {"passed": true, "conflicts": 0},
      "frontmatter": {"passed": false, "errors": 2},
      "structure": {"passed": false, "errors": 1},
      "corruption": {"passed": false, "errors": 3},
      "content": {"passed": true, "warnings": 1}
    }
  },
  "results": [
    {
      "path": "specs/018-spec-validation/",
      "valid": false,
      "checks": {
        "sequences": {"passed": true},
        "frontmatter": {"passed": true},
        "structure": {"passed": true},
        "corruption": {
          "passed": false,
          "errors": [
            {
              "type": "duplicate-section",
              "message": "Duplicate section: 'Auto-Fix Capability'",
              "locations": [245, 320],
              "severity": "error",
              "fixable": true
            },
            {
              "type": "malformed-code-block",
              "message": "Code block not properly closed",
              "line": 67,
              "severity": "error",
              "fixable": false
            }
          ]
        }
      }
    }
  ]
}
```

## Exit Codes

- `0` - All checks passed
- `1` - Errors found (any check failed)
- `2` - Warnings found (only in --strict mode)
- `3` - Command error (invalid arguments, etc.)

**Note:** `harnspec check` (sequence conflicts) uses same exit code pattern.

## Auto-Fix Mode

```bash
harnspec validate --fix
```

**What Gets Fixed:**

- Missing frontmatter fields (adds with defaults)
- Date formatting (converts to ISO 8601)
- Duplicate sections (removes duplicates, keeps first)
- Unclosed code blocks (closes them)
- Visual badges (updates from frontmatter)
- Missing sub-spec references in README.md (adds links)

**What Doesn't Get Fixed:**

- Invalid status values (requires decision)
- Empty sections (requires content)
- Broken links (requires investigation)
- Complex corruption (requires judgment)
- Sub-specs exceeding line limits (requires manual splitting)
- Orphaned sub-specs (requires decision to keep or remove)

**Output:**

```
📋 Checking and fixing specs...

Fixed 3 issues:
  ✓ specs/044-spec-relationships-clarity/
    • Added missing field: created = 2025-11-04
    • Formatted date: 2025/11/04 → 2025-11-04
  
  ✓ specs/018-spec-validation/
    • Removed duplicate section: "Auto-Fix Capability"

Could not auto-fix 2 issues:
  ✗ specs/044-spec-relationships-clarity/
    • Invalid status: "wip" - Please use: planned, in-progress, complete, archived

Results: Auto-fixed 3/5 issues
```

## Filtering Specs

```bash
# By status
harnspec validate --status=in-progress
harnspec validate --status=planned,in-progress

# By tag
harnspec validate --tag=api
harnspec validate --tag=quality,validation

# By priority
harnspec validate --priority=high,critical

# By path pattern
harnspec validate specs/2025*
harnspec validate specs/archived/
```

## CI/CD Integration

### GitHub Actions Example

```yaml
- name: Check spec quality
  run: |
    harnspec validate --format=json --strict > validate-results.json
  continue-on-error: true

- name: Comment PR with results
  uses: actions/github-script@v6
  with:
    script: |
      const results = require('./validate-results.json');
      // Post comment with results
```

### Pre-Commit Hook

```bash
#!/bin/sh
# .git/hooks/pre-commit

# Run comprehensive validation
harnspec validate --format=json > /dev/null 2>&1

if [ $? -ne 0 ]; then
  echo "❌ Spec quality checks failed!"
  echo "Run 'harnspec validate' to see details"
  echo "Run 'harnspec validate --fix' to auto-fix issues"
  exit 1
fi

echo "✓ All spec quality checks passed"
```

## Design Decisions

### Why Separate `validate` Command (Implementation Choice)

The original spec proposed expanding `harnspec check`, but the implementation created a separate `harnspec validate` command:

**Rationale:**

1. **Separation of concerns**: Sequence checking is fast/targeted; validation is comprehensive
2. **Performance**: Users can run quick sequence checks without validation overhead
3. **Backwards compatible**: Existing `harnspec check` behavior unchanged
4. **Incremental adoption**: Can add validation rules without affecting check command
5. **Clearer intent**: `validate` explicitly signals quality checking vs. `check` for conflicts

**Trade-offs:**

- Two commands to remember (but both are intuitive)
- More CLI surface area
- Better performance and flexibility

### Flag Design Philosophy

- **Positive flags**: Enable specific validations (`--frontmatter`, `--structure`)
- **Negative flags**: Disable validations (`--no-staleness`)
- **Default**: All available validations when no flags specified
- **Specificity**: Can validate individual specs or filter by status/tags

### Performance Considerations

- Fast by default (< 1s for 100 specs)
- Parallel spec loading
- Incremental checking (only changed specs in auto-check)
- Caching of check results
