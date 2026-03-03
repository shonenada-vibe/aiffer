## 2026-03-03 - Task: Implement comment serialization for AI payload

### What was done:
- Added new Rust model types in `models/mod.rs`:
  - `CommentInput` — comment data from frontend (filePath, lineNumber, lineType, body)
  - `CommentWithContext` — enriched comment with lineContent + contextBefore/contextAfter
  - `FileSummary` — lightweight file change summary (path, status, additions, deletions)
  - `ReviewPayload` — full payload with comments, fileSummary, stats, and formattedText
- Implemented `build_review_payload` Tauri command in `commands/mod.rs`:
  - Takes repo path + comment list from frontend
  - Reads unstaged diff, parses into DiffFile structures
  - `enrich_comment()` finds target line in diff, extracts 3 lines before/after as context
  - `format_review_text()` generates clean markdown for LLM consumption
  - Registered in lib.rs invoke_handler
- Updated TypeScript types in `comment.ts`:
  - Added `CommentInput`, `CommentWithContext`, `FileSummary`, `ReviewPayload` interfaces
  - Updated existing `CommentWithContext` to match Rust struct (was previously nested)
- Added 3 unit tests in `commands/mod.rs`:
  - `test_enrich_comment_finds_context` — verifies context extraction
  - `test_enrich_comment_missing_file` — handles missing file gracefully
  - `test_format_review_text_structure` — verifies markdown output structure

### Formatted Text Output:
```
# Code Review Comments

## Summary: N file(s) changed, +A -D

### Changed Files:
- path/to/file (Status, +A -D)

## Review Comments (N)

### Comment 1 — file:line (type)

\```
   1 |  context line
   2 |  context line
>  3 | +target line
   4 |  context line
\```

**Comment:** user's comment body

---
```

### Testing:
- `bun run check` — 0 errors, 0 warnings (123 files)
- `cargo clippy` — 0 warnings
- `cargo test` — 12 tests pass (9 git + 3 new serialization)

### Notes:
- Context extraction uses flat line list across all hunks within a file
- Line matching uses `new_lineno ?? old_lineno` to find the target line
- Formatted text uses markdown code blocks with line numbers for easy LLM parsing
- FileStatus is printed with Debug trait ({:?}) in formatted text — shows "Modified", "Added", etc.
