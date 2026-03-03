<script lang="ts">
  import type { DiffHunk, DiffLine } from "../types/diff";
  import DiffLineRow from "./DiffLine.svelte";
  import CommentForm from "./CommentForm.svelte";

  interface Props {
    hunk: DiffHunk;
    filePath: string;
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
</script>

<!-- Hunk header -->
<tr class="bg-blue-50">
  <td
    colspan="4"
    class="border-y border-blue-100 px-4 py-1 font-mono text-xs text-blue-700"
  >
    {hunk.header}
  </td>
</tr>
<!-- Hunk lines -->
{#each hunk.lines as line, i (i)}
  {@const key = lineKey(line)}
  <DiffLineRow
    {line}
    {filePath}
    highlighted={highlightedLineKey === key}
    {onClickLine}
    {onClickComment}
  />
  {#if activeCommentKey === key}
    <CommentForm
      onSubmit={(body) => onSubmitComment(filePath, line, body)}
      onCancel={onCancelComment}
    />
  {/if}
{/each}
