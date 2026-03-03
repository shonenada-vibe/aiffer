<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import Header from "./lib/components/Header.svelte";
  import Sidebar from "./lib/components/Sidebar.svelte";
  import DiffContent from "./lib/components/DiffContent.svelte";
  import AiPanel from "./lib/components/AiPanel.svelte";
  import ReviewSummary from "./lib/components/ReviewSummary.svelte";
  import { diffStore } from "./lib/stores/diff.svelte";
  import { commentStore } from "./lib/stores/comments.svelte";
  import type { DiffLine } from "./lib/types/diff";
  import type { CommentInput, ReviewPayload } from "./lib/types/comment";

  let sidebarWidth = $state(250);
  let aiPanelOpen = $state(false);
  let reviewPanelOpen = $state(false);
  let aiResponse = $state("");
  let aiLoading = $state(false);
  let aiError: string | null = $state(null);

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

      // TODO: Read AI config from settings (task 015). For now use defaults.
      const config = {
        endpoint: "https://api.openai.com/v1",
        apiKey: "",
        model: "gpt-4o",
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
    onSubmitReview={handleSubmitReview}
    onOpenFolder={handleOpenFolder}
    onToggleAiPanel={() => (aiPanelOpen = !aiPanelOpen)}
    onToggleSettings={() => {}}
    onRefresh={handleRefresh}
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
