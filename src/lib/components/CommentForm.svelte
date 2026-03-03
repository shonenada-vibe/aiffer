<script lang="ts">
  interface Props {
    onSubmit: (body: string) => void;
    onCancel: () => void;
  }

  let { onSubmit, onCancel }: Props = $props();

  let body = $state("");
  let textareaEl: HTMLTextAreaElement | undefined = $state();

  $effect(() => {
    textareaEl?.focus();
  });

  function handleSubmit() {
    const trimmed = body.trim();
    if (!trimmed) return;
    onSubmit(trimmed);
    body = "";
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      e.preventDefault();
      onCancel();
    }
    if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      handleSubmit();
    }
  }
</script>

<tr>
  <td colspan="4" class="border-b border-gray-200 p-0">
    <div
      class="mx-4 my-2 rounded-md border border-gray-300 bg-white shadow-sm"
    >
      <div class="p-3">
        <textarea
          bind:this={textareaEl}
          bind:value={body}
          class="w-full resize-y rounded border border-gray-200 bg-gray-50 px-3 py-2 font-sans text-sm text-gray-800 placeholder-gray-400 focus:border-blue-400 focus:bg-white focus:outline-none focus:ring-1 focus:ring-blue-400"
          rows="3"
          placeholder="Leave a comment..."
          onkeydown={handleKeydown}
        ></textarea>
      </div>
      <div
        class="flex items-center justify-between rounded-b-md border-t border-gray-100 bg-gray-50 px-3 py-2"
      >
        <span class="text-xs text-gray-400">
          <kbd
            class="rounded border border-gray-300 bg-white px-1 py-0.5 font-mono text-[10px]"
          >
            {navigator.platform.includes("Mac") ? "Cmd" : "Ctrl"}+Enter
          </kbd>
          to submit,
          <kbd
            class="rounded border border-gray-300 bg-white px-1 py-0.5 font-mono text-[10px]"
          >
            Esc
          </kbd>
          to cancel
        </span>
        <div class="flex gap-2">
          <button
            class="rounded px-3 py-1 text-sm text-gray-600 hover:bg-gray-200"
            onclick={onCancel}
          >
            Cancel
          </button>
          <button
            class="rounded bg-green-600 px-3 py-1 text-sm font-medium text-white hover:bg-green-700 disabled:cursor-not-allowed disabled:opacity-50"
            onclick={handleSubmit}
            disabled={body.trim().length === 0}
          >
            Add Comment
          </button>
        </div>
      </div>
    </div>
  </td>
</tr>
