## 2026-03-04 - Task: Implement folder opening via file dialog and drag-and-drop

### What was done:

**Rust backend:**
- Added `last_opened_folder` field to `AppSettings` struct (with `#[serde(default)]` for backward compatibility)
- Added `validate_git_repo` Tauri command - validates path exists, is a directory, and contains a git repo; returns canonical path or user-friendly error
- Added `get_initial_path` Tauri command - reads CLI arguments (`std::env::args`) and returns the first non-flag argument if it's a valid directory
- Registered both new commands in Tauri invoke handler

**Frontend:**
- Added `lastOpenedFolder` to `AppSettings` interface and defaults
- Implemented `tryOpenFolder(path)` function that validates via `validate_git_repo` backend command, opens the folder, and persists it as last opened
- On app startup: checks CLI argument first via `get_initial_path`, then falls back to `lastOpenedFolder` from settings
- Added drag-and-drop support: `ondragover`, `ondragleave`, `ondrop` handlers on the root div
- Added visual drag-over overlay with folder icon and instructions
- Added `setError()` method to diff store for showing non-git repo errors
- Settings panel preserves `lastOpenedFolder` when saving other settings
- Non-git directories show an actionable error message suggesting `git init`

### Testing:
- `bun run check` - 0 errors, 0 warnings
- `cargo check` - passes
- `cargo clippy` - no warnings
- `cargo test` - all 12 tests pass

### Notes:
- Drag-and-drop uses the `File.path` property exposed by Tauri on desktop
- CLI usage: `aiffer /path/to/repo` - the Rust side reads `std::env::args` and the frontend queries it on startup
- The `validate_git_repo` command uses git2's `Repository::discover` under the hood, so it handles nested directories within a repo
- Settings backward compatibility: `last_opened_folder` uses `#[serde(default)]` so old settings files without this field still load correctly
