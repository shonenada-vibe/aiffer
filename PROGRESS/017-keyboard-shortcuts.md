## 2026-03-04 - Task: Add keyboard shortcuts for common actions

### What was done:
- Added app-level keyboard handler via `<svelte:window onkeydown>` in App.svelte
- `n` / `p` — navigate to next/previous file (scrolls and updates sidebar selection)
- `j` / `k` — navigate to next/previous hunk using `data-hunk-header` attribute on hunk rows
- `c` — opens comment form on the currently highlighted line (via `data-line-highlighted` + `data-comment-trigger` attributes)
- `Cmd+Enter` — submits review (works both in text inputs and globally)
- `Cmd+Shift+R` — refreshes diffs
- `?` — toggles keyboard shortcut help overlay
- `Escape` — closes help overlay
- Shortcuts are suppressed when focus is in text inputs (INPUT, TEXTAREA, SELECT, contentEditable)
- Created `KeyboardShortcutHelp.svelte` modal with grouped shortcuts display
- Added `data-hunk-header` to DiffHunk.svelte for hunk navigation
- Added `data-line-highlighted` and `data-comment-trigger` to DiffLine.svelte for comment shortcut

### Testing:
- `cargo check` — passes
- `cargo clippy` — no warnings
- `cargo test` — all 12 tests pass
- `bun run check` — 0 errors, 0 warnings

### Notes:
- Hunk navigation uses scroll position relative to `<main>` container to find next/prev hunk
- The `c` shortcut requires a line to be highlighted first (click a line number, then press `c`)
