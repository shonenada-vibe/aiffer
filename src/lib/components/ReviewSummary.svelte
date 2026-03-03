<script lang="ts">
  import { commentStore } from "../stores/comments.svelte";
  import { diffStore } from "../stores/diff.svelte";
  import type { Comment } from "../types/comment";

  interface Props {
    isOpen: boolean;
    onClose: () => void;
    onSubmitToAi: () => void;
  }

  let { isOpen, onClose, onSubmitToAi }: Props = $props();

  let confirmClear = $state(false);

  interface CommentGroup {
    filePath: string;
    comments: Comment[];
  }

  let grouped = $derived.by(() => {
    const map = new Map<string, Comment[]>();
    for (const comment of commentStore.allComments) {
      const existing = map.get(comment.filePath) ?? [];
      existing.push(comment);
      map.set(comment.filePath, existing);
    }
    const groups: CommentGroup[] = [];
    for (const [filePath, comments] of map) {
      groups.push({ filePath, comments });
    }
    return groups;
  });

  function getLineContent(
    filePath: string,
    lineNumber: number,
  ): string | null {
    const file = diffStore.diffFiles.find((f) => f.path === filePath);
    if (!file) return null;
    for (const hunk of file.hunks) {
      for (const line of hunk.lines) {
        const ln = line.newLineno ?? line.oldLineno ?? 0;
        if (ln === lineNumber) {
          return line.content;
        }
      }
    }
    return null;
  }

  function scrollToComment(comment: Comment) {
    const el = document.getElementById(`diff-file-${CSS.escape(comment.filePath)}`);
    if (el) {
      el.scrollIntoView({ behavior: "smooth", block: "start" });
    }
  }

  function handleClearAll() {
    commentStore.clearAll();
    confirmClear = false;
  }
</script>

