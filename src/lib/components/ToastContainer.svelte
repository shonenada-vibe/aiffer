<script lang="ts">
  import { toastStore } from "../stores/toast.svelte";
  import type { ToastType } from "../stores/toast.svelte";

  function iconPath(type: ToastType): string {
    switch (type) {
      case "success":
        return "M13.78 4.22a.75.75 0 0 1 0 1.06l-7.25 7.25a.75.75 0 0 1-1.06 0L2.22 9.28a.751.751 0 0 1 .018-1.042.751.751 0 0 1 1.042-.018L6 10.94l6.72-6.72a.75.75 0 0 1 1.06 0Z";
      case "error":
        return "M3.72 3.72a.75.75 0 0 1 1.06 0L8 6.94l3.22-3.22a.749.749 0 0 1 1.275.326.749.749 0 0 1-.215.734L9.06 8l3.22 3.22a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215L8 9.06l-3.22 3.22a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042L6.94 8 3.72 4.78a.75.75 0 0 1 0-1.06Z";
      case "warning":
        return "M6.457 1.047c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0 1 14.082 15H1.918a1.75 1.75 0 0 1-1.543-2.575ZM8 5a.75.75 0 0 0-.75.75v2.5a.75.75 0 0 0 1.5 0v-2.5A.75.75 0 0 0 8 5Zm1 6a1 1 0 1 0-2 0 1 1 0 0 0 2 0Z";
      case "info":
        return "M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8Zm8-6.5a6.5 6.5 0 1 0 0 13 6.5 6.5 0 0 0 0-13ZM6.5 7.75A.75.75 0 0 1 7.25 7h1a.75.75 0 0 1 .75.75v2.75h.25a.75.75 0 0 1 0 1.5h-2a.75.75 0 0 1 0-1.5h.25v-2h-.25a.75.75 0 0 1-.75-.75ZM8 6a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z";
    }
  }
</script>

{#if toastStore.toasts.length > 0}
  <div class="fixed top-14 left-4 z-[200] flex flex-col gap-2">
    {#each toastStore.toasts as toast (toast.id)}
      <div
        class="toast-item toast-{toast.type} flex items-start gap-2 rounded-lg px-4 py-3 shadow-lg"
        role="alert"
      >
        <svg class="toast-icon mt-0.5 h-4 w-4 shrink-0" viewBox="0 0 16 16" fill="currentColor">
          <path d={iconPath(toast.type)} />
        </svg>
        <span class="text-sm">{toast.message}</span>
        <button
          class="toast-close ml-2 shrink-0 rounded p-0.5 opacity-60 hover:opacity-100"
          onclick={() => toastStore.dismiss(toast.id)}
          title="Dismiss"
        >
          <svg class="h-3 w-3" viewBox="0 0 16 16" fill="currentColor">
            <path d="M3.72 3.72a.75.75 0 0 1 1.06 0L8 6.94l3.22-3.22a.749.749 0 0 1 1.275.326.749.749 0 0 1-.215.734L9.06 8l3.22 3.22a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215L8 9.06l-3.22 3.22a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042L6.94 8 3.72 4.78a.75.75 0 0 1 0-1.06Z" />
          </svg>
        </button>
      </div>
    {/each}
  </div>
{/if}

<style>
  .toast-item {
    min-width: 280px;
    max-width: 420px;
    border: 1px solid var(--panel-border);
    animation: toast-slide-in 0.2s ease-out;
  }
  @keyframes toast-slide-in {
    from {
      opacity: 0;
      transform: translateX(-20px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }
  .toast-success {
    background-color: var(--success-bg);
    color: var(--success-fg);
    border-color: var(--success-fg);
  }
  .toast-success .toast-icon {
    color: var(--success-fg);
  }
  .toast-error {
    background-color: var(--danger-bg);
    color: var(--danger-fg);
    border-color: var(--danger-fg);
  }
  .toast-error .toast-icon {
    color: var(--danger-fg);
  }
  .toast-warning {
    background-color: var(--warning-bg);
    color: var(--warning-fg);
    border-color: var(--warning-fg);
  }
  .toast-warning .toast-icon {
    color: var(--warning-fg);
  }
  .toast-info {
    background-color: var(--info-bg);
    color: var(--info-fg);
    border-color: var(--info-fg);
  }
  .toast-info .toast-icon {
    color: var(--info-fg);
  }
</style>
