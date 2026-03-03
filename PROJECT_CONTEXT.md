# Aiffer — Project Context

## Overview

**Aiffer** is a cross-platform desktop application that brings the GitHub Pull Request review experience to local git diffs. Users open a folder, see all uncommitted or staged changes in a familiar diff view, add inline comments on specific lines, and then submit those annotated comments to an AI agent that interprets the user's intent and suggests concrete code changes.

## Core User Flow

1. **Open folder** — User launches Aiffer in a git repository (or selects a folder)
2. **View diffs** — App reads `git diff` (staged/unstaged/between commits) and displays file-by-file diffs
3. **Review & comment** — User clicks on diff lines to add inline comments (like GitHub PR reviews)
4. **Submit to AI** — User clicks "Submit Review" which collects all comments with file/line context and sends to an AI agent
5. **AI response** — AI interprets the comments and responds with understanding of what changes the user wants

## Architecture

### Backend (Rust + Tauri v2)

The Rust backend handles:
- **Git operations** via the `git2` crate — reading diffs, file statuses, commit history
- **Diff parsing** — converting raw git diffs into structured data (files → hunks → lines)
- **Tauri IPC commands** — exposing backend functions to the frontend via `#[tauri::command]`
- **AI agent communication** — serializing comments + code context and calling LLM APIs

Key Rust modules:
- `commands/` — Tauri command handlers (thin layer, delegates to service modules)
- `git/` — Git repository operations (diff, status, file reading)
- `models/` — Shared data structures (DiffFile, DiffHunk, DiffLine, Comment, ReviewPayload)

### Frontend (Svelte 5 + TypeScript + TailwindCSS)

The frontend renders the GitHub-style review UI:
- **File tree sidebar** — lists changed files with status icons (A/M/D/R)
- **Diff viewer** — unified diff view with line numbers, syntax highlighting, +/- coloring
- **Comment system** — inline comment forms, threaded comment display
- **Review panel** — summary of all comments, submit button
- **AI response panel** — displays AI agent's interpretation

Key Svelte components:
- `DiffViewer.svelte` — main diff rendering component
- `DiffHunk.svelte` — renders a single hunk with context/added/removed lines
- `DiffLine.svelte` — single line with gutter, line number, content, comment trigger
- `CommentForm.svelte` — inline comment input (appears below a line)
- `CommentThread.svelte` — displays existing comments on a line
- `FileTree.svelte` — sidebar file list with change indicators
- `ReviewSummary.svelte` — collected comments overview
- `AiResponsePanel.svelte` — AI interpretation display

### Data Flow

```
Git Repo → [git2 crate] → Structured Diff Data → [Tauri IPC] → Svelte Frontend
                                                                      ↓
User Comments + Line Locations → [Tauri IPC] → Rust Backend → AI Agent API
                                                                      ↓
                                                              AI Response → Frontend Display
```

## Key Data Models

### DiffFile
```rust
struct DiffFile {
    path: String,
    old_path: Option<String>,  // for renames
    status: FileStatus,        // Added, Modified, Deleted, Renamed
    hunks: Vec<DiffHunk>,
    stats: DiffStats,          // additions, deletions
}
```

### DiffHunk
```rust
struct DiffHunk {
    header: String,            // @@ -10,5 +10,7 @@
    old_start: u32,
    new_start: u32,
    lines: Vec<DiffLine>,
}
```

### DiffLine
```rust
struct DiffLine {
    line_type: LineType,       // Context, Addition, Deletion
    old_lineno: Option<u32>,
    new_lineno: Option<u32>,
    content: String,
}
```

### Comment
```rust
struct Comment {
    id: String,
    file_path: String,
    line_number: u32,
    line_type: LineType,
    body: String,
    created_at: DateTime,
}
```

### ReviewPayload (sent to AI)
```rust
struct ReviewPayload {
    comments: Vec<CommentWithContext>,  // comment + surrounding code
    diff_summary: String,              // overall diff stats
    file_list: Vec<String>,            // all changed files
}
```

## UI Design Reference

The UI closely mirrors GitHub's Pull Request "Files changed" tab:

- **Layout**: Left sidebar (file tree) + Main area (diff viewer) + Optional right panel (AI response)
- **Colors**: GitHub-style diff coloring — green (#e6ffec) for additions, red (#ffebe9) for deletions, neutral for context
- **Line gutter**: Line numbers on both sides (old/new), clickable "+" icon appears on hover to add comments
- **Comment UI**: Inline below the commented line, with avatar placeholder, text area, and submit button
- **File headers**: Collapsible file sections with path, status badge, and stats (e.g., "+12 -5")
- **Top bar**: Repository/folder name, branch info, overall diff stats, "Submit Review" button

## Dependencies

### Rust (Cargo.toml)
- `tauri` v2 — app framework
- `git2` — libgit2 bindings for git operations
- `serde` + `serde_json` — serialization
- `thiserror` — error handling
- `chrono` — datetime handling
- `uuid` — comment IDs
- `reqwest` — HTTP client for AI API calls
- `tokio` — async runtime

### Frontend (package.json)
- `@tauri-apps/api` v2 — Tauri frontend bindings
- `svelte` v5 — UI framework
- `typescript` — type safety
- `tailwindcss` v4 — styling
- `@tailwindcss/vite` — Vite integration
- `vite` — bundler
- `@sveltejs/vite-plugin-svelte` — Svelte Vite plugin
