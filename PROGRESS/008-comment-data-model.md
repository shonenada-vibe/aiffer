## 2026-03-03 - Task: Implement comment data model and state management in Svelte stores

### What was done:
- Created `src/lib/types/comment.ts` with:
  - `Comment` interface: id, filePath, lineNumber, lineType, body, createdAt
  - `CommentWithContext` interface: comment + lineContent + contextBefore/After (for AI payload)
- Created `src/lib/stores/comments.svelte.ts` reactive store:
  - Internal state: `Map<string, Comment[]>` keyed by `"filePath:lineNumber"`
  - `allComments` — derived flat sorted list of all comments
  - `commentCount` — derived total count
  - `getCommentsForLine(filePath, lineNumber)` — returns comments for a specific line
  - `getCommentCountForLine(filePath, lineNumber)` — count for badge display
  - `getCommentedLines()` — returns the full map
  - `addComment(filePath, lineNumber, lineType, body)` — creates with UUID and ISO timestamp
  - `removeComment(commentId)` — removes by ID, cleans up empty keys
  - `editComment(commentId, newBody)` — updates body in place
  - `clearAll()` — resets to empty map
- Wired `commentStore.commentCount` to Header badge in App.svelte

### Key Design Decisions:
- Map keyed by `filePath:lineNumber` for O(1) lookup per line
- Immutable updates: always creates new Map when modifying (triggers Svelte reactivity)
- UUID generated via `crypto.randomUUID()` (available in all modern browsers + Tauri webview)
- Comments sorted by createdAt for consistent ordering

### Testing:
- `bun run check` — 0 errors, 0 warnings (120 files)

### Notes:
- Store uses `.svelte.ts` extension for $state/$derived rune support
- Import as `import { commentStore } from "./lib/stores/comments.svelte"`
- Comment form UI (task 009) will call commentStore.addComment
- Review summary (task 011) will use commentStore.allComments
