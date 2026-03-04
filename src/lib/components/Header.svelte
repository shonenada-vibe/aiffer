<script lang="ts">
  import type { CommitInfo, DiffType } from "../types/diff";
  import type { Theme } from "../stores/settings.svelte";

  interface Props {
    folderPath: string;
    commentCount: number;
    diffType: DiffType;
    commits: CommitInfo[];
    fromRef: string | null;
    toRef: string | null;
    theme: Theme;
    onSubmitReview: () => void;
    onOpenFolder: () => void;
    onToggleAiPanel: () => void;
    onToggleSettings: () => void;
    onRefresh: () => void;
    onToggleTheme: () => void;
    onDiffTypeChange: (type_: DiffType) => void;
    onFromRefChange: (ref_: string) => void;
    onToRefChange: (ref_: string) => void;
  }

  let {
    folderPath,
    commentCount,
    diffType,
    commits,
    fromRef,
    toRef,
    theme,
    onSubmitReview,
    onOpenFolder,
    onToggleAiPanel,
    onToggleSettings,
    onRefresh,
    onToggleTheme,
    onDiffTypeChange,
    onFromRefChange,
    onToRefChange,
  }: Props = $props();

  let folderName = $derived(
    folderPath ? folderPath.split("/").pop() ?? folderPath : "No folder open",
  );

  const DIFF_TYPE_LABELS: Record<DiffType, string> = {
    unstaged: "Unstaged Changes",
    staged: "Staged Changes",
    commits: "Between Commits",
  };

  function formatCommitLabel(c: CommitInfo): string {
    const summary =
      c.summary.length > 40 ? c.summary.slice(0, 40) + "…" : c.summary;
    return `${c.shortId} — ${summary}`;
  }
</script>

<header
  class="flex h-14 items-center justify-between border-b border-[var(--header-border)] bg-[var(--header-bg)] px-4 text-[var(--header-fg)]"
