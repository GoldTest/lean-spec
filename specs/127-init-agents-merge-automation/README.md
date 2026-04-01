---
status: complete
created: '2025-11-27'
tags:
  - init
  - dx
  - ai-agents
  - onboarding
  - automation
priority: high
created_at: '2025-11-27T03:18:24.101Z'
updated_at: '2025-12-04T06:46:29.580Z'
transitions:
  - status: in-progress
    at: '2025-11-27T03:19:29.290Z'
  - status: complete
    at: '2025-11-27T05:56:46.920Z'
completed_at: '2025-11-27T05:56:46.920Z'
completed: '2025-11-27'
depends_on:
  - 126-ai-tool-auto-detection
---

# Automated AGENTS.md Merge with AI CLI Detection

> **Status**: ✅ Complete · **Priority**: High · **Created**: 2025-11-27 · **Tags**: init, dx, ai-agents, onboarding, automation

**Project**: harnspec  
**Team**: Core Development

## Overview

### Problem

Current `harnspec init` workflow with existing `AGENTS.md` files is too manual:

1. **Manual Merge Execution**: User must manually invoke AI tool with the merge prompt:

   ```bash
   copilot -p "follow .harnspec/MERGE-AGENTS-PROMPT.md to edit AGENTS.md" --allow-all-tools
   ```

2. **No AI CLI Auto-Detection**: We already detect AI tools in spec 126, but don't leverage this to **automatically execute** the merge

3. **Extra Step After Init**: User sees instructions but has to manually copy/paste or run a separate command

### Current Flow (Pain Points)

```
harnspec init
  ↓
Found existing AGENTS.md
  ↓
Choose: "AI-Assisted Merge (recommended)"
  ↓
✓ Created AI consolidation prompt
→ .harnspec/MERGE-AGENTS-PROMPT.md
  ↓
📝 Next steps:
  1. Open .harnspec/MERGE-AGENTS-PROMPT.md  ← Manual
  2. Send it to your AI coding assistant      ← Manual
  3. Let AI create the consolidated AGENTS.md ← Manual
  4. Review and commit the result             ← Manual
```

### Proposed Flow

```
harnspec init
  ↓
Found existing AGENTS.md
  ↓
Choose: "AI-Assisted Merge (recommended)"
  ↓
🔍 Detected: copilot CLI installed
  ↓
? Auto-merge with copilot? (Y/n)
  ↓ [Yes]
Running: copilot -p "..." --allow-all-tools
  ↓
✓ AGENTS.md merged successfully!
  Review changes: git diff AGENTS.md
```

### Goal

Make AGENTS.md merging a **single-step operation** by:

1. Auto-detecting installed AI CLI tools (reusing spec 126 detection)
2. Offering to automatically execute the merge using detected tool
3. Falling back to manual instructions only when no CLI is available

## Design

### AI CLI Tool Registry

Extend `AI_TOOL_CONFIGS` with CLI execution capabilities:

```typescript
interface AIToolConfig {
  // ... existing fields from spec 126
  cli?: {
    command: string;           // Primary CLI command
    promptFlag: string;        // Flag for inline prompts (e.g., '-p')
    allowToolsFlag?: string;   // Flag to enable tool use (e.g., '--allow-all-tools')
    filePromptFlag?: string;   // Flag for file-based prompts (e.g., '-f')
  };
}
```

### Supported AI CLIs

| Tool | CLI Command | Prompt Flag | Tools Flag | Status |
|------|-------------|-------------|------------|--------|
| GitHub Copilot | `copilot` | `-p` | `--allow-all-tools` | ✅ Verified |
| Claude Code | `claude` | `--prompt` | `--allow-all-tools` | ⚠️ Verify syntax |
| Gemini CLI | `gemini` | TBD | TBD | ⚠️ Need to verify |
| Aider | `aider` | `--message` | N/A | ⚠️ Different model |
| Codex CLI | `codex` | TBD | TBD | ⚠️ Need to verify |

### Execution Strategy

```typescript
interface MergeExecution {
  tool: AIToolKey;
  command: string;           // Full command to execute
  prompt: string;            // The merge prompt content
  workingDir: string;        // Project directory
}

async function executeMergeWithAI(
  cwd: string,
  promptPath: string,
  tool: AIToolKey
): Promise<{ success: boolean; output?: string; error?: string }>;
```

### User Flow Options

**Option A: Direct Execution (Recommended)**

- Detect CLI → Ask to auto-execute → Run merge → Show result

**Option B: Command Suggestion**  

- Detect CLI → Show ready-to-run command → User copies/executes

**Option C: Hybrid**

- Detect CLI → Show command → Ask "Run now? (Y/n)"

**Decision**: Use **Option C (Hybrid)** - Shows command for transparency, asks permission before executing

### Security Considerations

1. **Explicit Consent**: Always ask before executing AI commands
2. **Show Command**: Display exact command before running
3. **Tool Flags**: Use appropriate safety flags per tool
4. **Working Directory**: Execute in project root only
5. **Output Capture**: Show AI output for user review

### Error Handling

```typescript
type MergeResult = 
  | { status: 'success'; message: string }
  | { status: 'declined'; message: string }      // User said no
  | { status: 'no-cli'; message: string }        // No CLI detected
  | { status: 'error'; error: string }           // Execution failed
  | { status: 'timeout'; message: string };      // AI took too long
```

