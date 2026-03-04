## 2026-03-04 - Task: Add comprehensive error handling and loading states

### What was done:

**Toast notification system:**
- Created `toast.svelte.ts` store with support for success/error/warning/info toasts
- Auto-dismiss with configurable duration (4s default, 6s for errors)
- Created `ToastContainer.svelte` component with themed styling using CSS variables
- Slide-in animation, dismiss button per toast
- Wired toast notifications throughout the app:
  - "Opened repository: {name}" on successful folder open
  - "Comment added" on comment submission
  - "Settings saved" on settings save
  - "Diffs refreshed" on manual refresh
  - "AI review received" on successful AI response
  - Error toasts for failed folder opens, AI failures

**Error boundary:**
- Created `ErrorBoundary.svelte` component wrapping the entire app
- Catches `window.onerror` and `window.onunhandledrejection` events
- Shows overlay with error message, actionable hint, dismiss and reload buttons
- Themed with CSS variables

**Improved error messages (actionable):**
- Folder opening: distinguishes "not a git repo" (suggests git init) vs "permission denied" (suggests checking access)
- Diff refresh: distinguishes not-a-repo, permission denied, corrupted repo (suggests git fsck)
- AI submission: distinguishes missing endpoint, invalid API key (401), timeout, network error, invalid model
- All error messages tell the user what to do next

**Pre-existing (no changes needed):**
- Loading skeletons in DiffContent and Sidebar already exist
- Empty states ("No changes detected", "No folder open") already exist
- AI error state with retry button already exists

### Testing:
- `bun run check` - 0 errors, 0 warnings
- `cargo clippy` - no warnings
- `cargo test` - all 12 tests pass

### Notes:
- ErrorBoundary uses `Event` type for `onerror` handler (Svelte window binding) and casts to `ErrorEvent`
- Toast store uses module-level `$state` rune pattern consistent with other stores
- Toast durations: success/info 4s, error 6s, all manually dismissable
