<script lang="ts">
  import type { Comment } from "../types/comment";
  import { commentStore } from "../stores/comments.svelte";

  interface Props {
    comments: Comment[];
  }

  let { comments }: Props = $props();

  let showAll = $state(false);
  let editingId: string | null = $state(null);
  let editBody = $state("");
  let deletingId: string | null = $state(null);

  let visibleComments = $derived(
    comments.length > 3 && !showAll ? comments.slice(0, 2) : comments,
  );
  let hiddenCount = $derived(
    comments.length > 3 && !showAll ? comments.length - 2 : 0,
  );

  function startEdit(comment: Comment) {
    editingId = comment.id;
    editBody = comment.body;
  }

  function cancelEdit() {
    editingId = null;
    editBody = "";
  }

  function saveEdit() {
    if (editingId && editBody.trim()) {
      commentStore.editComment(editingId, editBody.trim());
      editingId = null;
      editBody = "";
    }
  }

  function confirmDelete(commentId: string) {
    deletingId = commentId;
  }

  function cancelDelete() {
    deletingId = null;
  }

  function executeDelete() {
    if (deletingId) {
      commentStore.removeComment(deletingId);
      deletingId = null;
    }
  }

  function formatTime(isoString: string): string {
    const date = new Date(isoString);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMin = Math.floor(diffMs / 60000);
    if (diffMin < 1) return "just now";
    if (diffMin < 60) return `${diffMin}m ago`;
    const diffHrs = Math.floor(diffMin / 60);
    if (diffHrs < 24) return `${diffHrs}h ago`;
    const diffDays = Math.floor(diffHrs / 24);
    return `${diffDays}d ago`;
  }
</script>

