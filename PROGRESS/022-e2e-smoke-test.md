## 2026-03-04 - Task: Create end-to-end smoke tests for core user flow

### What was done:

**Smoke test suite** (`src-tauri/tests/smoke_test.rs`):
- Created 18 integration tests covering the full backend user journey
- Test fixture: `setup_repo()` creates a temp git repo with 3 files (main.rs, lib.rs, README.md) and an initial commit

**Tests cover:**
1. Repo validation: accepts valid repos, rejects non-repos, rejects nonexistent paths
2. File modification diffs: modify a file, verify additions/deletions/hunks/line types
3. New file diffs: stage a new file, verify it appears with correct stats
4. Multiple files changed: modify 2 files + add 1, verify all 3 appear
5. Staged vs unstaged separation: stage one change, leave another unstaged, verify they appear in correct diff types
6. Review payload with comments: build payload, verify structure, context, and formatted text
7. Multiple comments on same file: verify both comments appear in payload
8. Commit history: get commits, verify structure
9. Between-commits diff: create two commits, diff them, verify result
10. Settings persistence: roundtrip serialization/deserialization
11. Settings backward compatibility: old settings without new fields still parse
12. Settings defaults: verify default values
13. Error handling: non-repo for diff/commits/payload, invalid diff type

**Documentation:**
- Added "Running Tests" section to CLAUDE.md with commands for all test types

### Testing:
- `cargo test` - all 30 tests pass (12 unit + 18 smoke)
- `bun run check` - 0 errors, 0 warnings
- `cargo clippy` - no warnings

### Notes:
- Smoke tests use the command layer (`commands::*`) which is the same entry point as Tauri IPC
- True browser/UI E2E would require Tauri test harness or WebDriver, which is significantly more complex
- The smoke tests cover the full data flow that the UI would exercise
- Untracked files need to be staged to get content in diffs (git2 behavior)
