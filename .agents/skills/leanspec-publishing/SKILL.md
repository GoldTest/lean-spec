---
name: leanspec-publishing
description: Publishing and release workflows for LeanSpec packages. Use when preparing releases, publishing to npm, bumping versions, syncing package versions, testing dev builds, or troubleshooting npm distribution.
---

# LeanSpec Publishing Skill

Teach agents how to publish and release LeanSpec packages to npm.

## Core Principles

1. **Root as Source of Truth**: All versions come from root `package.json`
2. **Platform Packages First**: Platform binaries must be published before main packages
3. **Automated Sync**: Use scripts, don't manually edit package.json files
4. **Test Before Release**: Always run `pnpm pre-release` before publishing
5. **Dev Tags for Testing**: Use `-dev` tags for testing across platforms

## Release Workflow

### Production Release (Recommended)

```bash
# 1. Update version (root only)
npm version patch  # or minor/major

# 2. Sync all packages
pnpm sync-versions

# 3. Validate everything
pnpm pre-release

# 4. Commit and push with tags
git add .
git commit -m "chore: release vX.X.X"
git push --follow-tags

# 5. Create GitHub Release (triggers publish workflow automatically)
gh release create vX.X.X --title "vX.X.X" --notes "Release notes here"
```

### Development Release

```bash
# Publish dev version via GitHub Actions (all platforms)
gh workflow run publish.yml --field dev=true

# Dry run (validates without publishing)
gh workflow run publish.yml --field dev=true --field dry_run=true

# Install and test dev version
npm install -g lean-spec@dev
lean-spec --version
```

## Version Management

- Root `package.json` is the single source of truth
- `pnpm sync-versions` propagates to all packages (including Rust crates)
- CI automatically validates version alignment
- **Never manually edit package versions** — use `npm version` + `pnpm sync-versions`

## Distribution Architecture

LeanSpec uses the **optional dependencies pattern** for Rust binaries:

```
Main Package (lean-spec)
├── bin/lean-spec.js (wrapper)
└── optionalDependencies:
    ├── @leanspec/cli-darwin-arm64
    ├── @leanspec/cli-darwin-x64
    ├── @leanspec/cli-linux-x64
    └── @leanspec/cli-windows-x64
```

⚠️ **Platform packages MUST be published before main packages.** The workflow handles this automatically.

## Package Structure

| Type | Packages |
|------|----------|
| **Main** (published) | `lean-spec`, `@leanspec/mcp`, `@leanspec/ui` |
| **Platform** (published) | `@leanspec/cli-{platform}`, `@leanspec/mcp-{platform}` (5 platforms each) |
| **Internal** (not published) | `@leanspec/desktop`, `@leanspec/ui-components` |

## Troubleshooting

**Binary not found**: `npm view @leanspec/cli-darwin-arm64 versions` to check, `pnpm rust:build` to rebuild.

**Version mismatch**: `pnpm sync-versions --dry-run` to check, `pnpm sync-versions` to fix.

**Permission issues**: `chmod +x /path/to/binary` (usually auto-fixed by postinstall).

## Detailed References

- [references/NPM-DISTRIBUTION.md](./references/NPM-DISTRIBUTION.md) - Architecture details
- [references/DEV-PUBLISHING.md](./references/DEV-PUBLISHING.md) - Development workflow
- [references/PUBLISHING.md](./references/PUBLISHING.md) - Full release checklist
- For CI/CD workflow details, use skill: `github-actions`
- For all scripts, use skill: `leanspec-scripts`
