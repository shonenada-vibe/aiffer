## 2026-03-03 - Task: Initialize Tauri v2 + Svelte 5 project with TailwindCSS v4

### What was done:
- Created Tauri v2 project manually (CLI required interactive terminal)
- Configured Svelte 5.53.6 with TypeScript strict mode
- Installed TailwindCSS v4.2.1 with @tailwindcss/vite plugin
- Set up Vite 7.3.1 with HMR on port 1420
- Created Rust backend with greet command for verification
- Generated app icons from SVG placeholder
- Set up project directory structure (src/lib/{components,stores,types,utils}, src-tauri/src/{commands,git,models})

### Testing:
- `cargo check` — passes with no errors
- `cargo clippy` — passes with no warnings
- `bun run check` (svelte-check) — 0 errors, 0 warnings across 108 files
- `bun run tauri dev` — Vite starts, Rust compiles 380 crates, app window launches

### Notes:
- Tauri CLI is installed as a bun dev dependency, use `bun run tauri` not `cargo tauri`
- Vite dev server runs on port 1420 (configured in vite.config.ts and tauri.conf.json)
- App window is 1280x800, resizable
- Svelte 5 runes syntax ($state, $derived, $effect) should be used
- TailwindCSS v4 uses `@import "tailwindcss"` in CSS, no tailwind.config.js needed
