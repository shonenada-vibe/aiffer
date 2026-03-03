<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import Header from "./lib/components/Header.svelte";
  import Sidebar from "./lib/components/Sidebar.svelte";
  import DiffContent from "./lib/components/DiffContent.svelte";
  import AiPanel from "./lib/components/AiPanel.svelte";
  import ReviewSummary from "./lib/components/ReviewSummary.svelte";
  import Settings from "./lib/components/Settings.svelte";
  import { diffStore } from "./lib/stores/diff.svelte";
  import { commentStore } from "./lib/stores/comments.svelte";
  import { settingsStore } from "./lib/stores/settings.svelte";
  import type { DiffLine, DiffType } from "./lib/types/diff";
  import type { CommentInput, ReviewPayload } from "./lib/types/comment";

  let sidebarWidth = $state(250);
  let aiPanelOpen = $state(false);
  let reviewPanelOpen = $state(false);
  let settingsOpen = $state(false);
  let aiResponse = $state("");
  let aiLoading = $state(false);
  let aiError: string | null = $state(null);

  // Load settings on mount
  $effect(() => {
    settingsStore.load();
  });

  async function handleOpenFolder() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Open Git Repository",
    });
    if (selected && typeof selected === "string") {
      await diffStore.openFolder(selected);
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
    } catch (err) {
      aiError = err instanceof Error ? err.message : String(err);
    } finally {
      aiLoading = false;
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
  }
</script>

<div class="flex h-screen flex-col overflow-hidden">
  <Header
    folderPath={diffStore.folderPath}
    commentCount={commentStore.commentCount}
    diffType={diffStore.diffType}
    commits={diffStore.commits}
    fromRef={diffStore.fromRef}
    toRef={diffStore.toRef}
    onSubmitReview={handleSubmitReview}
    onOpenFolder={handleOpenFolder}
    onToggleAiPanel={() => (aiPanelOpen = !aiPanelOpen)}
    onToggleSettings={() => (settingsOpen = !settingsOpen)}
    onRefresh={handleRefresh}
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
</div>

<Settings isOpen={settingsOpen} onClose={() => (settingsOpen = false)} />
