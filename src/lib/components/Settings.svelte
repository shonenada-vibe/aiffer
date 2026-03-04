<script lang="ts">
  import { settingsStore } from "../stores/settings.svelte";
  import type { AppSettings, Theme } from "../stores/settings.svelte";

  interface Props {
    isOpen: boolean;
    onClose: () => void;
  }

  let { isOpen, onClose }: Props = $props();

  let endpoint = $state("");
  let apiKey = $state("");
  let model = $state("");
  let diffType = $state("unstaged");
  let theme = $state<Theme>("light");
  let saving = $state(false);
  let showApiKey = $state(false);
  let validationError: string | null = $state(null);

  // Sync form state when panel opens
  $effect(() => {
    if (isOpen) {
      const s = settingsStore.settings;
      endpoint = s.aiEndpoint;
      apiKey = s.aiApiKey;
      model = s.aiModel;
      diffType = s.diffType;
      theme = s.theme;
      validationError = null;
    }
  });

  function validate(): boolean {
    if (endpoint && !endpoint.startsWith("http")) {
      validationError = "Endpoint must start with http:// or https://";
      return false;
    }
    validationError = null;
    return true;
  }

  async function handleSave() {
    if (!validate()) return;
    saving = true;
    try {
      const settings: AppSettings = {
        aiEndpoint: endpoint.trim(),
        aiApiKey: apiKey.trim(),
        aiModel: model.trim(),
        diffType,
        theme,
      };
      await settingsStore.save(settings);
      onClose();
    } catch (err) {
      validationError =
        err instanceof Error ? err.message : "Failed to save settings";
    } finally {
      saving = false;
    }
  }
</script>

