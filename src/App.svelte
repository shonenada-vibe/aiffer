<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { invoke } from "@tauri-apps/api/core";
  import Header from "./lib/components/Header.svelte";
  import Sidebar from "./lib/components/Sidebar.svelte";
  import DiffContent from "./lib/components/DiffContent.svelte";
  import AiPanel from "./lib/components/AiPanel.svelte";
  import ReviewSummary from "./lib/components/ReviewSummary.svelte";
  import Settings from "./lib/components/Settings.svelte";
  import KeyboardShortcutHelp from "./lib/components/KeyboardShortcutHelp.svelte";
  import ErrorBoundary from "./lib/components/ErrorBoundary.svelte";
  import ToastContainer from "./lib/components/ToastContainer.svelte";
  import { diffStore } from "./lib/stores/diff.svelte";
  import { commentStore } from "./lib/stores/comments.svelte";
  import { settingsStore } from "./lib/stores/settings.svelte";
  import { toastStore } from "./lib/stores/toast.svelte";
  import type { DiffLine, DiffType } from "./lib/types/diff";
  import type { CommentInput, ReviewPayload } from "./lib/types/comment";

  let sidebarWidth = $state(250);
  let aiPanelOpen = $state(false);
  let reviewPanelOpen = $state(false);
  let settingsOpen = $state(false);
  let shortcutHelpOpen = $state(false);
  let aiResponse = $state("");
  let aiLoading = $state(false);
  let aiError: string | null = $state(null);
  let dragOver = $state(false);

  // Load settings on mount, then auto-open last folder or CLI arg
  $effect(() => {
    initApp();
  });

  async function initApp() {
    await settingsStore.load();

    // Check CLI argument first
    try {
      const cliPath = await invoke<string | null>("get_initial_path");
      if (cliPath) {
        await tryOpenFolder(cliPath);
        return;
      }
    } catch {
      // No CLI arg, continue
    }

    // Otherwise, reopen last folder if available
    const lastFolder = settingsStore.settings.lastOpenedFolder;
    if (lastFolder) {
      await tryOpenFolder(lastFolder);
    }
  }

  async function tryOpenFolder(path: string) {
    try {
      const canonicalPath = await invoke<string>("validate_git_repo", { path });
      await diffStore.openFolder(canonicalPath);
      // Persist as last opened folder
      await settingsStore.save({
        ...settingsStore.settings,
        lastOpenedFolder: canonicalPath,
      });
      const folderName = canonicalPath.split("/").pop() ?? canonicalPath;
      toastStore.success(`Opened repository: ${folderName}`);
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      if (msg.includes("not a git repository") || msg.includes("NotARepo") || msg.includes("Not a repo")) {
        diffStore.setError(
          `Not a git repository: ${path}\n\nPlease open a folder that contains a .git directory, or initialize one with "git init".`,
        );
        toastStore.error("Not a git repository. Try running \"git init\" first.");
      } else if (msg.includes("Permission denied") || msg.includes("permission")) {
        diffStore.setError(
          `Permission denied: ${path}\n\nCheck that you have read access to this directory.`,
        );
        toastStore.error("Permission denied. Check folder access permissions.");
      } else {
        diffStore.setError(`Failed to open folder: ${msg}`);
        toastStore.error(`Failed to open folder: ${msg}`);
      }
    }
  }

  async function handleOpenFolder() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Open Git Repository",
    });
    if (selected && typeof selected === "string") {
      await tryOpenFolder(selected);
    }
  }

  function handleSubmitReview() {
    reviewPanelOpen = !reviewPanelOpen;
  }

  async function handleSubmitToAi() {
    reviewPanelOpen = false;
    aiPanelOpen = true;
    aiLoading = true;
    aiError = null;
    aiResponse = "";

    try {
      // Build the review payload with code context
      const comments: CommentInput[] = commentStore.allComments.map((c) => ({
        filePath: c.filePath,
        lineNumber: c.lineNumber,
        lineType: c.lineType,
        body: c.body,
      }));

      const payload = await invoke<ReviewPayload>("build_review_payload", {
        path: diffStore.folderPath,
        comments,
      });

      const s = settingsStore.settings;
      const config = {
        endpoint: s.aiEndpoint,
        apiKey: s.aiApiKey,
        model: s.aiModel,
      };

      const response = await invoke<string>("submit_review", {
        reviewText: payload.formattedText,
        config,
      });

      aiResponse = response;
      toastStore.success("AI review received");
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      if (msg.includes("endpoint") || msg.includes("Endpoint")) {
        aiError = `API endpoint not configured.\n\nGo to Settings and enter your API endpoint URL.`;
      } else if (msg.includes("API key") || msg.includes("api_key") || msg.includes("Unauthorized") || msg.includes("401")) {
        aiError = `Authentication failed.\n\nCheck your API key in Settings. Make sure it is valid and has not expired.`;
      } else if (msg.includes("timeout") || msg.includes("Timeout")) {
        aiError = `Request timed out.\n\nThe AI service may be slow or unreachable. Try again in a moment.`;
      } else if (msg.includes("connection") || msg.includes("Connection") || msg.includes("network")) {
        aiError = `Network error.\n\nCheck your internet connection and verify the API endpoint is reachable.`;
      } else if (msg.includes("model") || msg.includes("Model")) {
        aiError = `Invalid model.\n\nCheck your model name in Settings. The configured model may not be available.`;
      } else {
        aiError = msg;
      }
      toastStore.error("AI review failed. See details in the panel.");
    } finally {
      aiLoading = false;
    }
  }

  async function handleCopyPrompt() {
    try {
      const comments: CommentInput[] = commentStore.allComments.map((c) => ({
        filePath: c.filePath,
        lineNumber: c.lineNumber,
        lineType: c.lineType,
        body: c.body,
      }));

      const payload = await invoke<ReviewPayload>("build_review_payload", {
        path: diffStore.folderPath,
        comments,
      });

      await writeText(payload.formattedText);
      toastStore.success("Prompt copied to clipboard");
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      toastStore.error(`Failed to copy prompt: ${msg}`);
    }
  }

  async function handleCopySimplePrompt() {
    try {
      const allComments = commentStore.allComments;
      if (allComments.length === 0) {
        toastStore.info("No comments to copy");
        return;
      }

      const CONTEXT_LINES = 3;
      const blocks: string[] = [];
      for (const comment of allComments) {
        const file = diffStore.diffFiles.find((f) => f.path === comment.filePath);
        let diffSnippet = "";
        if (file) {
          // Collect all lines from the hunk containing the commented line
          for (const hunk of file.hunks) {
            const allLines = hunk.lines;
            const targetIdx = allLines.findIndex((l) => {
              const ln = l.newLineno ?? l.oldLineno ?? 0;
              return ln === comment.lineNumber;
            });
            if (targetIdx === -1) continue;

            const startIdx = Math.max(0, targetIdx - CONTEXT_LINES);
            const endIdx = Math.min(allLines.length - 1, targetIdx + CONTEXT_LINES);
            const snippetLines: string[] = [];
            for (let i = startIdx; i <= endIdx; i++) {
              const l = allLines[i];
              const prefix = l.lineType === "addition" ? "+" : l.lineType === "deletion" ? "-" : " ";
              snippetLines.push(`${prefix}${l.content}`);
            }
            diffSnippet = snippetLines.join("\n");
            break;
          }
        }

        let block = `@${comment.filePath}:${comment.lineNumber}`;
        if (diffSnippet) {
          block += `\n${diffSnippet}`;
        }
        block += `\n\nComment: ${comment.body}`;
        blocks.push(block);
      }

      await writeText(blocks.join("\n\n"));
      toastStore.success("Simple prompt copied to clipboard");
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      toastStore.error(`Failed to copy prompt: ${msg}`);
    }
  }

  async function handleDiffTypeChange(type_: DiffType) {
    if (
      commentStore.commentCount > 0 &&
      !confirm(
        "Switching diff types may invalidate existing comments. Continue?",
      )
    ) {
      return;
    }
    diffStore.setDiffType(type_);
    if (type_ === "commits") {
      await diffStore.loadCommits();
    } else {
      await diffStore.refresh();
    }
  }

  async function handleFromRefChange(ref_: string) {
    diffStore.setFromRef(ref_);
    if (diffStore.toRef) {
      await diffStore.refresh();
    }
  }

  async function handleToRefChange(ref_: string) {
    diffStore.setToRef(ref_);
    if (diffStore.fromRef) {
      await diffStore.refresh();
    }
  }

  async function handleRefresh() {
    await diffStore.refresh();
    if (!diffStore.error) {
      toastStore.info("Diffs refreshed");
    }
  }

  function handleSelectFile(path: string) {
    diffStore.setSelectedFile(path);
    const el = document.getElementById(`diff-file-${CSS.escape(path)}`);
    if (el) {
      el.scrollIntoView({ behavior: "smooth", block: "start" });
    }
  }

  function handleSubmitComment(filePath: string, line: DiffLine, body: string) {
    const lineNumber = line.newLineno ?? line.oldLineno ?? 0;
    commentStore.addComment(filePath, lineNumber, line.lineType, body);
    toastStore.success("Comment added");
  }

  // Drag-and-drop handlers
  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    if (e.dataTransfer) {
      e.dataTransfer.dropEffect = "copy";
    }
    dragOver = true;
  }

  function handleDragLeave(e: DragEvent) {
    // Only dismiss if leaving the window entirely
    const related = e.relatedTarget as HTMLElement | null;
    if (!related || !document.body.contains(related)) {
      dragOver = false;
    }
  }

  async function handleDrop(e: DragEvent) {
    e.preventDefault();
    dragOver = false;

    if (!e.dataTransfer) return;

    // Try to get the path from dropped files
    const items = e.dataTransfer.files;
    if (items.length > 0) {
      // Tauri exposes the file path via the File object's webkitRelativePath
      // or through the dataTransfer items
      const item = items[0];
      // On desktop Tauri apps, File.name may contain the path
      // Use the Tauri file drop event path if available
      const path = (item as File & { path?: string }).path;
      if (path) {
        await tryOpenFolder(path);
      }
    }
  }

  function isTextInput(el: EventTarget | null): boolean {
    if (!el || !(el instanceof HTMLElement)) return false;
    const tag = el.tagName;
    return (
      tag === "INPUT" ||
      tag === "TEXTAREA" ||
      tag === "SELECT" ||
      el.isContentEditable
    );
  }

  function scrollToFileByIndex(index: number) {
    const files = diffStore.diffFiles;
    if (files.length === 0 || index < 0 || index >= files.length) return;
    const file = files[index];
    diffStore.setSelectedFile(file.path);
    const el = document.getElementById(`diff-file-${CSS.escape(file.path)}`);
    if (el) el.scrollIntoView({ behavior: "smooth", block: "start" });
  }

  function getCurrentFileIndex(): number {
    const selected = diffStore.selectedFile;
    if (!selected) return -1;
    return diffStore.diffFiles.findIndex((f) => f.path === selected);
  }

  function scrollToHunk(direction: "next" | "prev") {
    const hunkHeaders = document.querySelectorAll<HTMLElement>(
      "[data-hunk-header]",
    );
    if (hunkHeaders.length === 0) return;

    const scrollContainer = document.querySelector("main");
    if (!scrollContainer) return;
    const scrollTop = scrollContainer.scrollTop;

    if (direction === "next") {
      for (const el of hunkHeaders) {
        if (el.offsetTop > scrollTop + 60) {
          el.scrollIntoView({ behavior: "smooth", block: "start" });
          return;
        }
      }
    } else {
      for (let i = hunkHeaders.length - 1; i >= 0; i--) {
        if (hunkHeaders[i].offsetTop < scrollTop - 10) {
          hunkHeaders[i].scrollIntoView({ behavior: "smooth", block: "start" });
          return;
        }
      }
    }
  }

  function openCommentOnHighlightedLine() {
    const highlighted = document.querySelector<HTMLElement>(
      "[data-line-highlighted]",
    );
    if (highlighted) {
      const btn = highlighted.querySelector<HTMLElement>(
        "[data-comment-trigger]",
      );
      if (btn) btn.click();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (isTextInput(e.target)) {
      if (e.key === "Enter" && e.metaKey) {
        e.preventDefault();
        handleSubmitReview();
      }
      return;
    }

    if (e.metaKey && e.shiftKey && e.key.toLowerCase() === "r") {
      e.preventDefault();
      handleRefresh();
      return;
    }

    if (e.metaKey && e.key === "Enter") {
      e.preventDefault();
      handleSubmitReview();
      return;
    }

    if (e.metaKey || e.ctrlKey || e.altKey) return;

    switch (e.key) {
      case "n": {
        e.preventDefault();
        const idx = getCurrentFileIndex();
        scrollToFileByIndex(idx + 1);
        break;
      }
      case "p": {
        e.preventDefault();
        const idx = getCurrentFileIndex();
        scrollToFileByIndex(idx - 1);
        break;
      }
      case "j":
        e.preventDefault();
        scrollToHunk("next");
        break;
      case "k":
        e.preventDefault();
        scrollToHunk("prev");
        break;
      case "c":
        e.preventDefault();
        openCommentOnHighlightedLine();
        break;
      case "?":
        e.preventDefault();
        shortcutHelpOpen = !shortcutHelpOpen;
        break;
      case "Escape":
        shortcutHelpOpen = false;
        break;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<ErrorBoundary>
{#snippet children()}
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="flex h-screen flex-col overflow-hidden"
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
>
  <Header
    folderPath={diffStore.folderPath}
    commentCount={commentStore.commentCount}
    diffType={diffStore.diffType}
    commits={diffStore.commits}
    fromRef={diffStore.fromRef}
    toRef={diffStore.toRef}
    theme={settingsStore.theme}
    onSubmitReview={handleSubmitReview}
    onOpenFolder={handleOpenFolder}
    onToggleAiPanel={() => (aiPanelOpen = !aiPanelOpen)}
    onToggleSettings={() => (settingsOpen = !settingsOpen)}
    onRefresh={handleRefresh}
    onToggleTheme={() => settingsStore.toggleTheme()}
    onDiffTypeChange={handleDiffTypeChange}
    onFromRefChange={handleFromRefChange}
    onToRefChange={handleToRefChange}
  />

  <div class="flex flex-1 overflow-hidden">
    <Sidebar
      width={sidebarWidth}
      files={diffStore.files}
      selectedFile={diffStore.selectedFile}
      loading={diffStore.loading}
      totalAdditions={diffStore.totalAdditions}
      totalDeletions={diffStore.totalDeletions}
      hasFolder={diffStore.folderPath !== ""}
      onWidthChange={(w) => (sidebarWidth = w)}
      onSelectFile={handleSelectFile}
    />

    <DiffContent
      hasFolder={diffStore.folderPath !== ""}
      diffFiles={diffStore.diffFiles}
      loading={diffStore.loading}
      error={diffStore.error}
      totalAdditions={diffStore.totalAdditions}
      totalDeletions={diffStore.totalDeletions}
      onSubmitComment={handleSubmitComment}
    />

    <ReviewSummary
      isOpen={reviewPanelOpen}
      onClose={() => (reviewPanelOpen = false)}
      onSubmitToAi={handleSubmitToAi}
      onCopyPrompt={handleCopyPrompt}
      onCopySimplePrompt={handleCopySimplePrompt}
    />

    <AiPanel
      isOpen={aiPanelOpen}
      loading={aiLoading}
      response={aiResponse}
      error={aiError}
      onClose={() => (aiPanelOpen = false)}
      onRetry={handleSubmitToAi}
    />
  </div>

  <!-- Drag-and-drop overlay -->
  {#if dragOver}
    <div class="drop-overlay fixed inset-0 z-[100] flex items-center justify-center">
      <div class="drop-overlay-content rounded-2xl border-2 border-dashed px-12 py-8 text-center">
        <svg class="mx-auto mb-3 h-12 w-12" viewBox="0 0 16 16" fill="currentColor" style="color: var(--accent-fg);">
          <path d="M1.75 1A1.75 1.75 0 0 0 0 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0 0 16 13.25v-8.5A1.75 1.75 0 0 0 14.25 3H7.5a.25.25 0 0 1-.2-.1l-.9-1.2C6.07 1.26 5.55 1 5 1H1.75Z" />
        </svg>
        <p class="text-lg font-medium" style="color: var(--app-fg);">Drop a folder to open</p>
        <p class="mt-1 text-sm" style="color: var(--panel-muted-fg);">The folder must be a git repository</p>
      </div>
    </div>
  {/if}
</div>

<Settings isOpen={settingsOpen} onClose={() => (settingsOpen = false)} />
<KeyboardShortcutHelp
  isOpen={shortcutHelpOpen}
  onClose={() => (shortcutHelpOpen = false)}
/>
<ToastContainer />
{/snippet}
</ErrorBoundary>

<style>
  .drop-overlay {
    background-color: var(--overlay-bg);
  }
  .drop-overlay-content {
    background-color: var(--panel-bg);
    border-color: var(--accent-fg);
  }
</style>
