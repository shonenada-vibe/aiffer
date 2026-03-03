## 2026-03-03 - Task: Build comment thread display

### What was done:
- Created `CommentThread.svelte` — renders below diff lines when comments exist:
  - Each comment shows: avatar placeholder (gray circle with user icon), "You" label, relative timestamp
  - Comment body with `whitespace-pre-wrap` for multi-line support
  - Edit button: turns body into textarea with Save/Cancel, Cmd+Enter to save, Escape to cancel
  - Delete button: shows inline confirmation (Delete/Cancel) before removing
  - Edit/Delete buttons appear on hover (`group-hover/comment` pattern)
  - GitHub-style card: blue left border accent, gray-50 background, rounded-right
  - Collapse logic: if >3 comments, shows first 2 + "Show N more" link
  - Directly imports `commentStore` for edit/delete operations (no prop drilling needed)
- Updated `DiffHunk.svelte`:
  - Imports `commentStore` to look up comments per line
  - Computes `lineNumber` for each line (newLineno ?? oldLineno ?? 0) to query store
  - Renders CommentThread after each line that has comments
  - CommentThread appears before CommentForm (existing comments first, then new form)
  - Passes `commentCount` to DiffLineRow for gutter badge
- Updated `DiffLine.svelte`:
  - Added `commentCount` prop
  - Shows blue circular badge with comment count in the gutter when comments exist
  - Badge replaces the hover-only "+" button when comments are present
  - Badge is always visible (not hover-dependent) for quick scanning
  - Clicking badge opens/closes the comment form

### Data Flow:
1. User submits comment via CommentForm → stored in commentStore keyed by `filePath:lineNumber`
2. DiffHunk reads `commentStore.getCommentsForLine(filePath, lineNo)` for each line (reactive via $state)
3. CommentThread renders existing comments with edit/delete capability
4. Edit: `commentStore.editComment(id, newBody)` updates in-place
5. Delete: `commentStore.removeComment(id)` removes from store, reactively updates UI

### Testing:
- `bun run check` — 0 errors, 0 warnings (122 files)
- `cargo clippy` — 0 warnings
- `cargo test` — 9 tests pass

### Notes:
- CommentThread directly imports commentStore rather than threading callbacks through DiffContent → DiffFileSection → DiffHunk. This is cleaner since edit/delete don't require coordination at the DiffContent level.
- Comment count badge uses z-[1] to stay above the line number text
- The `group/comment` Tailwind variant is used for per-card hover detection on edit/delete buttons
- Relative time formatting handles: "just now", "Nm ago", "Nh ago", "Nd ago"
