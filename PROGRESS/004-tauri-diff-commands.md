## 2026-03-03 - Task: Create Tauri IPC commands to expose git diff data to frontend

### What was done:
- Implemented `get_diff(path, diff_type)` Tauri command — returns `Vec<DiffFile>` with full hunk/line data
- Implemented `get_file_status(path)` Tauri command — returns sorted `Vec<FileEntry>` combining both unstaged and staged changes
- Created `CommandError` type that wraps `GitError` and implements `serde::Serialize` for Tauri error handling
- Registered both commands in `lib.rs` invoke_handler
- Created `src/lib/types/diff.ts` — TypeScript interfaces matching all Rust models (DiffFile, DiffHunk, DiffLine, FileEntry, FileStatus, LineType)

### Frontend invoke examples:
```typescript
import { invoke } from "@tauri-apps/api/core";
// Full diff
const files = await invoke<DiffFile[]>("get_diff", { path: ".", diffType: "unstaged" });
// File list only
const entries = await invoke<FileEntry[]>("get_file_status", { path: "." });
```

### Testing:
- `cargo clippy -- -D warnings` — 0 warnings
- `cargo test` — 9 tests pass
- `bun run check` — 0 errors, 0 warnings (112 files)

### Notes:
- `get_file_status` merges unstaged + staged entries, deduplicating by path
- Tauri command errors are serialized as plain strings for frontend consumption
- TypeScript types use camelCase matching Rust's `#[serde(rename_all = "camelCase")]`
