---
status: complete
created: 2026-01-20
priority: medium
tags:
- agent-skills
- cli
- onboarding
- init
- automation
depends_on:
- 211-leanspec-as-anthropic-skill
created_at: 2026-01-20T01:34:15.757209028Z
updated_at: 2026-01-21T07:39:40.654895566Z
completed_at: 2026-01-21T07:39:40.654895566Z
transitions:
- status: in-progress
  at: 2026-01-21T06:54:03.593737113Z
- status: complete
  at: 2026-01-21T07:39:40.654895566Z
---

# Agent Skills Auto-Installation in Init Command

## Overview

Enable automated installation of LeanSpec Agent Skills during `harnspec init`, making it effortless for users to set up agent skills support for their AI coding tools.

### Problem

Spec 211 created the `leanspec-sdd` agent skill (plus internal skills for LeanSpec contributors), but users must manually copy it from `.github/skills/` to their project or user-level skills directories. This creates friction in the onboarding experience.

**Note**: Only `leanspec-sdd` is distributed to users. The `leanspec-publishing` and `leanspec-development` skills are internal tools for LeanSpec project contributors.

**Current workflow** (manual):

```bash
harnspec init
# User must then:
cp -r .github/skills/leanspec-sdd ~/.copilot/skills/  # Manual copy
```

**Desired workflow** (automated):

```bash
harnspec init
# Detects installed AI tools (Copilot, Claude, Cursor, etc.)
# Prompts: "Install LeanSpec skills to .github/skills/? [Y/n]"
# Automatically copies skills to appropriate locations
```

### What This Spec Delivers

Integrate agent skills installation into the `harnspec init` command:

1. **Detect AI coding tools** installed (reuse spec 126 logic)
2. **Smart defaults** - Suggest appropriate skills folders per tool
3. **Interactive prompts** - Let users choose where to install
4. **Non-interactive mode** - CLI flags for automation
5. **Multi-tool support** - Install to multiple locations simultaneously
6. **Validation** - Verify successful installation

### Benefits

- **Zero-friction onboarding** - Skills installed automatically during init
- **Tool-aware** - Detects user's AI tools and suggests right locations
- **Flexible** - Supports project-level, user-level, or both
- **Automation-friendly** - CLI flags for CI/CD and scripts

## Design

### 1. Skills Folder Detection

Detect existing skills infrastructure and recommend installation locations.

**Project-Level Locations** (check in current directory):

```
.github/skills/      # GitHub Copilot
.claude/skills/      # Claude
.cursor/skills/      # Cursor
.codex/skills/       # Codex CLI
.gemini/skills/      # Gemini CLI
.vscode/skills/      # VS Code
.skills/             # Generic fallback
```

**User-Level Locations** (check in home directory):

```
~/.copilot/skills/   # GitHub Copilot global
~/.claude/skills/    # Claude global
~/.cursor/skills/    # Cursor global
~/.codex/skills/     # Codex CLI global
~/.gemini/skills/    # Gemini CLI global
~/.vscode/skills/    # VS Code global
~/.skills/           # Generic global fallback
```

**Detection Strategy**:

1. Check if any project-level skills folders exist
2. Check if any user-level skills folders exist
3. Leverage spec 126 AI tool detection to suggest defaults
4. Build list of recommended installation targets

### 2. Installation Flow

**Scenario A: No Existing Skills Folders** (fresh project)

```
$ harnspec init

Welcome to LeanSpec! 🚀

🔍 Detected AI tools: GitHub Copilot

? Install LeanSpec Agent Skills? (Recommended)
  ❯ Yes - Project-level (.github/skills/ for this project)
    Yes - User-level (~/.copilot/skills/ for all projects)
    Skip for now

Installing skill to .github/skills/...
  ✓ leanspec-sdd installed

💡 Restart your AI tool to discover the new skill
```

**Scenario B: Existing Project Skills Folder**

```
$ harnspec init

🔍 Found existing skills folder: .github/skills/

? Install LeanSpec skills to .github/skills/? (Recommended)
  ❯ Yes
    No

Installing to .github/skills/...
  ✓ leanspec-sdd installed
```

**Scenario C: Multiple Tools Detected**

