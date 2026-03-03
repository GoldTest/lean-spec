---
name: github-actions
description: GitHub Actions workflow management for LeanSpec. Use when triggering CI/CD workflows, monitoring build status, debugging failed runs, managing artifacts, or checking CI before releases.
---

# GitHub Actions Skill

Teach agents how to trigger, monitor, and manage GitHub Actions workflows for the LeanSpec project.

## Core Principles

1. **Use GitHub CLI (`gh`)** - All workflow interactions use the `gh` CLI tool
2. **Check Status Before Acting** - Always verify current run status before triggering new ones
3. **Monitor Until Completion** - Background workflows need periodic status checks
4. **Artifacts Are Ephemeral** - Download important artifacts before they expire (default: 30 days)
5. **Respect Rate Limits** - Don't poll too frequently (minimum 30s between checks)

## Available Workflows

| Workflow | File | Triggers | Purpose |
|----------|------|----------|---------|
| **CI** | `ci.yml` | push, PR to main | Build, test, lint (Node.js + Rust) |
| **Publish** | `publish.yml` | release, manual | Publish to npm (all platforms) |
| **Desktop Build** | `desktop-build.yml` | push, PR, manual | Build Tauri desktop apps |
| **Copilot Setup** | `copilot-setup-steps.yml` | push, PR, manual | Setup environment for Copilot agent |

## Quick Reference

### Check Workflow Status

```bash
# List recent workflow runs
gh run list --limit 10

# View specific workflow runs
gh run list --workflow ci.yml --limit 5
gh run list --workflow publish.yml --limit 5

# Get details of a specific run
gh run view <run-id>

# Watch a run in progress
gh run watch <run-id>
```

### Trigger Workflows

```bash
# Trigger CI manually (usually automatic on push/PR)
gh workflow run ci.yml

# Trigger publish (dev version)
gh workflow run publish.yml --field dev=true

# Trigger publish (dry run - validates without publishing)
gh workflow run publish.yml --field dev=true --field dry_run=true

# Trigger desktop build
gh workflow run desktop-build.yml
```

### Debug Failed Runs

```bash
# View failed run logs
gh run view <run-id> --log-failed

# View full logs
gh run view <run-id> --log

# Re-run failed jobs only
gh run rerun <run-id> --failed

# Re-run entire workflow
gh run rerun <run-id>
```

### Manage Artifacts

```bash
# List artifacts from a run
gh run view <run-id>

# Download all artifacts from a run
gh run download <run-id>

# Download specific artifact
gh run download <run-id> --name ui-dist
gh run download <run-id> --name binaries-linux-x64
```

## Decision Tree

```
Need to check if build passing?
└─> gh run list --workflow ci.yml --limit 1

Need to publish packages?
├─> Production: Create GitHub Release (recommended)
└─> Testing: gh workflow run publish.yml --field dev=true

Workflow failed?
├─> View logs: gh run view <id> --log-failed
├─> Transient: gh run rerun <id> --failed
└─> Real issue: Fix locally, push, let CI re-run

Need build artifacts?
└─> gh run download <run-id> --name <artifact-name>
```

## Reference Documentation

- [references/WORKFLOWS.md](./references/WORKFLOWS.md) - Detailed workflow documentation (jobs, triggers, durations)
- [references/COMMANDS.md](./references/COMMANDS.md) - Complete `gh` command reference
- [references/TROUBLESHOOTING.md](./references/TROUBLESHOOTING.md) - Common issues and solutions