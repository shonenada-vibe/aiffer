## 2026-03-03 - Task: Add hover effects, line highlighting, and expand/collapse to diff viewer

### What was done:
- **DiffLine.svelte** enhancements:
  - Added blue "+" button in gutter that appears on row hover (comment trigger)
  - Click-to-highlight: clicking line number toggles yellow highlight (bg-yellow-100 / bg-yellow-200 gutter)
  - Hover effect: subtle background change on mouseover (green-100/60, red-100/60, gray-50)
  - Line numbers become darker on hover (gray-400 → gray-600)
  - Added keyboard handler (Enter key) on line numbers for a11y
  - Passed filePath, highlighted, onClickLine, onClickComment as new props
- **DiffHunk.svelte** updates:
  - Passes filePath, highlightedLineKey, onClickLine, onClickComment through to lines
  - Generates unique lineKey from `filePath:oldLineno:newLineno`
- **DiffFileSection.svelte** updates:
  - File header is now `sticky top-0` so it stays visible when scrolling through long diffs
  - Added `forceCollapsed` prop (null = individual control, true/false = force all)
  - Local collapsed state toggles independently when forceCollapsed is null
  - Passes through highlightedLineKey, onClickLine, onClickComment
- **DiffContent.svelte** updates:
  - Stats bar is now `sticky top-0 z-[2]` to stay above file headers
  - Added "Expand all" / "Collapse all" / "Reset" buttons in stats bar
  - `forceCollapsed` state: null (default), true (all collapsed), false (all expanded)
  - Manages `highlightedLineKey` state — click toggles, second click clears
  - Accepts `onClickComment` prop and passes through

### Testing:
- `bun run check` — 0 errors, 0 warnings (118 files)
- App launches and renders (verified via curl http://localhost:1420 → HTTP 200)

### Notes:
- Comment trigger button calls onClickComment — wired to placeholder in App.svelte, will open form in task 009
- Sticky file headers use z-[1], stats bar uses z-[2] to layer properly
- Line highlighting key format: `${filePath}:${oldLineno}:${newLineno}`
