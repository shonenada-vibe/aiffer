## 2026-03-03 - Task: Build the inline comment form

### What was done:
- Created `CommentForm.svelte` — inline form rendered as a `<tr>` in the diff table:
  - Textarea with auto-focus on mount ($effect)
  - "Add Comment" green button (disabled when empty)
  - "Cancel" button
  - Cmd+Enter (Mac) / Ctrl+Enter (Win) to submit
  - Escape to cancel
  - GitHub-style card: white bg, rounded border, shadow, gray footer bar
  - Keyboard shortcut hints shown in footer
- Updated prop chain through all diff components:
  - `DiffHunk.svelte`: accepts `activeCommentKey`, `onSubmitComment`, `onCancelComment`; renders CommentForm after the matching line
  - `DiffFileSection.svelte`: passes through new props
  - `DiffContent.svelte`: manages `activeCommentKey` state; clicking '+' toggles form open/closed; submit clears form; replaces `onClickComment` with internal handler
  - `App.svelte`: `handleSubmitComment` calls `commentStore.addComment`, removed old `handleClickComment`
- Prop interface change: DiffContent now takes `onSubmitComment` instead of `onClickComment`

### Data Flow:
1. User clicks '+' on DiffLine → bubbles to DiffContent.handleClickComment → sets activeCommentKey
2. DiffHunk renders CommentForm after the line matching activeCommentKey
3. User types and submits → DiffContent.handleSubmitComment → App.handleSubmitComment → commentStore.addComment
4. Comment count badge in Header updates reactively via commentStore.commentCount

### Testing:
- `bun run check` — 0 errors, 0 warnings (121 files)
- `cargo clippy` — 0 warnings, 9 tests pass

### Notes:
- Form is a table row (<tr>) with colspan=4 to span all gutter + content columns
- activeCommentKey uses same format as highlightedLineKey: `filePath:oldLineno:newLineno`
- Only one comment form can be open at a time (opening another closes the current)
- `navigator.platform.includes("Mac")` used to show correct modifier key hint
