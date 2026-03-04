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

  function statusColor(status: FileStatus): string {
    const map: Record<FileStatus, string> = {
      added: "text-green-600 bg-green-50 dark:text-[#3fb950] dark:bg-[#3fb950]/10",
      modified: "text-yellow-600 bg-yellow-50 dark:text-[#d29922] dark:bg-[#d29922]/10",
      deleted: "text-red-600 bg-red-50 dark:text-[#f85149] dark:bg-[#f85149]/10",
      renamed: "text-blue-600 bg-blue-50 dark:text-[#58a6ff] dark:bg-[#58a6ff]/10",
      copied: "text-blue-600 bg-blue-50 dark:text-[#58a6ff] dark:bg-[#58a6ff]/10",
      untracked: "text-gray-500 bg-gray-100 dark:text-[#8b949e] dark:bg-[#8b949e]/10",
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
  class="relative flex flex-col border-r border-gray-200 bg-gray-50 dark:border-[#30363d] dark:bg-[#161b22]"
  style="width: {width}px; min-width: {width}px;"
>
  <!-- Sidebar header -->
  <div
    class="flex h-10 items-center justify-between border-b border-gray-200 px-3 dark:border-[#30363d]"
  >
    <span class="text-xs font-semibold uppercase tracking-wide text-gray-500 dark:text-[#8b949e]">
      Changed Files
    </span>
    {#if files.length > 0}
      <span class="text-xs text-gray-400 dark:text-[#8b949e]">{files.length}</span>
    {/if}
  </div>

  <!-- Overall stats -->
  {#if hasFolder && files.length > 0}
    <div
      class="flex items-center gap-3 border-b border-gray-200 px-3 py-2 text-xs dark:border-[#30363d]"
    >
      <span class="text-gray-500 dark:text-[#8b949e]">
        {files.length} file{files.length !== 1 ? "s" : ""} changed
      </span>
      {#if totalAdditions > 0}
        <span class="font-medium text-green-600 dark:text-[#3fb950]">+{totalAdditions}</span>
      {/if}
      {#if totalDeletions > 0}
        <span class="font-medium text-red-600 dark:text-[#f85149]">-{totalDeletions}</span>
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
            <div class="h-4 w-4 animate-pulse rounded bg-gray-200 dark:bg-[#30363d]"></div>
            <div class="h-3 flex-1 animate-pulse rounded bg-gray-200 dark:bg-[#30363d]"></div>
          </div>
        {/each}
      </div>
    {:else if !hasFolder}
      <!-- Empty state: no folder -->
      <div
        class="flex flex-col items-center justify-center py-8 text-gray-400 dark:text-[#8b949e]"
      >
        <svg class="mb-2 h-8 w-8" viewBox="0 0 16 16" fill="currentColor">
          <path
            d="M1.75 1A1.75 1.75 0 0 0 0 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0 0 16 13.25v-8.5A1.75 1.75 0 0 0 14.25 3H7.5a.25.25 0 0 1-.2-.1l-.9-1.2C6.07 1.26 5.55 1 5 1H1.75Z"
          />
        </svg>
        <span class="text-sm">Open a folder to view changes</span>
      </div>
    {:else if files.length === 0}
      <!-- Empty state: no changes -->
      <div
        class="flex flex-col items-center justify-center py-8 text-gray-400 dark:text-[#8b949e]"
      >
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
            class="flex w-full items-center gap-2 rounded px-2 py-1.5 text-left text-sm transition-colors
              {selectedFile === file.path
              ? 'bg-blue-100 text-blue-900 dark:bg-[#1f6feb]/20 dark:text-[#58a6ff]'
              : 'text-gray-700 hover:bg-gray-100 dark:text-[#e6edf3] dark:hover:bg-[#30363d]'}"
            onclick={() => onSelectFile(file.path)}
            title={file.path}
          >
            <!-- Status badge -->
            <span
              class="inline-flex h-4 w-4 shrink-0 items-center justify-center rounded text-[10px] font-bold leading-none {statusColor(file.status)}"
            >
              {statusLabel(file.status)}
            </span>

            <!-- File name + directory -->
            <span class="min-w-0 flex-1 truncate">
              {#if fileDir(file.path)}
                <span class="text-gray-400 dark:text-[#8b949e]">{fileDir(file.path)}</span>
              {/if}
              <span class="font-medium">{fileName(file.path)}</span>
            </span>

            <!-- Diff stats -->
            <span class="flex shrink-0 gap-1 text-xs">
              {#if file.additions > 0}
                <span class="text-green-600 dark:text-[#3fb950]">+{file.additions}</span>
              {/if}
              {#if file.deletions > 0}
                <span class="text-red-600 dark:text-[#f85149]">-{file.deletions}</span>
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
    class="absolute top-0 right-0 z-10 h-full w-1 cursor-col-resize hover:bg-blue-400"
    class:bg-blue-400={isResizing}
    onmousedown={onMouseDown}
  ></div>
</aside>
