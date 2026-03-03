## 2026-03-03 - Task: Implement git diff reading in Rust using git2 crate

### What was done:
- Added dependencies: git2 0.20, thiserror 2, chrono 0.4, uuid 1, tempfile 3 (dev)
- Created `src-tauri/src/models/mod.rs` with all shared data types:
  - `FileStatus` (Added, Modified, Deleted, Renamed, Copied, Untracked)
  - `LineType` (Context, Addition, Deletion)
  - `DiffLine`, `DiffHunk`, `DiffStats`, `DiffFile`, `FileEntry`
  - All types derive Serialize/Deserialize with camelCase rename
- Created `src-tauri/src/git/mod.rs` with:
  - `open_repo(path)` — discovers git repo from path
  - `get_diff(repo, diff_type)` — reads unstaged or staged diffs
  - `parse_diff(diff)` — converts git2::Diff into Vec<DiffFile> with hunks and lines
  - `get_file_entries(diff)` — lightweight file list with stats
- Created `src-tauri/src/commands/mod.rs` — moved greet command
- Updated `lib.rs` to declare all modules

### Testing:
- 9 unit tests, all passing:
  - test_open_repo, test_open_non_repo
  - test_unstaged_diff, test_staged_diff
  - test_deleted_file_diff, test_untracked_file
  - test_get_file_entries, test_invalid_diff_type
  - test_multi_hunk_diff (verifies multiple hunks for distant changes)
- `cargo clippy -- -D warnings` — 0 warnings

### Notes:
- `get_diff` uses explicit lifetime `'a` since Diff borrows from Repository
- Untracked files are included via `include_untracked(true)` in DiffOptions
- Test helper `setup_test_repo()` scopes tree/sig borrows to avoid borrow-move conflicts
- Line content has trailing newlines stripped for cleaner frontend display