```
$ harnspec init

🔍 Detected AI tools:
   • GitHub Copilot (VS Code extension)
   • Claude Desktop (~/.claude directory)

? Where should we install LeanSpec skills? (Select all that apply)
  ◉ .github/skills/ (project, for GitHub Copilot)
  ◉ .claude/skills/ (project, for Claude)
  ◯ ~/.copilot/skills/ (user-level, all projects)
  ◯ ~/.claude/skills/ (user-level, all projects)
  ◯ Skip installation

Installing to 2 locations...
  ✓ .github/skills/leanspec-sdd/ installed
  ✓ .claude/skills/leanspec-sdd/ installed
```

### 3. Installation Implementation

**Copy Strategy** (recommended for v1):

- Copy skill folders from bundled templates
- Each location gets independent copy
- Simple, works everywhere (Windows/macOS/Linux)
- Users can customize per-project

**Source Location**:
The `leanspec-sdd` skill is bundled with harnspec installation:

```
/path/to/harnspec-install/.github/skills/
└── leanspec-sdd/
    ├── SKILL.md
    ├── references/
    │   ├── WORKFLOW.md
    │   ├── BEST-PRACTICES.md
    │   ├── EXAMPLES.md
    │   └── COMMANDS.md
    └── scripts/
        └── validate-spec.sh
```

**Note**: The `leanspec-publishing` and `leanspec-development` skills remain in the LeanSpec repo but are not distributed to users.

**Installation Process**:

1. Detect or prompt for target location(s)
2. Create target directory if needed (e.g., `.github/skills/`)
3. Copy `leanspec-sdd/` folder recursively
4. Verify files were written successfully
5. Show success message with next steps

### 4. CLI Flags

Support non-interactive mode for automation:

```bash
# Interactive (default)
harnspec init

# Auto-install to project-level (detect tool)
harnspec init --skill

# Install to specific tool locations
harnspec init --skill-github       # .github/skills/
harnspec init --skill-claude       # .claude/skills/
harnspec init --skill-cursor       # .cursor/skills/
harnspec init --skill-codex        # .codex/skills/
harnspec init --skill-gemini       # .gemini/skills/
harnspec init --skill-vscode       # .vscode/skills/

# Install to user-level
harnspec init --skill-user         # Tool-specific user dir

# Install to multiple locations
harnspec init --skill-github --skill-claude

# Skip skill installation
harnspec init --no-skill

# Silent mode (yes to all)
harnspec init -y --skill
```

### 5. Integration with AI Tool Detection (Spec 126)

Reuse existing AI tool detection to provide smart defaults:

**Tool Detection → Skills Folder Mapping**:

```rust
// Pseudo-code
let detected_tools = detect_ai_tools(); // From spec 126

if detected_tools.contains("github_copilot") {
    recommend(".github/skills/");
}
if detected_tools.contains("claude") {
    recommend(".claude/skills/");
}
if detected_tools.contains("cursor") {
    recommend(".cursor/skills/");
}
// ... etc for other tools
```

**Benefits**:

- Zero-config for most users
- Tool-specific recommendations
- Intelligent defaults
- User can override

## Plan

### Phase 1: Detection Logic (2-3 days)

- [ ] Create `detect_skills_locations()` function
  - [ ] Check project-level folders (.github/skills/, .claude/skills/, etc.)
  - [ ] Check user-level folders (~/.copilot/skills/, etc.)
  - [ ] Return list of existing and potential locations
- [ ] Integrate with spec 126 AI tool detection
  - [ ] Map detected tools to recommended skills folders
  - [ ] Build prioritized list of installation targets
- [ ] Add tests for detection logic
  - [ ] Test with various folder combinations
  - [ ] Test with different tool installations

### Phase 2: Installation Logic (2-3 days)

- [ ] Create `install_skills()` function
  - [ ] Copy skill folders recursively
  - [ ] Handle multiple target locations
  - [ ] Create parent directories as needed
  - [ ] Validate successful installation
- [ ] Bundle skills with CLI distribution
  - [ ] Ensure skills are in package/binary
  - [ ] Determine bundled skills location at runtime
  - [ ] Handle cross-platform paths
- [ ] Add tests for installation
  - [ ] Test copy operation
  - [ ] Test permission handling
  - [ ] Test error cases (no space, no permissions, etc.)

### Phase 3: Init Command Integration (3-4 days)

- [ ] Update `init.rs` (or TypeScript equivalent)
  - [ ] Add skills installation prompts
  - [ ] Add multi-select for multiple locations
  - [ ] Add success messages with next steps
