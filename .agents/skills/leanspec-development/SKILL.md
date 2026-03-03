---
name: leanspec-development
description: Development workflows and contribution guidelines for LeanSpec. Use when contributing code, fixing bugs, setting up dev environment, running tests or linting, or working with the monorepo structure.
---

# LeanSpec Development Skill

**Quick activation context** for AI agents working on LeanSpec development.

## Quick Navigation

| Goal | Reference |
|------|-----------|
| **Mandatory rules & conventions** | [RULES.md](./references/RULES.md) |
| **i18n file locations & patterns** | [I18N.md](./references/I18N.md) |
| **Monorepo structure & packages** | [STRUCTURE.md](./references/STRUCTURE.md) |
| **All scripts & commands** | Use skill: `leanspec-scripts` |

**Everything else**: Read root `README.md`, `package.json` scripts, or explore the codebase.

## Core Principles

The non-negotiable mental model:

1. **Use pnpm** - Never npm or yarn
2. **DRY** - Extract shared logic, avoid duplication
3. **Test What Matters** - Business logic and data integrity, not presentation
4. **Leverage Turborepo** - Smart caching (19s → 126ms builds)
5. **i18n is MANDATORY** - Every user-facing string needs both en AND zh-CN (see [I18N.md](./references/I18N.md))
6. **Follow Rust Quality** - All code must pass `cargo clippy -- -D warnings`

## Essential Commands

```bash
pnpm install              # Install dependencies
pnpm build                # Build all packages
pnpm dev                  # Start web UI + HTTP server
pnpm typecheck            # ← NEVER SKIP before marking work complete
pnpm test                 # All tests
pnpm pre-release          # Full validation: typecheck, test, build, lint
```

**⚠️ Always run `pnpm typecheck` before marking work complete.**

**For all commands, use skill: `leanspec-scripts`.**

## Critical Rules

Rules enforced by hooks or CI:

1. **Light/Dark Theme** - ALL UI must support both themes
2. **i18n** - Update BOTH en and zh-CN → [I18N.md](./references/I18N.md) ⚠️ commonly forgotten
3. **Regression Tests** - Bug fixes MUST include failing-then-passing tests
4. **Rust Quality** - Must pass `cargo clippy -- -D warnings`
5. **Rust Params Structs** - Functions with >7 args must use a params struct (enforced by `clippy.toml`)
5. **Use shadcn/ui** - No native HTML form elements
6. **cursor-pointer** - All clickable items must use `cursor-pointer`

**See [RULES.md](./references/RULES.md) for complete requirements.**
