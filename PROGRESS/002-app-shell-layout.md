## 2026-03-03 - Task: Build the main app shell layout

### What was done:
- Created `Header.svelte` — dark top bar with app logo, folder path display, refresh/AI/settings buttons, and green "Submit Review" button with comment count badge
- Created `Sidebar.svelte` — left panel (resizable via drag, 180–500px range) with "Changed Files" header and empty state placeholder
- Created `DiffContent.svelte` — main content area with welcome empty state and diff stats bar placeholder
- Created `AiPanel.svelte` — collapsible right panel (384px) for AI response display
- Updated `App.svelte` to compose all layout components with proper state management

### UI Details:
- Header: GitHub-style dark (bg-gray-900), 56px height, Octicon-style SVG icons
- Sidebar: resizable with drag handle, default 250px, bg-gray-50
- Main area: white bg, flex-1 to fill remaining space
- AI panel: slides in from right, 384px wide, bg-gray-50, close button
- Full-height layout with overflow-hidden to prevent scroll issues

### Testing:
- `bun run check` — 0 errors, 0 warnings (111 files)
- `cargo clippy` — 0 warnings
- `bun run tauri dev` — app launches, layout renders correctly

### Notes:
- Sidebar resize uses mousedown/mousemove/mouseup pattern with cursor lock
- a11y: resize handle uses `role="separator"` with aria attributes and svelte-ignore pragma
- All callback props use `on*` naming convention (onSubmitReview, onClose, etc.)
- Components are ready for wiring to real data in subsequent tasks