- [ ] Add CLI flags
  - [ ] `--skill` (auto-install to detected tool's default)
  - [ ] `--skill-github`, `--skill-claude`, etc. (tool-specific)
  - [ ] `--skill-user` (user-level installation)
  - [ ] `--no-skill` (skip installation)
- [ ] Handle edge cases
  - [ ] Skills already installed → skip or update?
  - [ ] No AI tools detected → offer generic location
  - [ ] User declines → show manual instructions
- [ ] Update init tests
  - [ ] Test each installation scenario
  - [ ] Test flag combinations
  - [ ] Test error handling

### Phase 4: Cross-Platform Testing (2-3 days)

- [ ] Test on macOS
  - [ ] Interactive mode
  - [ ] All CLI flags
  - [ ] User-level installation (~/ paths)
- [ ] Test on Linux
  - [ ] Same as macOS
  - [ ] Permission scenarios
- [ ] Test on Windows
  - [ ] Path handling (forward vs backslash)
  - [ ] Home directory detection
  - [ ] Copy operations
- [ ] Test in CI/CD
  - [ ] Non-interactive mode
  - [ ] Automation scenarios

### Phase 5: Documentation & Polish (1-2 days)

- [ ] Update CLI documentation
  - [ ] Document new flags
  - [ ] Add examples
  - [ ] Update help text
- [ ] Update onboarding docs
  - [ ] Show automated flow
  - [ ] Explain manual fallback
  - [ ] Document tool-specific locations
- [ ] Update README/quickstart
  - [ ] Highlight skills auto-installation
  - [ ] Show expected output
- [ ] Add troubleshooting guide
  - [ ] Common issues
  - [ ] Manual installation steps
  - [ ] Permission problems

## Test

- [ ] **Detection works correctly**
  - Detects existing skills folders (project and user-level)
  - Correctly identifies AI tools from spec 126
  - Maps tools to appropriate skills folder recommendations
- [ ] **Installation succeeds**
  - `leanspec-sdd` copied to target location(s)
  - All files present (SKILL.md, references/, scripts/)
  - Files readable by AI tools
  - Directory structure preserved
- [ ] **Interactive prompts work**
  - User can select single or multiple locations
  - Can skip installation
  - Shows appropriate defaults
- [ ] **CLI flags work**
  - Each flag installs to correct location
  - Multiple flags work together
  - `--no-skill` skips installation
  - Works with `-y` (non-interactive)
- [ ] **Cross-platform compatibility**
  - Works on macOS, Linux, Windows
  - Path handling correct for each platform
  - Permissions preserved
- [ ] **Error handling**
  - Graceful failure if no permissions
  - Clear error messages
  - Doesn't break init flow if skills fail
- [ ] **Integration with existing init**
  - Doesn't break existing init functionality
  - Works with other init flags
  - Proper ordering with AGENTS.md, MCP config, etc.

## Notes

### Relationship to Other Specs

**Depends on**:

- **211-leanspec-as-anthropic-skill** (complete) - The skills to install
- **126-ai-tool-auto-detection** (complete) - Tool detection logic

**Coordinates with**:

- **222-cross-tool-agent-skills-compatibility** (planned) - Advanced compatibility, symlink strategies, sync mechanisms
- **127-init-agents-merge-automation** (complete) - AGENTS.md merging
- **145-mcp-config-auto-setup** (complete) - MCP configuration

**Key difference from spec 222**:

- This spec: Basic copy-based installation in init
- Spec 222: Advanced cross-tool compatibility, version sync, platform-specific optimizations

### Implementation Location

**Rust CLI** (`rust/leanspec-cli/src/commands/init.rs`):

- Primary implementation target
- More mature init command
- Better cross-platform support
- Direct filesystem access

**TypeScript CLI** (if needed):

- May need updates for consistency
- Less priority if Rust is canonical

### Design Decisions

**Why copy instead of symlink?**

- Works on Windows without admin privileges
- Users can customize per-project
- No breakage if harnspec moved/uninstalled
- Simpler implementation
- Spec 222 can add symlink option later

**Why project-level default?**

- Git-tracked and shared with team
- Version-controlled methodology
- Most teams want consistent approach
- User-level still available as option

**Why bundle skills with CLI?**

- No internet required during init
- Faster installation
- Version-locked (skills match CLI version)
- Offline-friendly

### Future Enhancements (Spec 222)

These are explicitly out of scope for this spec:

- ❌ Symlink-based installation
- ❌ Skills version management/sync
- ❌ Update existing skills command
- ❌ Tool-specific skill variants
- ❌ Skills marketplace integration
- ❌ Advanced platform-specific optimizations

Keep it simple for v1 - ship the basics that provide immediate value.
