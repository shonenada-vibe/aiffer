## 2026-03-03 - Task: Build the unified diff viewer displaying file diffs with line numbers

### What was done:
- Created `DiffLine.svelte` — single diff line with:
  - Old/new line numbers in gutter columns (monospace, gray, right-aligned)
  - Prefix column (+/-/space)
  - Content column with monospace text
  - GitHub-style coloring: green-50 bg + green-800 text for additions, red-50 bg + red-800 text for deletions
  - Gutter uses darker shade (green-100 / red-100 / gray-50)
  - Row hover effect with brightness-95
- Created `DiffHunk.svelte` — renders hunk header (blue-50 background, blue-700 text) + all lines
- Created `DiffFileSection.svelte` — collapsible file section with:
  - File header: collapse chevron, file icon, file path (mono), status badge (colored pill), stats (+N -M)
  - Rename display: "(was old_path)"
  - Table-based layout for precise gutter alignment
  - Collapse/expand toggle on click
- Rewrote `DiffContent.svelte` with full state handling:
  - Empty state (no folder): welcome message
  - Error state: warning icon + error message
  - Loading state: skeleton placeholders (3 file sections with 6 line placeholders each)
  - No changes state: checkmark + "working directory is clean"
  - Active state: stats bar + file sections with full diff rendering
- Updated `App.svelte` to pass diffFiles, loading, error, totals to DiffContent

### Testing:
- `bun run check` — 0 errors, 0 warnings (118 files)
- `bun run tauri dev` — app launches, renders correctly

### Notes:
- File sections have `id="diff-file-{file.path}"` for scroll-to-file from sidebar
- Table layout ensures line numbers stay aligned across hunks
- `whitespace-pre-wrap break-all` preserves code formatting while allowing wrapping on narrow windows
- Empty line content renders as "\n" to prevent collapsed rows
