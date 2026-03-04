## 2026-03-04 - Task: Implement dark and light theme toggle matching GitHub's theme

### What was done:
- Added comprehensive CSS variables to app.css for both light and dark themes:
  - Diff line colors (add/del/highlight backgrounds, gutter, text)
  - Hunk header colors
  - Gutter colors
  - Accent/interactive colors
  - Semantic status colors (success, danger, warning, info, neutral)
  - Button colors (primary, danger, secondary)
  - Focus ring, overlay, skeleton colors
- Updated all components to use CSS variables instead of hardcoded Tailwind dark: variants:
  - DiffLine.svelte - uses scoped CSS with CSS variable mapping per line type
  - DiffHunk.svelte - hunk header uses CSS vars
  - DiffFileSection.svelte - file header, status badges, stats use CSS vars
  - DiffContent.svelte - stats bar, toolbar, empty states, skeletons use CSS vars
  - CommentForm.svelte - form card, textarea, buttons use CSS vars
  - CommentThread.svelte - comment cards, actions, edit mode use CSS vars
  - Sidebar.svelte - file list, selection, status badges, skeletons use CSS vars
  - Header.svelte - logo, submit button use CSS vars
  - Settings.svelte - overlay, validation error, save button use CSS vars
  - AiPanel.svelte - spinner, error state, retry button, prose use CSS vars
  - ReviewSummary.svelte - comment count badge, line type badges, buttons use CSS vars

### Testing:
- `bun run check` - 0 errors, 0 warnings
- `cargo check` - passes
- `cargo clippy` - no warnings

### Notes:
- Theme infrastructure was already solid (settings store, toggle, persistence, .dark class)
- The main work was replacing hardcoded color classes with CSS variable usage
- Components now use scoped <style> blocks with CSS variables for theme-dependent colors
- Some components use inline style attributes for dynamic CSS variable references (e.g., status badge colors)
- GitHub's exact color palette is preserved for both light and dark themes