{#if isOpen}
  <!-- Backdrop -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/40"
    onclick={(e) => {
      if (e.target === e.currentTarget) onClose();
    }}
    onkeydown={(e) => {
      if (e.key === "Escape") onClose();
    }}
  >
    <!-- Modal -->
    <div
      class="w-full max-w-lg rounded-lg border border-[var(--panel-border)] bg-[var(--panel-bg)] shadow-xl"
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between border-b border-[var(--panel-border)] px-6 py-4"
      >
        <h2 class="text-lg font-semibold text-[var(--app-fg)]">Settings</h2>
        <button
          onclick={onClose}
          class="rounded p-1 text-[var(--panel-muted-fg)] hover:bg-[var(--panel-muted-bg)] hover:text-[var(--app-fg)]"
          title="Close settings"
        >
          <svg class="h-5 w-5" viewBox="0 0 16 16" fill="currentColor">
            <path
              d="M3.72 3.72a.75.75 0 0 1 1.06 0L8 6.94l3.22-3.22a.749.749 0 0 1 1.275.326.749.749 0 0 1-.215.734L9.06 8l3.22 3.22a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215L8 9.06l-3.22 3.22a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042L6.94 8 3.72 4.78a.75.75 0 0 1 0-1.06Z"
            />
          </svg>
        </button>
      </div>

      <!-- Content -->
      <div class="space-y-6 px-6 py-4">
        <!-- AI Configuration -->
        <div>
          <h3 class="mb-3 text-sm font-semibold text-[var(--app-fg)]">
            AI Configuration
          </h3>
          <div class="space-y-3">
            <div>
              <label
                for="ai-endpoint"
                class="mb-1 block text-xs font-medium text-[var(--panel-muted-fg)]"
                >API Endpoint</label
              >
              <input
                id="ai-endpoint"
                type="url"
                bind:value={endpoint}
                placeholder="https://api.openai.com/v1"
                class="w-full rounded-md border border-[var(--input-border)] bg-[var(--input-bg)] px-3 py-2 text-sm text-[var(--app-fg)] placeholder-[var(--panel-faint-fg)] focus:border-blue-400 focus:outline-none focus:ring-1 focus:ring-blue-400"
              />
              <p class="mt-0.5 text-[11px] text-[var(--panel-faint-fg)]">
                Any OpenAI-compatible API endpoint
              </p>
            </div>

            <div>
              <label
                for="ai-api-key"
                class="mb-1 block text-xs font-medium text-[var(--panel-muted-fg)]"
                >API Key</label
              >
              <div class="flex gap-2">
                <input
                  id="ai-api-key"
                  type={showApiKey ? "text" : "password"}
                  bind:value={apiKey}
                  placeholder="sk-..."
                  class="flex-1 rounded-md border border-[var(--input-border)] bg-[var(--input-bg)] px-3 py-2 font-mono text-sm text-[var(--app-fg)] placeholder-[var(--panel-faint-fg)] focus:border-blue-400 focus:outline-none focus:ring-1 focus:ring-blue-400"
                />
                <button
                  type="button"
                  onclick={() => (showApiKey = !showApiKey)}
                  class="rounded-md border border-[var(--input-border)] bg-[var(--input-bg)] px-2 text-xs text-[var(--panel-muted-fg)] hover:bg-[var(--panel-muted-bg)]"
                >
                  {showApiKey ? "Hide" : "Show"}
                </button>
              </div>
            </div>

            <div>
              <label
                for="ai-model"
                class="mb-1 block text-xs font-medium text-[var(--panel-muted-fg)]"
                >Model</label
              >
              <input
                id="ai-model"
                type="text"
                bind:value={model}
                placeholder="gpt-4o"
                class="w-full rounded-md border border-[var(--input-border)] bg-[var(--input-bg)] px-3 py-2 text-sm text-[var(--app-fg)] placeholder-[var(--panel-faint-fg)] focus:border-blue-400 focus:outline-none focus:ring-1 focus:ring-blue-400"
              />
            </div>
          </div>
        </div>

        <!-- Appearance -->
        <div>
          <h3 class="mb-3 text-sm font-semibold text-[var(--app-fg)]">
            Appearance
          </h3>
          <div>
            <label
              for="theme"
              class="mb-1 block text-xs font-medium text-[var(--panel-muted-fg)]"
              >Theme</label
            >
            <select
              id="theme"
              bind:value={theme}
              class="w-full rounded-md border border-[var(--input-border)] bg-[var(--input-bg)] px-3 py-2 text-sm text-[var(--app-fg)] focus:border-blue-400 focus:outline-none focus:ring-1 focus:ring-blue-400"
            >
              <option value="light">Light</option>
              <option value="dark">Dark</option>
            </select>
          </div>
        </div>

        <!-- Diff Options -->
        <div>
          <h3 class="mb-3 text-sm font-semibold text-[var(--app-fg)]">
            Diff Options
          </h3>
          <div>
            <label
              for="diff-type"
              class="mb-1 block text-xs font-medium text-[var(--panel-muted-fg)]"
              >Default Diff Type</label
            >
            <select
              id="diff-type"
              bind:value={diffType}
              class="w-full rounded-md border border-[var(--input-border)] bg-[var(--input-bg)] px-3 py-2 text-sm text-[var(--app-fg)] focus:border-blue-400 focus:outline-none focus:ring-1 focus:ring-blue-400"
            >
              <option value="unstaged">Unstaged Changes</option>
              <option value="staged">Staged Changes</option>
            </select>
          </div>
        </div>

        {#if validationError}
          <p class="text-sm text-red-600">{validationError}</p>
        {/if}
      </div>

      <!-- Footer -->
      <div
        class="flex items-center justify-end gap-2 border-t border-[var(--panel-border)] px-6 py-4"
      >
        <button
          onclick={onClose}
          class="rounded-md px-4 py-2 text-sm text-[var(--panel-muted-fg)] hover:bg-[var(--panel-muted-bg)]"
        >
          Cancel
        </button>
        <button
          onclick={handleSave}
          disabled={saving}
          class="rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 disabled:opacity-50"
        >
          {saving ? "Saving..." : "Save"}
        </button>
      </div>
    </div>
  </div>
{/if}
