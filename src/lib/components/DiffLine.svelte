<script lang="ts">
  import type { DiffLine } from "../types/diff";

  interface Props {
    line: DiffLine;
    filePath: string;
    highlighted: boolean;
    commentCount: number;
    onClickLine: (filePath: string, line: DiffLine) => void;
    onClickComment: (filePath: string, line: DiffLine) => void;
  }

  let {
    line,
    filePath,
    highlighted,
    commentCount,
    onClickLine,
    onClickComment,
  }: Props = $props();

  let bgClass = $derived(
    highlighted
      ? "bg-yellow-100"
      : line.lineType === "addition"
        ? "bg-green-50"
        : line.lineType === "deletion"
          ? "bg-red-50"
          : "",
  );

  let hoverBgClass = $derived(
    highlighted
      ? ""
      : line.lineType === "addition"
        ? "hover:bg-green-100/60"
        : line.lineType === "deletion"
          ? "hover:bg-red-100/60"
          : "hover:bg-gray-50",
  );

  let gutterBgClass = $derived(
    highlighted
      ? "bg-yellow-200"
      : line.lineType === "addition"
        ? "bg-green-100"
        : line.lineType === "deletion"
          ? "bg-red-100"
          : "bg-gray-50",
  );

  let textClass = $derived(
    line.lineType === "addition"
      ? "text-green-800"
      : line.lineType === "deletion"
        ? "text-red-800"
        : "text-gray-800",
  );

  let prefix = $derived(
    line.lineType === "addition"
      ? "+"
      : line.lineType === "deletion"
        ? "-"
        : " ",
  );

  let lineNo = $derived(line.newLineno ?? line.oldLineno ?? 0);
</script>

<tr
  class="group leading-5 {bgClass} {hoverBgClass}"
  data-line-highlighted={highlighted ? "" : undefined}
>
  <!-- Comment trigger button (appears on hover) / Comment count badge -->
  <td
    class="w-[1px] select-none whitespace-nowrap border-r border-gray-200 {gutterBgClass} relative"
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
          class="flex h-4 min-w-4 items-center justify-center rounded-full bg-blue-500 px-1 text-[10px] font-bold leading-none text-white"
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
          class="h-3.5 w-3.5 rounded bg-blue-500 p-0.5 text-white"
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
      class="block cursor-pointer px-2 text-right font-mono text-xs text-gray-400 group-hover:text-gray-600"
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
    class="w-[1px] select-none whitespace-nowrap border-r border-gray-200 {gutterBgClass}"
  >
    <span
      class="block cursor-pointer px-2 text-right font-mono text-xs text-gray-400 group-hover:text-gray-600"
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
    class="w-[1px] select-none whitespace-nowrap pl-2 pr-0 text-center align-top font-mono text-xs {textClass}"
  >
    {prefix}
  </td>
  <!-- Content -->
  <td
    class="whitespace-pre-wrap break-all pr-4 align-top font-mono text-xs {textClass}"
  >
    {line.content || "\n"}
  </td>
</tr>