{#if isOpen}
  <aside class="flex w-96 flex-col border-l border-gray-200 bg-white">
    <!-- Panel header -->
    <div
      class="flex h-10 items-center justify-between border-b border-gray-200 bg-gray-50 px-3"
    >
      <div class="flex items-center gap-2">
        <span
          class="text-xs font-semibold uppercase tracking-wide text-gray-500"
        >
          Review Summary
        </span>
        {#if commentStore.commentCount > 0}
          <span
            class="rounded-full bg-blue-100 px-1.5 py-0.5 text-xs font-medium leading-none text-blue-700"
          >
            {commentStore.commentCount}
          </span>
        {/if}
      </div>
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

    <!-- Panel content -->
    <div class="flex-1 overflow-y-auto">
      {#if commentStore.commentCount === 0}
        <!-- Empty state -->
        <div
          class="flex flex-col items-center justify-center px-4 py-12 text-gray-400"
        >
          <svg class="mb-3 h-10 w-10" viewBox="0 0 16 16" fill="currentColor">
            <path
              d="M1 2.75C1 1.784 1.784 1 2.75 1h10.5c.966 0 1.75.784 1.75 1.75v7.5A1.75 1.75 0 0 1 13.25 12H9.06l-2.573 2.573A1.458 1.458 0 0 1 4 13.543V12H2.75A1.75 1.75 0 0 1 1 10.25Zm1.75-.25a.25.25 0 0 0-.25.25v7.5c0 .138.112.25.25.25h2a.75.75 0 0 1 .75.75v2.19l2.72-2.72a.749.749 0 0 1 .53-.22h4.5a.25.25 0 0 0 .25-.25v-7.5a.25.25 0 0 0-.25-.25Z"
            />
          </svg>
          <p class="text-sm font-medium text-gray-600">No comments yet</p>
          <p class="mt-1 text-center text-xs text-gray-400">
            Click the + icon on diff lines to add review comments
          </p>
        </div>
      {:else}
        <div class="divide-y divide-gray-100">
          {#each grouped as group (group.filePath)}
            <!-- File group -->
            <div class="py-2">
              <!-- File header -->
              <div class="flex items-center gap-2 px-3 py-1">
                <svg
                  class="h-3.5 w-3.5 shrink-0 text-gray-400"
                  viewBox="0 0 16 16"
                  fill="currentColor"
                >
                  <path
                    d="M2 1.75C2 .784 2.784 0 3.75 0h6.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v9.586A1.75 1.75 0 0 1 13.25 16h-9.5A1.75 1.75 0 0 1 2 14.25Zm1.75-.25a.25.25 0 0 0-.25.25v12.5c0 .138.112.25.25.25h9.5a.25.25 0 0 0 .25-.25V6h-2.75A1.75 1.75 0 0 1 9 4.25V1.5Zm6.75.062V4.25c0 .138.112.25.25.25h2.688l-.011-.013-2.914-2.914-.013-.011Z"
                  />
                </svg>
                <span
                  class="min-w-0 flex-1 truncate font-mono text-xs font-medium text-gray-700"
                  title={group.filePath}
                >
                  {group.filePath}
                </span>
                <span class="text-xs text-gray-400">
                  {group.comments.length}
                </span>
              </div>

              <!-- Comments in this file -->
              <div class="mt-1 space-y-1 px-3">
                {#each group.comments as comment (comment.id)}
                  {@const lineContent = getLineContent(comment.filePath, comment.lineNumber)}
                  <button
                    class="w-full rounded border border-gray-200 bg-gray-50 p-2 text-left transition-colors hover:border-blue-300 hover:bg-blue-50"
                    onclick={() => scrollToComment(comment)}
                    title="Click to scroll to this comment"
                  >
                    <div class="flex items-center gap-2 text-xs text-gray-500">
                      <span class="font-mono">L{comment.lineNumber}</span>
                      <span
                        class="rounded px-1 py-0.5 text-[10px] font-medium {comment.lineType === 'addition'
                          ? 'bg-green-100 text-green-700'
                          : comment.lineType === 'deletion'
                            ? 'bg-red-100 text-red-700'
                            : 'bg-gray-100 text-gray-600'}"
                      >
                        {comment.lineType === "addition"
                          ? "+"
                          : comment.lineType === "deletion"
                            ? "-"
                            : "ctx"}
                      </span>
                    </div>
                    {#if lineContent}
                      <div
                        class="mt-1 truncate rounded bg-gray-100 px-1.5 py-0.5 font-mono text-[11px] text-gray-600"
                      >
                        {lineContent}
                      </div>
                    {/if}
                    <p
                      class="mt-1 line-clamp-2 text-sm text-gray-800"
                    >
                      {comment.body}
                    </p>
                  </button>
                {/each}
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Panel footer -->
    {#if commentStore.commentCount > 0}
      <div
        class="flex items-center justify-between border-t border-gray-200 bg-gray-50 px-3 py-2"
      >
        {#if confirmClear}
          <div class="flex items-center gap-2">
            <span class="text-xs text-red-600">Clear all comments?</span>
            <button
              class="rounded bg-red-600 px-2 py-1 text-xs text-white hover:bg-red-700"
              onclick={handleClearAll}
            >
              Confirm
            </button>
            <button
              class="rounded px-2 py-1 text-xs text-gray-600 hover:bg-gray-200"
              onclick={() => (confirmClear = false)}
            >
              Cancel
            </button>
          </div>
        {:else}
          <button
            class="rounded px-2 py-1 text-xs text-red-500 hover:bg-red-50 hover:text-red-700"
            onclick={() => (confirmClear = true)}
          >
            Clear All
          </button>
        {/if}
        {#if !confirmClear}
          <button
            class="rounded-md bg-green-600 px-3 py-1.5 text-sm font-medium text-white hover:bg-green-700"
            onclick={onSubmitToAi}
          >
            Submit to AI
          </button>
        {/if}
      </div>
    {/if}
  </aside>
{/if}
