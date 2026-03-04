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
<tr class="bg-blue-50 dark:bg-[#161b22]" data-hunk-header>
  <td
    colspan="4"
    class="border-y border-blue-100 dark:border-[#30363d] px-4 py-1 font-mono text-xs text-blue-700 dark:text-[#58a6ff]"
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
