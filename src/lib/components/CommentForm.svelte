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
  <td colspan="4" class="comment-form-cell p-0">
    <div class="comment-form-card mx-4 my-2 rounded-md shadow-sm">
      <div class="p-3">
        <textarea
          bind:this={textareaEl}
          bind:value={body}
          class="comment-form-textarea w-full resize-y rounded px-3 py-2 font-sans text-sm focus:outline-none focus:ring-1"
          rows="3"
          placeholder="Leave a comment..."
          onkeydown={handleKeydown}
        ></textarea>
      </div>
      <div class="comment-form-footer flex items-center justify-between rounded-b-md px-3 py-2">
        <span class="comment-form-hint text-xs">
          <kbd class="comment-form-kbd rounded px-1 py-0.5 font-mono text-[10px]">
            {navigator.platform.includes("Mac") ? "Cmd" : "Ctrl"}+Enter
          </kbd>
          to submit,
          <kbd class="comment-form-kbd rounded px-1 py-0.5 font-mono text-[10px]">
            Esc
          </kbd>
          to cancel
        </span>
        <div class="flex gap-2">
          <button
            class="comment-form-cancel rounded px-3 py-1 text-sm"
            onclick={onCancel}
          >
            Cancel
          </button>
          <button
            class="comment-form-submit rounded px-3 py-1 text-sm font-medium text-white disabled:cursor-not-allowed disabled:opacity-50"
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

<style>
  .comment-form-cell {
    border-bottom: 1px solid var(--panel-border);
  }
  .comment-form-card {
    border: 1px solid var(--panel-border);
    background-color: var(--panel-bg);
  }
  .comment-form-textarea {
    border: 1px solid var(--input-border);
    background-color: var(--input-muted-bg);
    color: var(--app-fg);
  }
  .comment-form-textarea::placeholder {
    color: var(--panel-faint-fg);
  }
  .comment-form-textarea:focus {
    border-color: var(--focus-ring);
    background-color: var(--input-bg);
    box-shadow: 0 0 0 1px var(--focus-ring);
  }
  .comment-form-footer {
    border-top: 1px solid var(--panel-border-subtle);
    background-color: var(--input-muted-bg);
  }
  .comment-form-hint {
    color: var(--panel-faint-fg);
  }
  .comment-form-kbd {
    border: 1px solid var(--panel-border);
    background-color: var(--panel-bg);
  }
  .comment-form-cancel {
    color: var(--btn-secondary-fg);
  }
  .comment-form-cancel:hover {
    background-color: var(--btn-secondary-hover-bg);
  }
  .comment-form-submit {
    background-color: var(--btn-primary-bg);
  }
  .comment-form-submit:hover {
    background-color: var(--btn-primary-hover-bg);
  }
</style>
