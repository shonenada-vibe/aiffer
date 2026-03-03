## 2026-03-03 - Task: Add toggle between staged, unstaged, and commit-range diffs

### What was done:
- Added `CommitInfo` model in Rust (`models/mod.rs`) for commit metadata
- Added `get_diff_between_commits()` in `git/mod.rs` to diff two arbitrary commits using `revparse_single` + `diff_tree_to_tree`
- Added `get_commits()` in `git/mod.rs` to list recent commits via `revwalk`
- Added `InvalidRef` error variant to `GitError`
- Updated `get_diff` Tauri command to accept optional `from_ref`/`to_ref` params; routes to `get_diff_between_commits` when `diff_type == "commits"`
- Added `get_commits` Tauri command and registered it in `lib.rs`
- Added `DiffType` and `CommitInfo` TypeScript types in `types/diff.ts`
- Extended `diffStore` with `commits`, `fromRef`, `toRef` state, `loadCommits()` method, and commit-aware `refresh()`
- Added diff type selector dropdown in `Header.svelte` (Unstaged / Staged / Between Commits)
- When "Between Commits" is selected, two commit selector dropdowns appear (From → To)
- Current diff type is shown prominently in the header
- Added comment invalidation warning: when user has comments and switches diff type, a confirm dialog warns them

### Testing:
- `cargo check` — passes
- `cargo clippy` — no warnings
- `cargo test` — all 12 tests pass
- `bun run check` — 0 errors, 0 warnings

### Notes:
- Comments are preserved when switching diff types (no auto-clear), user is just warned
- Commits are loaded lazily only when "Between Commits" mode is selected or when a folder is opened
- The diff only refreshes in "commits" mode when both from and to refs are selected
