## 2026-03-03 - Task: Build the AI response display panel

### What was done:
- Installed `marked` (v17) for markdown parsing and `@tailwindcss/typography` for prose styling
- Updated `AiPanel.svelte` with full markdown rendering:
  - `marked.parse(response)` converts AI response to HTML
  - `{@html renderedHtml}` renders the HTML in a prose-styled container
  - Tailwind typography `prose prose-sm` classes for headings, paragraphs, code, lists, etc.
  - Custom prose modifiers for compact heading sizes and code block styling
- Added "Copy to Clipboard" button in panel header:
  - Uses `navigator.clipboard.writeText()`
  - Shows clipboard icon, switches to green checkmark for 2 seconds after copy
  - Only visible when there's a response and not loading
- Panel already had (from task 013): loading spinner, error state with retry button, close button
- Added `@plugin "@tailwindcss/typography"` to `app.css`

### Testing:
- `bun run check` — 0 errors, 0 warnings (124 files)

### Notes:
- `marked` is used synchronously — `marked.parse()` returns string directly
- `renderedHtml` is `$derived` from `response` for reactivity
- The `prose` class customizations: `prose-h1:text-lg`, `prose-h2:text-base`, `prose-h3:text-sm` keep headings compact in the narrow panel
- `prose-code:before:content-none prose-code:after:content-none` removes the backtick decorations around inline code
- Using `{@html}` is safe here since content comes from the AI API response via our own backend
