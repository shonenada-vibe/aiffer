# Aiffer - AI-Powered Diff Reviewer

A cross-platform desktop app (Rust + Tauri + Svelte) that displays git diffs of the current folder and lets users comment on lines like a GitHub Pull Request review. Comments with code locations are collected and sent to an AI Agent to understand what changes the user wants.

## Tech Stack

- **Backend**: Rust + Tauri v2
- **Frontend**: Svelte 5 + TypeScript + TailwindCSS v4
- **Package Manager**: Bun
- **Git Operations**: `git2` Rust crate
- **Styling**: TailwindCSS v4 (GitHub-inspired dark/light theme)

## Project Structure

```
aiffer/
├── src-tauri/          # Rust backend (Tauri)
│   ├── src/
│   │   ├── main.rs     # Tauri entry point
│   │   ├── lib.rs      # Library root
│   │   ├── commands/   # Tauri IPC commands
│   │   ├── git/        # Git operations (diff parsing, file reading)
│   │   └── models/     # Shared data structures
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                # Svelte frontend
│   ├── lib/
│   │   ├── components/ # Svelte components
│   │   ├── stores/     # Svelte stores (state management)
│   │   ├── types/      # TypeScript type definitions
│   │   └── utils/      # Utility functions
│   ├── App.svelte      # Root component
│   └── main.ts         # Frontend entry point
├── tasks.json          # Task tracking for agents
├── setup.sh            # Environment setup script
├── PROGRESS/           # Agent progress logs
└── CLAUDE.md           # This file
```

## Agent Workflow

Each agent MUST follow this workflow exactly.

### Step 1: Initialize Environment

Run `setup.sh` to set up the development environment.

```bash
chmod +x setup.sh && ./setup.sh
```

**DO NOT skip this step.** The script installs all dependencies and starts the development server. Environment is isolated — no system pollution.

### Step 2: Select Next Task

Read `tasks.json` and select ONE task to work on.

Selection criteria (in order of priority):
1. Choose a task where `passes: false`
2. Consider dependencies — fundamental features (lower IDs) should be done first
3. Pick the highest-priority incomplete task

### Step 3: Implement the Task

- Read the task description and steps carefully
- Implement the functionality to satisfy all steps
- Follow existing code patterns and conventions in this file

### Step 4: Test Thoroughly

After implementation, verify ALL steps in the task:
- Run `cargo check` and `cargo clippy` for Rust code
- Run `bun run check` for Svelte/TypeScript code
- Run `cargo test` for Rust unit tests
- Use browser testing for UI features (MCP Playwright tools)
- Ensure no compile errors, no lint warnings
- Fix any errors before proceeding

### Step 5: Update Progress

Write your work to `PROGRESS/{task_id}.md`:

```
## [Date and Time] - Task: [task description]

### What was done:
- [specific changes made]

### Testing:
- [how it was tested]

### Notes:
- [any relevant notes for future agents]
```

### Step 6: Commit Changes

Commit changes following Google git message convention:

```
type(scope): short description

Detailed description of what was changed and why.
List specific files or modules affected.
```

Types: `feat`, `fix`, `refactor`, `style`, `test`, `docs`, `chore`, `build`

### Step 7: Mark Task Complete

In `tasks.json`, change the task's `passes` from `false` to `true`.

**IMPORTANT:**
- Only mark `passes: true` after ALL steps are verified
- Never delete or modify task descriptions
- Never remove tasks from the list

## Agent Key Rules

1. **One task per session** — Focus on completing one task well
2. **Test before marking complete** — All steps must pass
3. **Document in PROGRESS/** — Help future agents understand your work
4. **Commit your changes** — Keep git history clean
5. **Never remove tasks** — Only flip `passes: false` to `true`

## Coding Conventions

### Rust (src-tauri/)

- Use `snake_case` for functions, variables, modules
- Use `PascalCase` for types, structs, enums
- Use `SCREAMING_SNAKE_CASE` for constants
- Derive `serde::Serialize` and `serde::Deserialize` on all types shared with frontend
- Use `thiserror` for custom error types
- Use `Result<T, E>` for fallible operations — no `.unwrap()` in production code
- Run `cargo clippy` and fix all warnings
- Group imports: std → external crates → internal modules

### Svelte / TypeScript (src/)

- Use Svelte 5 runes syntax (`$state`, `$derived`, `$effect`)
- Use TypeScript strict mode
- Use `camelCase` for variables/functions, `PascalCase` for components/types
- Keep components small and focused (< 200 lines)
- Use Svelte stores for shared state
- Use TailwindCSS utility classes — no custom CSS unless absolutely necessary
- Mirror GitHub's PR review UI patterns (colors, spacing, icons)

### General

- No `console.log` in production code (use proper logging)
- No hardcoded strings for user-facing text — keep them as constants
- Prefer composition over inheritance
- Keep functions pure where possible
