<script lang="ts">
  import type { DiffFile, DiffLine } from "../types/diff";
  import DiffFileSection from "./DiffFileSection.svelte";

  interface Props {
    hasFolder: boolean;
    diffFiles: DiffFile[];
    loading: boolean;
    error: string | null;
    totalAdditions: number;
    totalDeletions: number;
    onSubmitComment: (filePath: string, line: DiffLine, body: string) => void;
  }

  let {
    hasFolder,
    diffFiles,
    loading,
    error,
    totalAdditions,
    totalDeletions,
    onSubmitComment,
  }: Props = $props();

  // null = individual control, true/false = force all
  let forceCollapsed: boolean | null = $state(null);
  let highlightedLineKey: string | null = $state(null);
  let activeCommentKey: string | null = $state(null);

  function lineKey(filePath: string, line: DiffLine): string {
    return `${filePath}:${line.oldLineno ?? ""}:${line.newLineno ?? ""}`;
  }

  function collapseAll() {
    forceCollapsed = true;
  }

  function expandAll() {
    forceCollapsed = false;
  }

  function resetForce() {
    forceCollapsed = null;
  }

  function handleClickLine(filePath: string, line: DiffLine) {
    const key = lineKey(filePath, line);
    highlightedLineKey = highlightedLineKey === key ? null : key;
  }

  function handleClickComment(filePath: string, line: DiffLine) {
    const key = lineKey(filePath, line);
    activeCommentKey = activeCommentKey === key ? null : key;
  }

  function handleSubmitComment(filePath: string, line: DiffLine, body: string) {
    onSubmitComment(filePath, line, body);
    activeCommentKey = null;
  }

  function handleCancelComment() {
    activeCommentKey = null;
  }
</script>

