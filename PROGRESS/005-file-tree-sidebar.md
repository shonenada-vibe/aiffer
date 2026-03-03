## 2026-03-03 - Task: Build file tree sidebar showing changed files with status indicators

### What was done:
- Created `src/lib/stores/diff.svelte.ts` — reactive store using Svelte 5 runes ($state/$derived) for:
  - folderPath, files, diffFiles, selectedFile, loading, error, diffType
  - totalAdditions/totalDeletions derived values
  - refresh(), openFolder(), setSelectedFile(), setDiffType() actions
  - Invokes Tauri commands (get_file_status, get_diff) with error handling
- Rewrote `Sidebar.svelte` with full data binding:
  - File list with status badges (A=green, M=yellow, D=red, R=blue, C=blue, U=gray)
  - File path display with directory prefix in muted color, filename in bold
  - Per-file diff stats (+N / -M) aligned right
  - Overall summary bar: "N files changed +X -Y"
  - Selected file highlighting (blue background)
  - Loading skeleton (5 animated placeholder rows)
  - Empty states: "Open a folder" / "No changes detected"
- Added Tauri dialog plugin:
  - Installed `@tauri-apps/plugin-dialog` (frontend) + `tauri-plugin-dialog` (Rust)
  - Registered plugin in lib.rs, added `dialog:allow-open` permission
- Updated `App.svelte`:
  - handleOpenFolder() uses native folder picker dialog
  - handleRefresh() calls diffStore.refresh()
  - handleSelectFile() updates store + scrolls to file in diff viewer
  - All sidebar props wired from diffStore

### Testing:
- `bun run check` — 0 errors, 0 warnings (115 files)
- `cargo clippy -- -D warnings` — 0 warnings
- `cargo test` — 9 tests pass
- `bun run tauri dev` — app launches, sidebar renders

### Notes:
- Store file must use `.svelte.ts` extension for rune support in non-component files
- Import path is `./lib/stores/diff.svelte` (no .ts extension in imports)
- Sidebar scrolls to file via `document.getElementById(`diff-file-${CSS.escape(path)}`)` — needs DiffViewer to set those IDs
