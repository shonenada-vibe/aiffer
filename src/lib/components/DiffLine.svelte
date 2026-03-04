<script lang="ts">
  import type { DiffLine } from "../types/diff";
  import { highlightLine, detectLanguage } from "../utils/highlight";

  interface Props {
    line: DiffLine;
    filePath: string;
    highlighted: boolean;
    commentCount: number;
    language: string | null;
    onClickLine: (filePath: string, line: DiffLine) => void;
    onClickComment: (filePath: string, line: DiffLine) => void;
  }

  let {
    line,
    filePath,
    highlighted,
    commentCount,
    language,
    onClickLine,
    onClickComment,
  }: Props = $props();

  let highlightedContent = $derived(highlightLine(line.content, language));

  let lineNo = $derived(line.newLineno ?? line.oldLineno ?? 0);

  let prefix = $derived(
    line.lineType === "addition"
      ? "+"
      : line.lineType === "deletion"
        ? "-"
        : " ",
  );
</script>

<tr
  class="group leading-5 diff-line"
  class:diff-line-add={!highlighted && line.lineType === "addition"}
  class:diff-line-del={!highlighted && line.lineType === "deletion"}
  class:diff-line-highlight={highlighted}
  class:diff-line-ctx={!highlighted && line.lineType !== "addition" && line.lineType !== "deletion"}
  data-line-highlighted={highlighted ? "" : undefined}
>
  <!-- Comment trigger button (appears on hover) / Comment count badge -->
  <td
    class="diff-gutter w-[1px] select-none whitespace-nowrap relative"
  >
    {#if commentCount > 0}
      <button
        class="absolute inset-0 z-[1] flex items-center justify-center"
        onclick={(e) => {
          e.stopPropagation();
          onClickComment(filePath, line);
        }}
        title="{commentCount} comment{commentCount !== 1 ? 's' : ''} on line {lineNo}"
      >
        <span
          class="diff-comment-badge flex h-4 min-w-4 items-center justify-center rounded-full px-1 text-[10px] font-bold leading-none text-white"
        >
          {commentCount}
        </span>
      </button>
    {:else}
      <button
        class="absolute inset-0 flex items-center justify-center opacity-0 transition-opacity group-hover:opacity-100"
        data-comment-trigger
        onclick={(e) => {
          e.stopPropagation();
          onClickComment(filePath, line);
        }}
        title="Add comment on line {lineNo}"
      >
        <svg
          class="diff-comment-trigger h-3.5 w-3.5 rounded p-0.5 text-white"
          viewBox="0 0 16 16"
          fill="currentColor"
        >
          <path
            d="M7.75 2a.75.75 0 0 1 .75.75V7h4.25a.75.75 0 0 1 0 1.5H8.5v4.25a.75.75 0 0 1-1.5 0V8.5H2.75a.75.75 0 0 1 0-1.5H7V2.75A.75.75 0 0 1 7.75 2Z"
          />
        </svg>
      </button>
    {/if}
    <!-- Old line number -->
    <span
      class="diff-lineno block cursor-pointer px-2 text-right font-mono text-xs"
      role="button"
      tabindex="-1"
      onclick={() => onClickLine(filePath, line)}
      onkeydown={(e) => { if (e.key === 'Enter') onClickLine(filePath, line); }}
    >
      {line.oldLineno ?? ""}
    </span>
  </td>
  <!-- New line number -->
  <td
    class="diff-gutter w-[1px] select-none whitespace-nowrap"
  >
    <span
      class="diff-lineno block cursor-pointer px-2 text-right font-mono text-xs"
      role="button"
      tabindex="-1"
      onclick={() => onClickLine(filePath, line)}
      onkeydown={(e) => { if (e.key === 'Enter') onClickLine(filePath, line); }}
    >
      {line.newLineno ?? ""}
    </span>
  </td>
  <!-- Prefix (+/-/space) -->
  <td
    class="diff-text w-[1px] select-none whitespace-nowrap pl-2 pr-0 text-center align-top font-mono text-xs"
  >
    {prefix}
  </td>
  <!-- Content -->
  <td
    class="hljs diff-text whitespace-pre-wrap break-all pr-4 align-top font-mono text-xs"
  >
    {#if language}
      {@html highlightedContent || "&nbsp;"}
    {:else}
      {line.content || "\n"}
    {/if}
  </td>
</tr>

<style>
  .diff-line {
    --line-bg: transparent;
    --line-hover-bg: var(--panel-muted-bg);
    --line-gutter-bg: var(--gutter-bg);
    --line-text-color: var(--app-fg);
  }
  .diff-line-add {
    --line-bg: var(--diff-add-bg);
    --line-hover-bg: var(--diff-add-hover-bg);
    --line-gutter-bg: var(--diff-add-gutter-bg);
    --line-text-color: var(--diff-add-fg);
  }
  .diff-line-del {
    --line-bg: var(--diff-del-bg);
    --line-hover-bg: var(--diff-del-hover-bg);
    --line-gutter-bg: var(--diff-del-gutter-bg);
    --line-text-color: var(--diff-del-fg);
  }
  .diff-line-highlight {
    --line-bg: var(--diff-highlight-bg);
    --line-hover-bg: var(--diff-highlight-bg);
    --line-gutter-bg: var(--diff-highlight-gutter-bg);
    --line-text-color: var(--app-fg);
  }
  .diff-line-ctx {
    --line-bg: transparent;
    --line-hover-bg: var(--panel-muted-bg);
    --line-gutter-bg: var(--gutter-bg);
    --line-text-color: var(--app-fg);
  }
  .diff-line {
    background-color: var(--line-bg);
  }
  .diff-line:hover {
    background-color: var(--line-hover-bg);
  }
  .diff-line-highlight:hover {
    background-color: var(--diff-highlight-bg);
  }
  .diff-gutter {
    background-color: var(--line-gutter-bg);
    border-right: 1px solid var(--gutter-border);
  }
  .diff-lineno {
    color: var(--gutter-fg);
  }
  .diff-line:hover .diff-lineno {
    color: var(--gutter-hover-fg);
  }
  .diff-text {
    color: var(--line-text-color);
  }
  .diff-comment-badge {
    background-color: var(--accent-fg);
  }
  .diff-comment-trigger {
    background-color: var(--accent-fg);
  }
</style>