>
  <!-- Left section: logo + folder info -->
  <div class="flex items-center gap-3">
    <div class="flex items-center gap-2">
      <svg
        class="h-6 w-6"
        style="color: var(--accent-fg);"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
      >
        <path
          d="M12 3L2 12h3v8h6v-6h2v6h6v-8h3L12 3z"
          fill="currentColor"
          stroke="none"
        />
      </svg>
      <span class="text-lg font-semibold">Aiffer</span>
    </div>

    <span class="text-[var(--header-muted-fg)]">|</span>

    <button
      onclick={onOpenFolder}
      class="flex items-center gap-1.5 rounded px-2 py-1 text-sm text-[var(--header-muted-fg)] hover:bg-[var(--header-hover-bg)] hover:text-[var(--header-fg)]"
      title="Open folder"
    >
      <svg class="h-4 w-4" viewBox="0 0 16 16" fill="currentColor">
        <path
          d="M1.75 1A1.75 1.75 0 0 0 0 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0 0 16 13.25v-8.5A1.75 1.75 0 0 0 14.25 3H7.5a.25.25 0 0 1-.2-.1l-.9-1.2C6.07 1.26 5.55 1 5 1H1.75Z"
        />
      </svg>
      {folderName}
    </button>

    {#if folderPath}
      <span class="max-w-xs truncate text-xs text-[var(--header-muted-fg)]" title={folderPath}>
        {folderPath}
      </span>
    {/if}
  </div>

  <!-- Center section: diff type selector -->
  <div class="flex items-center gap-2">
    <select
      class="rounded border border-[var(--header-border)] bg-[var(--panel-bg)] px-2 py-1 text-sm text-[var(--header-fg)] focus:outline-none"
      value={diffType}
      onchange={(e) =>
        onDiffTypeChange(
          (e.target as HTMLSelectElement).value as DiffType,
        )}
    >
      {#each Object.entries(DIFF_TYPE_LABELS) as [value, label]}
        <option {value}>{label}</option>
      {/each}
    </select>

    {#if diffType === "commits"}
      <select
        class="max-w-48 rounded border border-[var(--header-border)] bg-[var(--panel-bg)] px-2 py-1 text-sm text-[var(--header-fg)] focus:outline-none"
        value={fromRef ?? ""}
        onchange={(e) =>
          onFromRefChange((e.target as HTMLSelectElement).value)}
      >
        <option value="" disabled>From…</option>
        {#each commits as c}
          <option value={c.oid}>{formatCommitLabel(c)}</option>
        {/each}
      </select>

      <span class="text-xs text-[var(--header-muted-fg)]">→</span>

      <select
        class="max-w-48 rounded border border-[var(--header-border)] bg-[var(--panel-bg)] px-2 py-1 text-sm text-[var(--header-fg)] focus:outline-none"
        value={toRef ?? ""}
        onchange={(e) =>
          onToRefChange((e.target as HTMLSelectElement).value)}
      >
        <option value="" disabled>To…</option>
        {#each commits as c}
          <option value={c.oid}>{formatCommitLabel(c)}</option>
        {/each}
      </select>
    {/if}
  </div>

  <!-- Right section: actions -->
  <div class="flex items-center gap-2">
    <button
      onclick={onRefresh}
      class="rounded p-1.5 text-[var(--header-muted-fg)] hover:bg-[var(--header-hover-bg)] hover:text-[var(--header-fg)]"
      title="Refresh diffs"
    >
      <svg class="h-4 w-4" viewBox="0 0 16 16" fill="currentColor">
        <path
          d="M8 2.5a5.487 5.487 0 0 0-4.131 1.869l1.204 1.204A.25.25 0 0 1 4.896 6H1.25A.25.25 0 0 1 1 5.75V2.104a.25.25 0 0 1 .427-.177l1.38 1.38A7.001 7.001 0 0 1 14.95 7.16a.75.75 0 1 1-1.49.178A5.501 5.501 0 0 0 8 2.5ZM1.705 8.005a.75.75 0 0 1 .834.656 5.501 5.501 0 0 0 9.592 2.97l-1.204-1.204a.25.25 0 0 1 .177-.427h3.646a.25.25 0 0 1 .25.25v3.646a.25.25 0 0 1-.427.177l-1.38-1.38A7.001 7.001 0 0 1 1.05 8.84a.75.75 0 0 1 .656-.834Z"
        />
      </svg>
    </button>

    <button
      onclick={onToggleAiPanel}
      class="rounded p-1.5 text-[var(--header-muted-fg)] hover:bg-[var(--header-hover-bg)] hover:text-[var(--header-fg)]"
      title="Toggle AI panel"
    >
      <svg class="h-4 w-4" viewBox="0 0 16 16" fill="currentColor">
        <path
          d="M5.433 2.304A4.494 4.494 0 0 0 3.5 6c0 1.598.564 3.05 1.47 4.235a6.517 6.517 0 0 0-1.97.77.75.75 0 1 0 .782 1.283c.628-.383 1.37-.662 2.218-.79a4.478 4.478 0 0 0 4 0c.848.128 1.59.407 2.218.79a.75.75 0 1 0 .782-1.283 6.517 6.517 0 0 0-1.97-.77A7.486 7.486 0 0 0 12.5 6a4.494 4.494 0 0 0-1.933-3.696A4.484 4.484 0 0 0 8 1.5a4.484 4.484 0 0 0-2.567.804ZM10 8.5a2 2 0 1 1-4 0 2 2 0 0 1 4 0Z"
        />
      </svg>
    </button>

    <button
      onclick={onToggleTheme}
      class="rounded p-1.5 text-[var(--header-muted-fg)] hover:bg-[var(--header-hover-bg)] hover:text-[var(--header-fg)]"
      title="Toggle theme"
    >
      {#if theme === "dark"}
        <!-- Sun icon (switch to light) -->
        <svg class="h-4 w-4" viewBox="0 0 16 16" fill="currentColor">
          <path
            d="M8 12a4 4 0 1 1 0-8 4 4 0 0 1 0 8Zm0-1.5a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5Zm5.657-8.157a.75.75 0 0 1 0 1.061l-1.061 1.06a.749.749 0 0 1-1.06-1.06l1.06-1.06a.75.75 0 0 1 1.06 0Zm-9.193 9.193a.75.75 0 0 1 0 1.06l-1.06 1.061a.75.75 0 1 1-1.061-1.06l1.06-1.061a.75.75 0 0 1 1.061 0ZM8 0a.75.75 0 0 1 .75.75v1.5a.75.75 0 0 1-1.5 0V.75A.75.75 0 0 1 8 0ZM3 8a.75.75 0 0 1-.75.75H.75a.75.75 0 0 1 0-1.5h1.5A.75.75 0 0 1 3 8Zm13 0a.75.75 0 0 1-.75.75h-1.5a.75.75 0 0 1 0-1.5h1.5A.75.75 0 0 1 16 8Zm-8 5a.75.75 0 0 1 .75.75v1.5a.75.75 0 0 1-1.5 0v-1.5A.75.75 0 0 1 8 13Zm3.536-1.464a.75.75 0 0 1 1.06 0l1.061 1.06a.75.75 0 0 1-1.06 1.061l-1.061-1.06a.75.75 0 0 1 0-1.061ZM2.343 2.343a.75.75 0 0 1 1.061 0l1.06 1.061a.751.751 0 0 1-1.042.018.751.751 0 0 1-.018-1.042l-1.06-1.06a.75.75 0 0 1 0 .023Z"
          />
        </svg>
      {:else}
        <!-- Moon icon (switch to dark) -->
        <svg class="h-4 w-4" viewBox="0 0 16 16" fill="currentColor">
          <path
            d="M9.598 1.591a.749.749 0 0 1 .785-.175 7.001 7.001 0 1 1-8.967 8.967.75.75 0 0 1 .961-.96 5.5 5.5 0 0 0 7.046-7.046.75.75 0 0 1 .175-.786Zm1.616 1.945a7 7 0 0 1-7.678 7.678 5.499 5.499 0 1 0 7.678-7.678Z"
          />
        </svg>
      {/if}
    </button>

    <button
      onclick={onToggleSettings}
      class="rounded p-1.5 text-[var(--header-muted-fg)] hover:bg-[var(--header-hover-bg)] hover:text-[var(--header-fg)]"
      title="Settings"
    >
      <svg class="h-4 w-4" viewBox="0 0 16 16" fill="currentColor">
        <path
          d="M8 0a8.2 8.2 0 0 1 .701.031C9.444.095 9.99.645 10.16 1.29l.288 1.107c.018.066.079.158.212.224.231.114.454.243.668.386.123.082.233.09.299.071l1.103-.303c.644-.176 1.392.021 1.82.63.27.385.506.792.704 1.218.315.675.111 1.422-.364 1.891l-.814.806c-.049.048-.098.147-.088.294a6.1 6.1 0 0 1 0 .772c-.01.147.04.246.088.294l.814.806c.475.469.679 1.216.364 1.891a7.977 7.977 0 0 1-.704 1.217c-.428.61-1.176.807-1.82.63l-1.103-.303c-.066-.018-.176-.011-.299.071a5.991 5.991 0 0 1-.668.386c-.133.066-.194.158-.211.224l-.29 1.106c-.168.646-.715 1.196-1.458 1.26a8.006 8.006 0 0 1-1.402 0c-.743-.064-1.29-.614-1.458-1.26l-.29-1.106c-.017-.066-.078-.158-.211-.224a5.99 5.99 0 0 1-.668-.386c-.123-.082-.233-.09-.299-.071l-1.103.303c-.644.176-1.392-.021-1.82-.63a8.12 8.12 0 0 1-.704-1.218c-.315-.675-.111-1.422.363-1.891l.815-.806c.049-.048.098-.147.088-.294a6.1 6.1 0 0 1 0-.772c.01-.147-.04-.246-.088-.294l-.815-.806C.635 6.045.431 5.298.746 4.623a7.92 7.92 0 0 1 .704-1.217c.428-.61 1.176-.807 1.82-.63l1.103.303c.066.018.176.011.299-.071.214-.143.437-.272.668-.386.133-.066.194-.158.211-.224l.29-1.106C6.009.645 6.556.095 7.299.03 7.53.01 7.764 0 8 0Zm-.571 1.525c-.036.003-.108.036-.137.146l-.289 1.105c-.147.561-.549.967-.998 1.189-.173.086-.34.183-.5.29-.417.278-.97.423-1.529.27l-1.103-.303c-.109-.03-.175.016-.195.046-.219.31-.41.641-.573.989-.014.031-.022.11.059.19l.815.806c.411.406.562.957.53 1.456a4.709 4.709 0 0 0 0 .582c.032.499-.119 1.05-.53 1.456l-.815.806c-.081.08-.073.159-.059.19.162.348.354.68.573.989.02.03.085.076.195.046l1.102-.303c.56-.153 1.113-.008 1.53.27.16.107.327.204.5.29.449.222.851.628.998 1.189l.289 1.105c.029.109.101.143.137.146a6.6 6.6 0 0 0 1.142 0c.036-.003.108-.036.137-.146l.289-1.105c.147-.561.549-.967.998-1.189.173-.086.34-.183.5-.29.417-.278.97-.423 1.529-.27l1.103.303c.109.03.175-.016.195-.046.219-.31.41-.641.573-.989.014-.031.022-.11-.059-.19l-.815-.806c-.411-.406-.562-.957-.53-1.456a4.709 4.709 0 0 0 0-.582c-.032-.499.119-1.05.53-1.456l.815-.806c.081-.08.073-.159.059-.19a6.464 6.464 0 0 0-.573-.989c-.02-.03-.085-.076-.195-.046l-1.102.303c-.56.153-1.113.008-1.53-.27a4.44 4.44 0 0 0-.501-.29c-.447-.222-.85-.628-.997-1.189l-.289-1.105c-.029-.11-.101-.143-.137-.146a6.6 6.6 0 0 0-1.142 0ZM11 8a3 3 0 1 1-6 0 3 3 0 0 1 6 0ZM9.5 8a1.5 1.5 0 1 0-3.001.001A1.5 1.5 0 0 0 9.5 8Z"
        />
      </svg>
    </button>

    <div class="mx-1 h-6 w-px bg-[var(--header-border)]"></div>

    <button
      onclick={onSubmitReview}
      class="flex items-center gap-1.5 rounded-md px-3 py-1.5 text-sm font-medium text-white"
      style="background-color: var(--btn-primary-bg);"
      onmouseenter={(e) => (e.currentTarget.style.backgroundColor = 'var(--btn-primary-hover-bg)')}
      onmouseleave={(e) => (e.currentTarget.style.backgroundColor = 'var(--btn-primary-bg)')}
    >
      <svg class="h-4 w-4" viewBox="0 0 16 16" fill="currentColor">
        <path
          d="M1.5 3.25a2.25 2.25 0 1 1 3 2.122v5.256a2.251 2.251 0 1 1-1.5 0V5.372A2.25 2.25 0 0 1 1.5 3.25Zm5.677-.177L9.573.677A.25.25 0 0 1 10 .854V2.5h1A2.5 2.5 0 0 1 13.5 5v5.628a2.251 2.251 0 1 1-1.5 0V5a1 1 0 0 0-1-1h-1v1.646a.25.25 0 0 1-.427.177L7.177 3.427a.25.25 0 0 1 0-.354ZM3.75 2.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5Zm0 9.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5Zm8.25.75a.75.75 0 1 0 1.5 0 .75.75 0 0 0-1.5 0Z"
        />
      </svg>
      Submit Review
      {#if commentCount > 0}
        <span
          class="ml-1 rounded-full bg-white/20 px-1.5 py-0.5 text-xs leading-none"
        >
          {commentCount}
        </span>
      {/if}
    </button>
  </div>
</header>