<main class="diff-content flex flex-1 flex-col overflow-auto">
  {#if !hasFolder}
    <!-- Empty state: no folder -->
    <div
      class="diff-empty-state flex flex-1 flex-col items-center justify-center"
    >
      <svg class="mb-4 h-16 w-16" viewBox="0 0 16 16" fill="currentColor">
        <path
          d="M8.75 1.75V5a.75.75 0 0 1-.75.75H4.75v8.5h6.5v-2.5a.75.75 0 1 1 1.5 0v2.5A1.75 1.75 0 0 1 11.25 16h-6.5A1.75 1.75 0 0 1 3 14.25V5.207c0-.464.184-.909.513-1.237l2.96-2.96A1.75 1.75 0 0 1 7.71 .5h3.54A1.75 1.75 0 0 1 13 2.25v3a.75.75 0 0 1-1.5 0v-3a.25.25 0 0 0-.25-.25h-2.5Z"
        />
        <path
          d="M8.753 5.505a.75.75 0 0 1 0-1.06l2.219-2.22a.75.75 0 1 1 1.06 1.061L10.564 4.75h2.186a.75.75 0 0 1 0 1.5h-2.186l1.469 1.468a.75.75 0 0 1-1.06 1.061l-2.22-2.219a.736.736 0 0 1 0-1.055Z"
        />
      </svg>
      <h2 class="diff-empty-title mb-2 text-lg font-medium">Welcome to Aiffer</h2>
      <p class="diff-empty-subtitle mb-4 max-w-md text-center text-sm">
        Open a git repository folder to view diffs and add review comments. Your
        comments will be collected and sent to an AI agent to understand the
        changes you want.
      </p>
      <p class="diff-empty-hint text-xs">
        Click the folder icon in the header to get started
      </p>
    </div>
  {:else if error}
    <!-- Error state -->
    <div
      class="flex flex-1 flex-col items-center justify-center"
      style="color: var(--danger-fg);"
    >
      <svg class="mb-3 h-10 w-10" viewBox="0 0 16 16" fill="currentColor">
        <path
          d="M6.457 1.047c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0 1 14.082 15H1.918a1.75 1.75 0 0 1-1.543-2.575ZM8 5a.75.75 0 0 0-.75.75v2.5a.75.75 0 0 0 1.5 0v-2.5A.75.75 0 0 0 8 5Zm1 6a1 1 0 1 0-2 0 1 1 0 0 0 2 0Z"
        />
      </svg>
      <p class="mb-1 text-sm font-medium">Error reading diffs</p>
      <p class="text-xs" style="opacity: 0.8;">{error}</p>
    </div>
  {:else if loading}
    <!-- Loading state -->
    <div class="flex-1 p-4">
      <div class="space-y-4">
        {#each Array(3) as _}
          <div class="diff-skeleton-card rounded border">
            <div class="diff-skeleton-header flex items-center gap-2 px-4 py-2">
              <div class="diff-skeleton-block h-4 w-4 animate-pulse rounded"></div>
              <div class="diff-skeleton-block h-3 w-48 animate-pulse rounded"></div>
            </div>
            <div class="space-y-0 p-0">
              {#each Array(6) as _}
                <div class="diff-skeleton-row flex">
                  <div
                    class="diff-skeleton-gutter h-5 w-10 animate-pulse"
                  ></div>
                  <div
                    class="diff-skeleton-gutter h-5 w-10 animate-pulse"
                  ></div>
                  <div class="diff-skeleton-line h-5 flex-1 animate-pulse"></div>
                </div>
              {/each}
            </div>
          </div>
        {/each}
      </div>
    </div>
  {:else if diffFiles.length === 0}
    <!-- Empty state: no changes -->
    <div
      class="diff-empty-state flex flex-1 flex-col items-center justify-center"
    >
      <svg class="mb-3 h-10 w-10" viewBox="0 0 16 16" fill="currentColor">
        <path
          d="M13.78 4.22a.75.75 0 0 1 0 1.06l-7.25 7.25a.75.75 0 0 1-1.06 0L2.22 9.28a.751.751 0 0 1 .018-1.042.751.751 0 0 1 1.042-.018L6 10.94l6.72-6.72a.75.75 0 0 1 1.06 0Z"
        />
      </svg>
      <p class="diff-empty-title text-sm font-medium">No changes detected</p>
      <p class="diff-empty-subtitle mt-1 text-xs">
        Your working directory is clean
      </p>
    </div>
  {:else}
    <!-- Diff stats bar -->
    <div
      class="diff-stats-bar sticky top-0 z-[2] flex h-10 items-center gap-4 px-4"
    >
      <span class="text-sm" style="color: var(--app-fg);">
        Showing
        <span class="font-medium">{diffFiles.length}</span>
        changed file{diffFiles.length !== 1 ? "s" : ""}
      </span>
      <span class="flex gap-2 text-sm">
        {#if totalAdditions > 0}
          <span class="font-medium" style="color: var(--success-fg);">+{totalAdditions}</span>
        {/if}
        {#if totalDeletions > 0}
          <span class="font-medium" style="color: var(--danger-fg);">-{totalDeletions}</span>
        {/if}
      </span>

      <div class="flex-1"></div>

      <!-- Expand / Collapse all -->
      <div class="flex items-center gap-1">
        <button
          class="diff-toolbar-btn rounded px-2 py-0.5 text-xs"
          onclick={expandAll}
          title="Expand all files"
        >
          Expand all
        </button>
        <span class="diff-toolbar-sep">|</span>
        <button
          class="diff-toolbar-btn rounded px-2 py-0.5 text-xs"
          onclick={collapseAll}
          title="Collapse all files"
        >
          Collapse all
        </button>
        {#if forceCollapsed !== null}
          <button
            class="diff-toolbar-accent ml-1 rounded px-1.5 py-0.5 text-xs"
            onclick={resetForce}
            title="Reset to individual control"
          >
            Reset
          </button>
        {/if}
      </div>
    </div>

    <!-- File diffs -->
    <div class="flex-1">
      {#each diffFiles as file (file.path)}
        <DiffFileSection
          {file}
          {forceCollapsed}
          {highlightedLineKey}
          {activeCommentKey}
          onClickLine={handleClickLine}
          onClickComment={handleClickComment}
          onSubmitComment={handleSubmitComment}
          onCancelComment={handleCancelComment}
        />
      {/each}
    </div>
  {/if}
</main>

<style>
  .diff-content {
    background-color: var(--panel-bg);
  }
  .diff-empty-state {
    color: var(--panel-faint-fg);
  }
  .diff-empty-title {
    color: var(--app-fg);
  }
  .diff-empty-subtitle {
    color: var(--panel-muted-fg);
  }
  .diff-empty-hint {
    color: var(--panel-faint-fg);
  }
  .diff-stats-bar {
    background-color: var(--panel-muted-bg);
    border-bottom: 1px solid var(--panel-border);
  }
  .diff-toolbar-btn {
    color: var(--panel-muted-fg);
  }
  .diff-toolbar-btn:hover {
    background-color: var(--header-hover-bg);
    color: var(--app-fg);
  }
  .diff-toolbar-sep {
    color: var(--panel-border);
  }
  .diff-toolbar-accent {
    color: var(--accent-fg);
  }
  .diff-toolbar-accent:hover {
    background-color: var(--accent-muted-bg);
  }
  .diff-skeleton-card {
    border-color: var(--panel-border);
  }
  .diff-skeleton-header {
    background-color: var(--panel-muted-bg);
  }
  .diff-skeleton-block {
    background-color: var(--skeleton-bg);
  }
  .diff-skeleton-row {
    border-top: 1px solid var(--panel-border-subtle);
  }
  .diff-skeleton-gutter {
    background-color: var(--panel-muted-bg);
    border-right: 1px solid var(--panel-border-subtle);
  }
  .diff-skeleton-line {
    background-color: var(--panel-muted-bg);
  }
</style>
