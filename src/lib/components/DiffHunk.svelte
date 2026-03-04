<script lang="ts">
  import type { DiffHunk, DiffLine } from "../types/diff";
  import { commentStore } from "../stores/comments.svelte";
  import DiffLineRow from "./DiffLine.svelte";
  import CommentForm from "./CommentForm.svelte";
  import CommentThread from "./CommentThread.svelte";

  interface Props {
    hunk: DiffHunk;
    filePath: string;
    language: string | null;
    highlightedLineKey: string | null;
    activeCommentKey: string | null;
    onClickLine: (filePath: string, line: DiffLine) => void;
    onClickComment: (filePath: string, line: DiffLine) => void;
    onSubmitComment: (filePath: string, line: DiffLine, body: string) => void;
    onCancelComment: () => void;
  }

  let {
    hunk,
    filePath,
    language,
    highlightedLineKey,
    activeCommentKey,
    onClickLine,
    onClickComment,
    onSubmitComment,
    onCancelComment,
  }: Props = $props();

  function lineKey(line: DiffLine): string {
    return `${filePath}:${line.oldLineno ?? ""}:${line.newLineno ?? ""}`;
  }

  function lineNumber(line: DiffLine): number {
    return line.newLineno ?? line.oldLineno ?? 0;
  }
</script>

<!-- Hunk header -->
<tr class="diff-hunk-header" data-hunk-header>
  <td
    colspan="4"
    class="px-4 py-1 font-mono text-xs"
  >
    {hunk.header}
  </td>
</tr>
<!-- Hunk lines -->
{#each hunk.lines as line, i (i)}
  {@const key = lineKey(line)}
  {@const lineNo = lineNumber(line)}
  {@const lineComments = commentStore.getCommentsForLine(filePath, lineNo)}
  <DiffLineRow
    {line}
    {filePath}
    {language}
    highlighted={highlightedLineKey === key}
    commentCount={lineComments.length}
    {onClickLine}
    {onClickComment}
  />
  {#if lineComments.length > 0}
    <CommentThread comments={lineComments} />
  {/if}
  {#if activeCommentKey === key}
    <CommentForm
      onSubmit={(body) => onSubmitComment(filePath, line, body)}
      onCancel={onCancelComment}
    />
  {/if}
{/each}

<style>
  .diff-hunk-header {
    background-color: var(--diff-hunk-bg);
  }
  .diff-hunk-header td {
    border-top: 1px solid var(--diff-hunk-border);
    border-bottom: 1px solid var(--diff-hunk-border);
    color: var(--diff-hunk-fg);
  }
</style>