### Updated Init Flow

```typescript
async function handleExistingFiles(
  action: 'merge-ai' | 'merge-append' | 'overwrite' | 'skip',
  existingFiles: string[],
  templateDir: string,
  cwd: string,
  variables: Record<string, string>,
  options?: { autoMerge?: boolean; skipPrompts?: boolean }
): Promise<void> {
  
  if (action === 'merge-ai' && file === 'AGENTS.md') {
    // 1. Create consolidation prompt (existing behavior)
    await createMergePrompt(existing, template, promptPath);
    
    // 2. NEW: Detect AI CLI and offer auto-merge
    const detectedTools = await detectInstalledAITools();
    const cliCapableTools = detectedTools.filter(t => 
      t.detected && AI_TOOL_CONFIGS[t.tool].cli
    );
    
    if (cliCapableTools.length > 0) {
      const tool = cliCapableTools[0]; // Use first detected tool
      const config = AI_TOOL_CONFIGS[tool.tool];
      const command = buildMergeCommand(config.cli!, promptPath);
      
      console.log(chalk.cyan(`\n🔍 Detected: ${config.description}`));
      console.log(chalk.gray(`   Command: ${command}`));
      
      if (!options?.skipPrompts) {
        const autoMerge = await confirm({
          message: 'Run merge automatically?',
          default: true,
        });
        
        if (autoMerge) {
          const result = await executeMergeWithAI(cwd, promptPath, tool.tool);
          if (result.success) {
            console.log(chalk.green('✓ AGENTS.md merged successfully!'));
            console.log(chalk.gray('  Review: git diff AGENTS.md'));
            return;
          } else {
            console.log(chalk.yellow(`⚠ Auto-merge failed: ${result.error}`));
            console.log(chalk.gray('  Falling back to manual merge...'));
          }
        }
      }
    }
    
    // 3. Fallback: Show manual instructions (existing behavior)
    showManualMergeInstructions(promptPath);
  }
}
```

## Plan

- [ ] **Phase 1: CLI Configuration**
  - [ ] Extend `AIToolConfig` interface with `cli` property
  - [ ] Add CLI configs for copilot, claude, gemini, aider
  - [ ] Test CLI detection for each tool

- [ ] **Phase 2: Merge Execution**
  - [ ] Implement `buildMergeCommand()` function
  - [ ] Implement `executeMergeWithAI()` with spawn/exec
  - [ ] Add timeout handling (60s default)
  - [ ] Capture and display AI output

- [ ] **Phase 3: Integration**
  - [ ] Update `handleExistingFiles()` with auto-merge flow
  - [ ] Add user confirmation prompt
  - [ ] Handle `-y` flag behavior (auto-execute if CLI detected)
  - [ ] Update error handling and fallbacks

- [ ] **Phase 4: Testing & Polish**
  - [ ] Test with copilot CLI
  - [ ] Test with claude CLI
  - [ ] Test fallback when no CLI available
  - [ ] Test `-y` flag behavior
  - [ ] Update documentation

## Test

**Automated Tests:**

- [ ] `buildMergeCommand()` generates correct command for each tool
- [ ] Detection reuses spec 126 logic correctly
- [ ] Timeout handling works (mock slow execution)
- [ ] Error handling covers all failure modes

**Manual Tests:**

- [ ] **Fresh project with existing AGENTS.md + copilot installed**

  ```bash
  mkdir test-project && cd test-project
  echo "# My Project\n\nExisting instructions..." > AGENTS.md
  harnspec init
  # Expected: Detects copilot, asks to auto-merge, executes successfully
  ```

- [ ] **No CLI detected**

  ```bash
  # In environment without AI CLIs
  harnspec init
  # Expected: Falls back to manual instructions (current behavior)
  ```

- [ ] **Auto-merge declined**

  ```bash
  harnspec init
  # Choose AI-Assisted Merge
  # Say "No" to auto-merge
  # Expected: Shows manual instructions
  ```

- [ ] **With -y flag**

  ```bash
  harnspec init -y
  # Expected: Auto-merges without asking if CLI detected
  ```

## Notes

### CLI Syntax Research Needed

Before implementation, verify exact CLI syntax for each tool:

| Tool | Verified | Notes |
|------|----------|-------|
| `copilot` | ✅ | `copilot -p "prompt" --allow-all-tools` |
| `claude` | ❓ | Need to verify prompt flag syntax |
| `gemini` | ❓ | Need to verify CLI exists and syntax |
| `aider` | ❓ | Different interaction model (chat-based) |

### Alternative: Command-to-Clipboard

If execution feels too invasive, alternative:

```
🔍 Detected: copilot CLI
Command copied to clipboard:
  copilot -p "follow .harnspec/MERGE-AGENTS-PROMPT.md..." --allow-all-tools

Paste in terminal to merge automatically.
```

### Future Enhancements

1. **Multi-tool selection**: If multiple CLIs detected, let user choose
2. **Dry-run mode**: Show what merge would produce without applying
3. **Interactive merge**: Stream AI output in real-time
4. **Merge preview**: Show diff before applying changes
5. **Retry with different tool**: If first tool fails, offer alternatives
