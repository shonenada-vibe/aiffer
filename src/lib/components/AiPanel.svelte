<script lang="ts">
  import { marked } from "marked";

  interface Props {
    isOpen: boolean;
    loading: boolean;
    response: string;
    error: string | null;
    onClose: () => void;
    onRetry: () => void;
  }

  let { isOpen, loading, response, error, onClose, onRetry }: Props =
    $props();

  let copied = $state(false);

  let renderedHtml = $derived(response ? marked.parse(response) : "");

  async function copyToClipboard() {
    try {
      await navigator.clipboard.writeText(response);
      copied = true;
      setTimeout(() => (copied = false), 2000);
    } catch {
      // Fallback: noop
    }
  }
</script>

{#if isOpen}
  <aside class="flex w-96 flex-col border-l border-[var(--panel-border)] bg-[var(--panel-muted-bg)]">
    <!-- Panel header -->
    <div
      class="flex h-10 items-center justify-between border-b border-[var(--panel-border)] px-3"
    >
      <span
        class="text-xs font-semibold uppercase tracking-wide text-[var(--panel-muted-fg)]"
      >
        AI Response
      </span>
      <div class="flex items-center gap-1">
        {#if response && !loading}
          <button
            onclick={copyToClipboard}
            class="rounded p-1 text-[var(--panel-muted-fg)] hover:bg-[var(--panel-border-subtle)] hover:text-[var(--app-fg)]"
            title={copied ? "Copied!" : "Copy to clipboard"}
          >
            {#if copied}
              <svg
                class="h-4 w-4"
                style="color: var(--success-fg);"
                viewBox="0 0 16 16"
                fill="currentColor"
              >
                <path
                  d="M13.78 4.22a.75.75 0 0 1 0 1.06l-7.25 7.25a.75.75 0 0 1-1.06 0L2.22 9.28a.751.751 0 0 1 .018-1.042.751.751 0 0 1 1.042-.018L6 10.94l6.72-6.72a.75.75 0 0 1 1.06 0Z"
                />
              </svg>
            {:else}
              <svg
                class="h-4 w-4"
                viewBox="0 0 16 16"
                fill="currentColor"
              >
                <path
                  d="M0 6.75C0 5.784.784 5 1.75 5h1.5a.75.75 0 0 1 0 1.5h-1.5a.25.25 0 0 0-.25.25v7.5c0 .138.112.25.25.25h7.5a.25.25 0 0 0 .25-.25v-1.5a.75.75 0 0 1 1.5 0v1.5A1.75 1.75 0 0 1 9.25 16h-7.5A1.75 1.75 0 0 1 0 14.25Z"
                />
                <path
                  d="M5 1.75C5 .784 5.784 0 6.75 0h7.5C15.216 0 16 .784 16 1.75v7.5A1.75 1.75 0 0 1 14.25 11h-7.5A1.75 1.75 0 0 1 5 9.25Zm1.75-.25a.25.25 0 0 0-.25.25v7.5c0 .138.112.25.25.25h7.5a.25.25 0 0 0 .25-.25v-7.5a.25.25 0 0 0-.25-.25Z"
                />
              </svg>
            {/if}
          </button>
        {/if}
        <button
          onclick={onClose}
          class="rounded p-1 text-[var(--panel-muted-fg)] hover:bg-[var(--panel-border-subtle)] hover:text-[var(--app-fg)]"
          title="Close panel"
        >
          <svg class="h-4 w-4" viewBox="0 0 16 16" fill="currentColor">
            <path
              d="M3.72 3.72a.75.75 0 0 1 1.06 0L8 6.94l3.22-3.22a.749.749 0 0 1 1.275.326.749.749 0 0 1-.215.734L9.06 8l3.22 3.22a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215L8 9.06l-3.22 3.22a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042L6.94 8 3.72 4.78a.75.75 0 0 1 0-1.06Z"
            />
          </svg>
        </button>
      </div>
    </div>

    <!-- Panel content -->
    <div class="flex-1 overflow-y-auto p-4">
      {#if loading}
        <!-- Loading state -->
        <div
          class="flex flex-col items-center justify-center py-12 text-[var(--panel-muted-fg)]"
        >
          <div
            class="ai-spinner mb-3 h-8 w-8 animate-spin rounded-full border-2"
          ></div>
          <span class="text-sm">Waiting for AI response...</span>
          <span class="mt-1 text-xs text-[var(--panel-faint-fg)]"
            >This may take a moment</span
          >
        </div>
      {:else if error}
        <!-- Error state -->
        <div
          class="flex flex-col items-center justify-center py-12"
          style="color: var(--danger-fg);"
        >
          <svg class="mb-3 h-8 w-8" viewBox="0 0 16 16" fill="currentColor">
            <path
              d="M6.457 1.047c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0 1 14.082 15H1.918a1.75 1.75 0 0 1-1.543-2.575ZM8 5a.75.75 0 0 0-.75.75v2.5a.75.75 0 0 0 1.5 0v-2.5A.75.75 0 0 0 8 5Zm1 6a1 1 0 1 0-2 0 1 1 0 0 0 2 0Z"
            />
          </svg>
          <p class="mb-1 text-sm font-medium">
            AI request failed
          </p>
          <p class="mb-3 text-center text-xs" style="opacity: 0.8;">{error}</p>
          <button
            class="rounded-md px-3 py-1.5 text-sm font-medium text-white"
            style="background-color: var(--btn-danger-bg);"
            onclick={onRetry}
          >
            Retry
          </button>
        </div>
      {:else if response}
        <!-- Rendered markdown response -->
        <div
          class="prose prose-sm max-w-none text-[var(--app-fg)] prose-headings:text-[var(--app-fg)] prose-h1:text-lg prose-h2:text-base prose-h3:text-sm prose-p:leading-relaxed prose-code:rounded prose-code:bg-[var(--panel-border-subtle)] prose-code:px-1 prose-code:py-0.5 prose-code:text-xs prose-code:text-[var(--app-fg)] prose-code:before:content-none prose-code:after:content-none prose-pre:bg-[var(--panel-muted-bg)] prose-pre:text-[var(--app-fg)]"
        >
          {@html renderedHtml}
        </div>
      {:else}
        <!-- Empty state -->
        <div
          class="flex flex-col items-center justify-center py-12 text-[var(--panel-muted-fg)]"
        >
          <svg class="mb-2 h-8 w-8" viewBox="0 0 16 16" fill="currentColor">
            <path
              d="M5.433 2.304A4.494 4.494 0 0 0 3.5 6c0 1.598.564 3.05 1.47 4.235a6.517 6.517 0 0 0-1.97.77.75.75 0 1 0 .782 1.283c.628-.383 1.37-.662 2.218-.79a4.478 4.478 0 0 0 4 0c.848.128 1.59.407 2.218.79a.75.75 0 1 0 .782-1.283 6.517 6.517 0 0 0-1.97-.77A7.486 7.486 0 0 0 12.5 6a4.494 4.494 0 0 0-1.933-3.696A4.484 4.484 0 0 0 8 1.5a4.484 4.484 0 0 0-2.567.804ZM10 8.5a2 2 0 1 1-4 0 2 2 0 0 1 4 0Z"
            />
          </svg>
          <span class="text-sm">Submit a review to see AI response</span>
        </div>
      {/if}
    </div>
  </aside>
{/if}

<style>
  .ai-spinner {
    border-color: var(--panel-border);
    border-top-color: var(--accent-fg);
  }
</style>
