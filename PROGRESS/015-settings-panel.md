## 2026-03-03 - Task: Build settings panel for AI configuration and app preferences

### What was done:
- Added `dirs` crate (v6) to Cargo.toml for finding OS config directory
- Added `AppSettings` struct to `models/mod.rs`:
  - `aiEndpoint`, `aiApiKey`, `aiModel`, `diffType` fields
  - Serde defaults for each field, `Default` impl
- Added `load_settings` and `save_settings` Tauri commands:
  - Reads/writes JSON to `{config_dir}/aiffer/settings.json`
  - `load_settings` returns defaults if file doesn't exist
  - `save_settings` creates directory if needed, writes pretty JSON
  - Registered in lib.rs invoke_handler
- Created `settings.svelte.ts` store:
  - Reactive `$state` for settings
  - `load()` calls Rust `load_settings`, falls back to defaults on error
  - `save()` updates state and calls Rust `save_settings`
- Created `Settings.svelte` modal:
  - AI Configuration section: endpoint URL, API key (masked/show toggle), model name
  - Diff Options section: default diff type dropdown (unstaged/staged)
  - URL validation (must start with http)
  - Save/Cancel buttons, Escape to close, backdrop click to close
  - Form state synced from store when modal opens
- Updated `App.svelte`:
  - Loads settings on mount via `$effect(() => settingsStore.load())`
  - Gear icon in header now toggles settings modal
  - AI config for `submit_review` reads from `settingsStore.settings`
  - Settings modal rendered outside flex layout (fixed overlay)

### Data Flow:
1. App mounts → settingsStore.load() → Rust load_settings → reads JSON file (or defaults)
2. User clicks gear → settingsOpen = true → Settings modal opens with current values
3. User edits and saves → settingsStore.save() → Rust save_settings → writes JSON
4. User submits review → reads settingsStore.settings for AI endpoint/key/model

### Testing:
- `bun run check` — 0 errors, 0 warnings (126 files)
- `cargo clippy` — 0 warnings
- `cargo test` — 12 tests pass

### Notes:
- Settings file location: `~/Library/Application Support/aiffer/settings.json` (macOS)
- API key is stored in plain text in the config file — consider keychain integration later
- CommandError extended with Io and Json variants for file/JSON errors
- `$effect` used for syncing form fields when modal opens (runs when `isOpen` changes)