<tr>
  <td colspan="4" class="comment-thread-cell p-0">
    <div class="mx-4 my-1 space-y-1">
      {#each visibleComments as comment (comment.id)}
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="group/comment comment-card rounded-r">
          <div class="flex items-start gap-2 px-3 py-2">
            <!-- Avatar placeholder -->
            <div class="comment-avatar flex h-6 w-6 shrink-0 items-center justify-center rounded-full">
              <svg
                class="comment-avatar-icon h-3.5 w-3.5"
                viewBox="0 0 16 16"
                fill="currentColor"
              >
                <path
                  d="M10.561 8.073a6.005 6.005 0 0 1 3.432 5.142.75.75 0 1 1-1.498.07 4.5 4.5 0 0 0-8.99 0 .75.75 0 0 1-1.498-.07 6.004 6.004 0 0 1 3.431-5.142 3.999 3.999 0 1 1 5.123 0ZM10.5 5a2.5 2.5 0 1 0-5 0 2.5 2.5 0 0 0 5 0Z"
                />
              </svg>
            </div>

            <div class="min-w-0 flex-1">
              <!-- Header: name + time -->
              <div class="flex items-center gap-2 text-xs">
                <span class="comment-author font-medium">You</span>
                <span class="comment-time">{formatTime(comment.createdAt)}</span>
              </div>

              {#if editingId === comment.id}
                <!-- Edit mode -->
                <textarea
                  bind:value={editBody}
                  class="comment-edit-textarea mt-1 w-full resize-y rounded px-2 py-1 text-sm focus:outline-none focus:ring-1"
                  rows="2"
                  onkeydown={(e) => {
                    if (e.key === "Escape") cancelEdit();
                    if (e.key === "Enter" && (e.metaKey || e.ctrlKey))
                      saveEdit();
                  }}
                ></textarea>
                <div class="mt-1 flex gap-1">
                  <button
                    class="comment-save-btn rounded px-2 py-0.5 text-xs text-white"
                    onclick={saveEdit}
                  >
                    Save
                  </button>
                  <button
                    class="comment-cancel-btn rounded px-2 py-0.5 text-xs"
                    onclick={cancelEdit}
                  >
                    Cancel
                  </button>
                </div>
              {:else}
                <!-- Display mode -->
                <p class="comment-body mt-0.5 whitespace-pre-wrap text-sm">
                  {comment.body}
                </p>
              {/if}
            </div>

            <!-- Actions -->
            {#if editingId !== comment.id}
              {#if deletingId === comment.id}
                <div class="flex shrink-0 items-center gap-1">
                  <button
                    class="comment-delete-confirm rounded px-2 py-0.5 text-xs text-white"
                    onclick={executeDelete}
                  >
                    Delete
                  </button>
                  <button
                    class="comment-cancel-btn rounded px-2 py-0.5 text-xs"
                    onclick={cancelDelete}
                  >
                    Cancel
                  </button>
                </div>
              {:else}
                <div
                  class="flex shrink-0 items-center gap-1 opacity-0 transition-opacity group-hover/comment:opacity-100"
                >
                  <button
                    class="comment-action-btn rounded p-1"
                    title="Edit"
                    onclick={() => startEdit(comment)}
                  >
                    <svg
                      class="h-3 w-3"
                      viewBox="0 0 16 16"
                      fill="currentColor"
                    >
                      <path
                        d="M11.013 1.427a1.75 1.75 0 0 1 2.474 0l1.086 1.086a1.75 1.75 0 0 1 0 2.474l-8.61 8.61c-.21.21-.47.364-.756.445l-3.251.93a.75.75 0 0 1-.927-.928l.929-3.25c.081-.286.235-.547.445-.758l8.61-8.61Zm.176 4.823L9.75 4.81l-6.286 6.287a.253.253 0 0 0-.064.108l-.558 1.953 1.953-.558a.253.253 0 0 0 .108-.064Zm1.238-3.763a.25.25 0 0 0-.354 0L10.811 3.75l1.439 1.44 1.263-1.263a.25.25 0 0 0 0-.354Z"
                      />
                    </svg>
                  </button>
                  <button
                    class="comment-action-btn-danger rounded p-1"
                    title="Delete"
                    onclick={() => confirmDelete(comment.id)}
                  >
                    <svg
                      class="h-3 w-3"
                      viewBox="0 0 16 16"
                      fill="currentColor"
                    >
                      <path
                        d="M11 1.75V3h2.25a.75.75 0 0 1 0 1.5H2.75a.75.75 0 0 1 0-1.5H5V1.75C5 .784 5.784 0 6.75 0h2.5C10.216 0 11 .784 11 1.75ZM4.496 6.675l.66 6.6a.25.25 0 0 0 .249.225h5.19a.25.25 0 0 0 .249-.225l.66-6.6a.75.75 0 0 1 1.492.149l-.66 6.6A1.748 1.748 0 0 1 10.595 15h-5.19a1.75 1.75 0 0 1-1.741-1.575l-.66-6.6a.75.75 0 1 1 1.492-.15ZM6.5 1.75V3h3V1.75a.25.25 0 0 0-.25-.25h-2.5a.25.25 0 0 0-.25.25Z"
                      />
                    </svg>
                  </button>
                </div>
              {/if}
            {/if}
          </div>
        </div>
      {/each}

      {#if hiddenCount > 0}
        <button
          class="comment-show-more w-full rounded py-1 text-center text-xs"
          onclick={() => (showAll = true)}
        >
          Show {hiddenCount} more comment{hiddenCount !== 1 ? "s" : ""}
        </button>
      {/if}
    </div>
  </td>
</tr>

<style>
  .comment-thread-cell {
    border-bottom: 1px solid var(--panel-border);
  }
  .comment-card {
    border-left: 2px solid var(--accent-fg);
    background-color: var(--panel-muted-bg);
  }
  .comment-avatar {
    background-color: var(--panel-border);
  }
  .comment-avatar-icon {
    color: var(--panel-muted-fg);
  }
  .comment-author {
    color: var(--app-fg);
  }
  .comment-time {
    color: var(--panel-faint-fg);
  }
  .comment-body {
    color: var(--app-fg);
  }
  .comment-edit-textarea {
    border: 1px solid var(--input-border);
    background-color: var(--input-bg);
    color: var(--app-fg);
  }
  .comment-edit-textarea:focus {
    border-color: var(--focus-ring);
  }
  .comment-save-btn {
    background-color: var(--btn-primary-bg);
  }
  .comment-save-btn:hover {
    background-color: var(--btn-primary-hover-bg);
  }
  .comment-cancel-btn {
    color: var(--btn-secondary-fg);
  }
  .comment-cancel-btn:hover {
    background-color: var(--btn-secondary-hover-bg);
  }
  .comment-delete-confirm {
    background-color: var(--btn-danger-bg);
  }
  .comment-delete-confirm:hover {
    background-color: var(--btn-danger-hover-bg);
  }
  .comment-action-btn {
    color: var(--panel-faint-fg);
  }
  .comment-action-btn:hover {
    background-color: var(--btn-secondary-hover-bg);
    color: var(--app-fg);
  }
  .comment-action-btn-danger {
    color: var(--panel-faint-fg);
  }
  .comment-action-btn-danger:hover {
    background-color: var(--danger-muted-bg);
    color: var(--danger-fg);
  }
  .comment-show-more {
    color: var(--accent-fg);
  }
  .comment-show-more:hover {
    background-color: var(--accent-muted-bg);
  }
</style>
