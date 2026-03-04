## 2026-03-04 - Task: Add syntax highlighting for diff content based on file type

### What was done:
- Installed `highlight.js` (v11.11.1) as a dependency
- Created `src/lib/utils/highlight.ts` utility:
  - Registers languages individually from highlight.js/lib/core for tree-shaking
  - `detectLanguage(filePath)` maps file extensions to highlight.js language names
  - `highlightLine(content, language)` highlights a single line with `ignoreIllegals: true` for partial-line safety
- Supported languages: JavaScript, TypeScript, Rust, Python, Go, HTML/XML/SVG/Svelte/Vue, CSS/SCSS, JSON, YAML, Markdown, Bash/Shell, SQL, Java, C, C++
- Updated component chain: DiffFileSection → DiffHunk → DiffLine to pass `language` prop
- DiffLine renders highlighted content via `{@html}` when language is detected, plain text otherwise
- Imported `highlight.js/styles/github.css` in `app.css`
- Added CSS override to make `.hljs` background transparent so diff line colors are preserved

### Testing:
- `cargo check` — passes
- `cargo clippy` — no warnings
- `cargo test` — all 12 tests pass
- `bun run check` — 0 errors, 0 warnings

### Notes:
- Uses individual language imports instead of full highlight.js to minimize bundle size
- `ignoreIllegals: true` ensures partial lines (common in diffs) don't throw errors
- The `.hljs` class on content `<td>` enables highlight.js span styling while inheriting diff background colors
- TOML files map to YAML highlighter as a reasonable approximation
