<script lang="ts">
  import type { FileEntry, FileStatus } from "../types/diff";

  interface Props {
    width: number;
    files: FileEntry[];
    selectedFile: string | null;
    loading: boolean;
    totalAdditions: number;
    totalDeletions: number;
    hasFolder: boolean;
    onWidthChange: (width: number) => void;
    onSelectFile: (path: string) => void;
  }

  let {
    width,
    files,
    selectedFile,
    loading,
    totalAdditions,
    totalDeletions,
    hasFolder,
    onWidthChange,
    onSelectFile,
  }: Props = $props();

  let isResizing = $state(false);

  function onMouseDown(e: MouseEvent) {
    e.preventDefault();
    isResizing = true;

    const onMouseMove = (e: MouseEvent) => {
      const newWidth = Math.max(180, Math.min(500, e.clientX));
      onWidthChange(newWidth);
    };

    const onMouseUp = () => {
      isResizing = false;
      document.removeEventListener("mousemove", onMouseMove);
      document.removeEventListener("mouseup", onMouseUp);
      document.body.style.cursor = "";
      document.body.style.userSelect = "";
    };

    document.addEventListener("mousemove", onMouseMove);
    document.addEventListener("mouseup", onMouseUp);
    document.body.style.cursor = "col-resize";
    document.body.style.userSelect = "none";
  }

  function statusLabel(status: FileStatus): string {
    const map: Record<FileStatus, string> = {
      added: "A",
      modified: "M",
      deleted: "D",
      renamed: "R",
      copied: "C",
      untracked: "U",
    };
    return map[status];
  }

  function statusVar(status: FileStatus): string {
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

  function fileName(path: string): string {
    return path.split("/").pop() ?? path;
  }

  function fileDir(path: string): string {
    const parts = path.split("/");
    if (parts.length <= 1) return "";
    return parts.slice(0, -1).join("/") + "/";
  }
</script>

<aside
  class="sidebar relative flex flex-col"
  style="width: {width}px; min-width: {width}px;"
>
  <!-- Sidebar header -->
  <div class="sidebar-header flex h-10 items-center justify-between px-3">
    <span class="sidebar-title text-xs font-semibold uppercase tracking-wide">
      Changed Files
    </span>
    {#if files.length > 0}
      <span class="sidebar-count text-xs">{files.length}</span>
    {/if}
  </div>

  <!-- Overall stats -->
  {#if hasFolder && files.length > 0}
    <div class="sidebar-stats flex items-center gap-3 px-3 py-2 text-xs">
      <span class="sidebar-stats-label">
        {files.length} file{files.length !== 1 ? "s" : ""} changed
      </span>
      {#if totalAdditions > 0}
        <span class="font-medium" style="color: var(--success-fg);">+{totalAdditions}</span>
      {/if}
      {#if totalDeletions > 0}
        <span class="font-medium" style="color: var(--danger-fg);">-{totalDeletions}</span>
      {/if}
    </div>
  {/if}

  <!-- File list -->
  <div class="flex-1 overflow-y-auto">
    {#if loading}
      <!-- Loading skeleton -->
      <div class="space-y-1 p-2">
        {#each Array(5) as _}
          <div class="flex items-center gap-2 rounded px-2 py-1.5">
            <div class="sidebar-skeleton h-4 w-4 animate-pulse rounded"></div>
            <div class="sidebar-skeleton h-3 flex-1 animate-pulse rounded"></div>
          </div>
        {/each}
      </div>
    {:else if !hasFolder}
      <!-- Empty state: no folder -->
      <div class="sidebar-empty flex flex-col items-center justify-center py-8">
        <svg class="mb-2 h-8 w-8" viewBox="0 0 16 16" fill="currentColor">
          <path
            d="M1.75 1A1.75 1.75 0 0 0 0 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0 0 16 13.25v-8.5A1.75 1.75 0 0 0 14.25 3H7.5a.25.25 0 0 1-.2-.1l-.9-1.2C6.07 1.26 5.55 1 5 1H1.75Z"
          />
        </svg>
        <span class="text-sm">Open a folder to view changes</span>
      </div>
    {:else if files.length === 0}
      <!-- Empty state: no changes -->
      <div class="sidebar-empty flex flex-col items-center justify-center py-8">
        <svg class="mb-2 h-8 w-8" viewBox="0 0 16 16" fill="currentColor">
          <path
            d="M13.78 4.22a.75.75 0 0 1 0 1.06l-7.25 7.25a.75.75 0 0 1-1.06 0L2.22 9.28a.751.751 0 0 1 .018-1.042.751.751 0 0 1 1.042-.018L6 10.94l6.72-6.72a.75.75 0 0 1 1.06 0Z"
          />
        </svg>
        <span class="text-sm">No changes detected</span>
      </div>
    {:else}
      <!-- File entries -->
      <div class="space-y-0.5 p-1">
        {#each files as file (file.path)}
          <button
            class="sidebar-file flex w-full items-center gap-2 rounded px-2 py-1.5 text-left text-sm transition-colors"
            class:sidebar-file-selected={selectedFile === file.path}
            onclick={() => onSelectFile(file.path)}
            title={file.path}
          >
            <!-- Status badge -->
            <span
              class="inline-flex h-4 w-4 shrink-0 items-center justify-center rounded text-[10px] font-bold leading-none"
              style="color: var(--{statusVar(file.status)}-fg); background-color: var(--{statusVar(file.status)}-muted-bg);"
            >
              {statusLabel(file.status)}
            </span>

            <!-- File name + directory -->
            <span class="min-w-0 flex-1 truncate">
              {#if fileDir(file.path)}
                <span class="sidebar-file-dir">{fileDir(file.path)}</span>
              {/if}
              <span class="font-medium">{fileName(file.path)}</span>
            </span>

            <!-- Diff stats -->
            <span class="flex shrink-0 gap-1 text-xs">
              {#if file.additions > 0}
                <span style="color: var(--success-fg);">+{file.additions}</span>
              {/if}
              {#if file.deletions > 0}
                <span style="color: var(--danger-fg);">-{file.deletions}</span>
              {/if}
            </span>
          </button>
        {/each}
      </div>
    {/if}
  </div>

  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <!-- Resize handle -->
  <div
    role="separator"
    aria-orientation="vertical"
    aria-valuenow={width}
    aria-valuemin={180}
    aria-valuemax={500}
    tabindex="-1"
    class="sidebar-resize absolute top-0 right-0 z-10 h-full w-1 cursor-col-resize"
    class:sidebar-resize-active={isResizing}
    onmousedown={onMouseDown}
  ></div>
</aside>

<style>
  .sidebar {
    background-color: var(--panel-muted-bg);
    border-right: 1px solid var(--panel-border);
  }
  .sidebar-header {
    border-bottom: 1px solid var(--panel-border);
  }
  .sidebar-title {
    color: var(--panel-muted-fg);
  }
  .sidebar-count {
    color: var(--panel-faint-fg);
  }
  .sidebar-stats {
    border-bottom: 1px solid var(--panel-border);
  }
  .sidebar-stats-label {
    color: var(--panel-muted-fg);
  }
  .sidebar-skeleton {
    background-color: var(--skeleton-bg);
  }
  .sidebar-empty {
    color: var(--panel-faint-fg);
  }
  .sidebar-file {
    color: var(--app-fg);
  }
  .sidebar-file:hover {
    background-color: var(--header-hover-bg);
  }
  .sidebar-file-selected {
    background-color: var(--accent-bg);
    color: var(--accent-fg);
  }
  .sidebar-file-selected:hover {
    background-color: var(--accent-hover-bg);
  }
  .sidebar-file-dir {
    color: var(--panel-faint-fg);
  }
  .sidebar-resize:hover {
    background-color: var(--accent-fg);
  }
  .sidebar-resize-active {
    background-color: var(--accent-fg);
  }
</style>
