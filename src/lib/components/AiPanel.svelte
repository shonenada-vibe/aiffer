<script lang="ts">
  interface Props {
    isOpen: boolean;
    onClose: () => void;
  }

  let { isOpen, onClose }: Props = $props();
</script>

{#if isOpen}
  <aside
    class="flex w-96 flex-col border-l border-gray-200 bg-gray-50"
  >
    <!-- Panel header -->
    <div
      class="flex h-10 items-center justify-between border-b border-gray-200 px-3"
    >
      <span
        class="text-xs font-semibold uppercase tracking-wide text-gray-500"
      >
        AI Response
      </span>
      <button
        onclick={onClose}
        class="rounded p-1 text-gray-400 hover:bg-gray-200 hover:text-gray-600"
        title="Close panel"
      >
        <svg class="h-4 w-4" viewBox="0 0 16 16" fill="currentColor">
          <path
            d="M3.72 3.72a.75.75 0 0 1 1.06 0L8 6.94l3.22-3.22a.749.749 0 0 1 1.275.326.749.749 0 0 1-.215.734L9.06 8l3.22 3.22a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215L8 9.06l-3.22 3.22a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042L6.94 8 3.72 4.78a.75.75 0 0 1 0-1.06Z"
          />
        </svg>
      </button>
    </div>

    <!-- Panel content placeholder -->
    <div class="flex flex-1 flex-col items-center justify-center p-4 text-gray-400">
      <svg class="mb-2 h-8 w-8" viewBox="0 0 16 16" fill="currentColor">
        <path
          d="M5.433 2.304A4.494 4.494 0 0 0 3.5 6c0 1.598.564 3.05 1.47 4.235a6.517 6.517 0 0 0-1.97.77.75.75 0 1 0 .782 1.283c.628-.383 1.37-.662 2.218-.79a4.478 4.478 0 0 0 4 0c.848.128 1.59.407 2.218.79a.75.75 0 1 0 .782-1.283 6.517 6.517 0 0 0-1.97-.77A7.486 7.486 0 0 0 12.5 6a4.494 4.494 0 0 0-1.933-3.696A4.484 4.484 0 0 0 8 1.5a4.484 4.484 0 0 0-2.567.804ZM10 8.5a2 2 0 1 1-4 0 2 2 0 0 1 4 0Z"
        />
      </svg>
      <span class="text-sm">Submit a review to see AI response</span>
    </div>
  </aside>
{/if}
