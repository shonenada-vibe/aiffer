<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    children: Snippet;
  }

  let { children }: Props = $props();

  let error: string | null = $state(null);

  function handleError(e: Event) {
    e.preventDefault();
    const errorEvent = e as ErrorEvent;
    error = errorEvent.message || "An unexpected error occurred";
  }

  function handleUnhandledRejection(e: PromiseRejectionEvent) {
    e.preventDefault();
    const msg =
      e.reason instanceof Error
        ? e.reason.message
        : typeof e.reason === "string"
          ? e.reason
          : "An unexpected error occurred";
    error = msg;
  }

  function dismiss() {
    error = null;
  }

  function reload() {
    window.location.reload();
  }
</script>

<svelte:window onerror={handleError} onunhandledrejection={handleUnhandledRejection} />

{#if error}
  <div class="error-boundary fixed inset-0 z-[300] flex items-center justify-center">
    <div class="error-boundary-card w-full max-w-md rounded-lg p-6 shadow-xl">
      <div class="mb-4 flex items-center gap-3">
        <svg class="error-boundary-icon h-8 w-8 shrink-0" viewBox="0 0 16 16" fill="currentColor">
          <path d="M6.457 1.047c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0 1 14.082 15H1.918a1.75 1.75 0 0 1-1.543-2.575ZM8 5a.75.75 0 0 0-.75.75v2.5a.75.75 0 0 0 1.5 0v-2.5A.75.75 0 0 0 8 5Zm1 6a1 1 0 1 0-2 0 1 1 0 0 0 2 0Z" />
        </svg>
        <h2 class="text-lg font-semibold" style="color: var(--app-fg);">Something went wrong</h2>
      </div>

      <p class="error-boundary-message mb-2 rounded p-3 font-mono text-sm">
        {error}
      </p>

      <p class="error-boundary-hint mb-4 text-sm">
        Try dismissing this error or reloading the app. If the problem persists, check that your git repository is valid and your settings are correct.
      </p>

      <div class="flex gap-2">
        <button
          class="error-boundary-dismiss rounded-md px-4 py-2 text-sm font-medium"
          onclick={dismiss}
        >
          Dismiss
        </button>
        <button
          class="error-boundary-reload rounded-md px-4 py-2 text-sm font-medium text-white"
          onclick={reload}
        >
          Reload App
        </button>
      </div>
    </div>
  </div>
{/if}

{@render children()}

<style>
  .error-boundary {
    background-color: var(--overlay-bg);
  }
  .error-boundary-card {
    background-color: var(--panel-bg);
    border: 1px solid var(--panel-border);
  }
  .error-boundary-icon {
    color: var(--danger-fg);
  }
  .error-boundary-message {
    background-color: var(--danger-muted-bg);
    color: var(--danger-fg);
    word-break: break-word;
  }
  .error-boundary-hint {
    color: var(--panel-muted-fg);
  }
  .error-boundary-dismiss {
    color: var(--btn-secondary-fg);
    border: 1px solid var(--panel-border);
  }
  .error-boundary-dismiss:hover {
    background-color: var(--btn-secondary-hover-bg);
  }
  .error-boundary-reload {
    background-color: var(--btn-danger-bg);
  }
  .error-boundary-reload:hover {
    background-color: var(--btn-danger-hover-bg);
  }
</style>
