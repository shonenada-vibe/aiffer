<script lang="ts">
  import type { DiffFile, DiffLine, FileStatus } from "../types/diff";
  import DiffHunkComponent from "./DiffHunk.svelte";

  interface Props {
    file: DiffFile;
    forceCollapsed: boolean | null;
    highlightedLineKey: string | null;
    activeCommentKey: string | null;
    onClickLine: (filePath: string, line: DiffLine) => void;
    onClickComment: (filePath: string, line: DiffLine) => void;
    onSubmitComment: (filePath: string, line: DiffLine, body: string) => void;
    onCancelComment: () => void;
  }

  let {
    file,
    forceCollapsed,
    highlightedLineKey,
    activeCommentKey,
    onClickLine,
    onClickComment,
    onSubmitComment,
    onCancelComment,
  }: Props = $props();

  let localCollapsed = $state(false);

  let collapsed = $derived(forceCollapsed !== null ? forceCollapsed : localCollapsed);

  function toggle() {
    localCollapsed = !localCollapsed;
  }

  function statusLabel(status: FileStatus): string {
    const map: Record<FileStatus, string> = {
      added: "Added",
      modified: "Modified",
      deleted: "Deleted",
      renamed: "Renamed",
      copied: "Copied",
      untracked: "Untracked",
    };
    return map[status];
  }

  function statusBadgeClass(status: FileStatus): string {
    const map: Record<FileStatus, string> = {
      added: "bg-green-100 text-green-700",
      modified: "bg-yellow-100 text-yellow-700",
      deleted: "bg-red-100 text-red-700",
      renamed: "bg-blue-100 text-blue-700",
      copied: "bg-blue-100 text-blue-700",
      untracked: "bg-gray-100 text-gray-600",
    };
    return map[status];
  }
</script>

<section id="diff-file-{file.path}" class="border-b border-gray-200">
  <!-- File header -->
  <button
    class="sticky top-0 z-[1] flex w-full items-center gap-2 border-b border-gray-200 bg-gray-50 px-4 py-2 text-left hover:bg-gray-100"
    onclick={toggle}
  >
    <!-- Collapse chevron -->
    <svg
      class="h-3.5 w-3.5 shrink-0 text-gray-500 transition-transform {collapsed
        ? '-rotate-90'
        : ''}"
      viewBox="0 0 16 16"
      fill="currentColor"
    >
      <path
        d="M12.78 5.22a.749.749 0 0 1 0 1.06l-4.25 4.25a.749.749 0 0 1-1.06 0L3.22 6.28a.749.749 0 1 1 1.06-1.06L8 8.939l3.72-3.719a.749.749 0 0 1 1.06 0Z"
      />
    </svg>

    <!-- File icon -->
    <svg
      class="h-4 w-4 shrink-0 text-gray-400"
      viewBox="0 0 16 16"
      fill="currentColor"
    >
      <path
        d="M2 1.75C2 .784 2.784 0 3.75 0h6.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v9.586A1.75 1.75 0 0 1 13.25 16h-9.5A1.75 1.75 0 0 1 2 14.25Zm1.75-.25a.25.25 0 0 0-.25.25v12.5c0 .138.112.25.25.25h9.5a.25.25 0 0 0 .25-.25V6h-2.75A1.75 1.75 0 0 1 9 4.25V1.5Zm6.75.062V4.25c0 .138.112.25.25.25h2.688l-.011-.013-2.914-2.914-.013-.011Z"
      />
    </svg>

    <!-- File path -->
    <span class="min-w-0 flex-1 truncate font-mono text-sm text-gray-800">
      {file.path}
      {#if file.oldPath}
        <span class="text-gray-400">(was {file.oldPath})</span>
      {/if}
    </span>

    <!-- Status badge -->
    <span
      class="shrink-0 rounded-full px-2 py-0.5 text-xs font-medium {statusBadgeClass(file.status)}"
    >
      {statusLabel(file.status)}
    </span>

    <!-- Stats -->
    <span class="flex shrink-0 gap-1.5 text-xs">
      {#if file.stats.additions > 0}
        <span class="font-medium text-green-600">+{file.stats.additions}</span>
      {/if}
      {#if file.stats.deletions > 0}
        <span class="font-medium text-red-600">-{file.stats.deletions}</span>
      {/if}
    </span>
  </button>

  <!-- Diff content -->
  {#if !collapsed}
    <div class="overflow-x-auto">
      <table class="w-full border-collapse">
        <tbody>
          {#each file.hunks as hunk, i (i)}
            <DiffHunkComponent
              {hunk}
              filePath={file.path}
              {highlightedLineKey}
              {activeCommentKey}
              {onClickLine}
              {onClickComment}
              {onSubmitComment}
              {onCancelComment}
            />
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</section>
