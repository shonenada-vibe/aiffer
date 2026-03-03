## 2026-03-03 - Task: Build the review summary panel

### What was done:
- Created `ReviewSummary.svelte` — right-side panel (384px) showing all comments grouped by file:
  - Header with "Review Summary" title, comment count badge, close button
  - Comments grouped by file path with file icon and comment count per file
  - Each comment card shows: line number, line type badge (+/-/ctx), code snippet (from diffStore), comment body (2-line clamp)
  - Clicking a comment card scrolls to that file in the diff viewer
  - "Clear All" button with inline confirmation (Confirm/Cancel)
  - "Submit to AI" green button that triggers AI submission flow
  - Empty state when no comments exist
- Updated `App.svelte`:
  - Added `reviewPanelOpen` state
  - `handleSubmitReview` now toggles the review summary panel
  - `handleSubmitToAi` placeholder closes review panel and opens AI panel
  - ReviewSummary placed between DiffContent and AiPanel in the layout

### Data Flow:
1. User clicks "Submit Review" in Header → toggles reviewPanelOpen
2. ReviewSummary reads commentStore.allComments reactively, groups by filePath
3. For each comment, looks up line content from diffStore.diffFiles
4. "Submit to AI" → closes review panel, opens AI panel (actual AI call is task 013)
5. "Clear All" → commentStore.clearAll(), resets confirmation state

### Testing:
- `bun run check` — 0 errors, 0 warnings (123 files)

### Notes:
- ReviewSummary imports both commentStore and diffStore directly for reactive data access
- `$derived.by()` used for grouping logic (computed from allComments)
- getLineContent() searches diffFiles hunks to find matching line content by lineNumber
- `line-clamp-2` Tailwind utility used to truncate long comment bodies
- Panel is between DiffContent and AiPanel in DOM order for logical right-side panel stacking
