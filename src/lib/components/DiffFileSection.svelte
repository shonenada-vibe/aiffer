<script lang="ts">
  import type { DiffFile, DiffLine, FileStatus } from "../types/diff";
  import { detectLanguage } from "../utils/highlight";
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
  let language = $derived(detectLanguage(file.path));

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

  function statusBadgeVar(status: FileStatus): string {
    const map: Record<FileStatus, string> = {
      added: "success",
      modified: "warning",
      deleted: "danger",
      renamed: "info",
      copied: "info",
      untracked: "neutral",
    };
    return map[status];
  }
</script>

<section id="diff-file-{file.path}" class="diff-file-section">
  <!-- File header -->
  <button
    class="diff-file-header sticky top-0 z-[1] flex w-full items-center gap-2 px-4 py-2 text-left"
    onclick={toggle}
  >
    <!-- Collapse chevron -->
    <svg
      class="diff-file-icon h-3.5 w-3.5 shrink-0 transition-transform {collapsed
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
      class="diff-file-icon h-4 w-4 shrink-0"
      viewBox="0 0 16 16"
      fill="currentColor"
    >
      <path
        d="M2 1.75C2 .784 2.784 0 3.75 0h6.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v9.586A1.75 1.75 0 0 1 13.25 16h-9.5A1.75 1.75 0 0 1 2 14.25Zm1.75-.25a.25.25 0 0 0-.25.25v12.5c0 .138.112.25.25.25h9.5a.25.25 0 0 0 .25-.25V6h-2.75A1.75 1.75 0 0 1 9 4.25V1.5Zm6.75.062V4.25c0 .138.112.25.25.25h2.688l-.011-.013-2.914-2.914-.013-.011Z"
      />
    </svg>

    <!-- File path -->
    <span class="diff-file-path min-w-0 flex-1 truncate font-mono text-sm">
      {file.path}
      {#if file.oldPath}
        <span class="diff-file-old-path">(was {file.oldPath})</span>
      {/if}
    </span>

    <!-- Status badge -->
    <span
      class="diff-status-badge shrink-0 rounded-full px-2 py-0.5 text-xs font-medium"
      style="background-color: var(--{statusBadgeVar(file.status)}-muted-bg); color: var(--{statusBadgeVar(file.status)}-fg);"
    >
      {statusLabel(file.status)}
    </span>

    <!-- Stats -->
    <span class="flex shrink-0 gap-1.5 text-xs">
      {#if file.stats.additions > 0}
        <span class="font-medium" style="color: var(--success-fg);">+{file.stats.additions}</span>
      {/if}
      {#if file.stats.deletions > 0}
        <span class="font-medium" style="color: var(--danger-fg);">-{file.stats.deletions}</span>
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
              {language}
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

<style>
  .diff-file-section {
    border-bottom: 1px solid var(--panel-border);
  }
  .diff-file-header {
    background-color: var(--panel-muted-bg);
    border-bottom: 1px solid var(--panel-border);
  }
  .diff-file-header:hover {
    background-color: var(--header-hover-bg);
  }
  .diff-file-icon {
    color: var(--panel-faint-fg);
  }
  .diff-file-path {
    color: var(--app-fg);
  }
  .diff-file-old-path {
    color: var(--panel-faint-fg);
  }
</style>
