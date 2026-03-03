## 2026-03-03 - Task: Implement AI agent communication

### What was done:
- Added `reqwest` (0.12, json feature) and `tokio` (1, full features) to Cargo.toml
- Created `src-tauri/src/ai/mod.rs`:
  - `AiConfig` struct: endpoint, apiKey, model
  - `AiError` enum: Request, Api, Config errors
  - `SYSTEM_PROMPT` constant instructing AI to interpret review comments
  - `ChatMessage`, `ChatRequest`, `ChatResponse` types (OpenAI-compatible)
  - `submit_review()` async function: validates config, POSTs to OpenAI chat completions endpoint
  - Supports any OpenAI-compatible API (OpenAI, Anthropic via proxy, local models, etc.)
- Added `submit_review` Tauri command in `commands/mod.rs`:
  - Async command that takes review_text and AiConfig
  - Returns AI response string
  - CommandError extended with AiError variant
- Registered `submit_review` in lib.rs invoke_handler
- Updated `App.svelte`:
  - `handleSubmitToAi()` now calls `build_review_payload` then `submit_review`
  - Manages `aiLoading`, `aiResponse`, `aiError` state
  - Passes all state to AiPanel
- Updated `AiPanel.svelte`:
  - Accepts `loading`, `response`, `error`, `onRetry` props
  - Shows loading spinner, error state with retry, or AI response
  - Response displayed as pre-wrapped text
- Added `AiConfig` to TypeScript types in `comment.ts`

### Data Flow:
1. User clicks "Submit to AI" in ReviewSummary
2. App.handleSubmitToAi() builds CommentInput[] from commentStore
3. Calls Rust `build_review_payload(path, comments)` → ReviewPayload with formattedText
4. Calls Rust `submit_review(reviewText, config)` → AI response string
5. AiPanel displays the response (or error with retry)

### Testing:
- `bun run check` — 0 errors, 0 warnings (123 files)
- `cargo clippy` — 0 warnings
- `cargo test` — 12 tests pass

### Notes:
- AI config is currently hardcoded with empty API key (will be configured in task 015 - Settings Panel)
- System prompt instructs AI to: summarize changes, explain per-comment what to change, flag conflicts, prioritize
- Uses temperature=0.3 for more focused responses
- Error response body truncated to 500 chars for readability
- API URL is constructed as `{endpoint}/chat/completions` (strips trailing slash)
