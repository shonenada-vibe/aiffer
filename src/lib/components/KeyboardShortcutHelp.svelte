<script lang="ts">
  interface Props {
    isOpen: boolean;
    onClose: () => void;
  }

  let { isOpen, onClose }: Props = $props();

  const SHORTCUT_GROUPS = [
    {
      title: "Navigation",
      shortcuts: [
        { keys: ["n"], description: "Next file" },
        { keys: ["p"], description: "Previous file" },
        { keys: ["j"], description: "Next hunk" },
        { keys: ["k"], description: "Previous hunk" },
      ],
    },
    {
      title: "Actions",
      shortcuts: [
        { keys: ["c"], description: "Open comment form on current line" },
        { keys: ["⌘", "Enter"], description: "Submit review" },
        { keys: ["⌘", "⇧", "R"], description: "Refresh diffs" },
      ],
    },
    {
      title: "General",
      shortcuts: [
        { keys: ["?"], description: "Show this help" },
        { keys: ["Esc"], description: "Close dialog / cancel" },
      ],
    },
  ];
</script>

{#if isOpen}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
    role="button"
    tabindex="-1"
    onclick={onClose}
    onkeydown={(e) => {
      if (e.key === "Escape") onClose();
    }}
  >
    <!-- Modal -->
    <!-- svelte-ignore a11y_interactive_supports_focus -->
    <div
      class="w-full max-w-md rounded-lg border border-[var(--panel-border)] bg-[var(--panel-bg)] shadow-xl"
      role="dialog"
      aria-label="Keyboard shortcuts"
      onclick={(e) => e.stopPropagation()}
      onkeydown={() => {}}
    >
      <div
        class="flex items-center justify-between border-b border-[var(--panel-border)] px-5 py-3"
      >
        <h2 class="text-base font-semibold text-[var(--app-fg)]">
          Keyboard Shortcuts
        </h2>
        <button
          class="rounded p-1 text-[var(--panel-muted-fg)] hover:bg-[var(--panel-muted-bg)] hover:text-[var(--app-fg)]"
          onclick={onClose}
          aria-label="Close keyboard shortcuts"
        >
          <svg class="h-4 w-4" viewBox="0 0 16 16" fill="currentColor">
            <path
              d="M3.72 3.72a.75.75 0 0 1 1.06 0L8 6.94l3.22-3.22a.749.749 0 0 1 1.275.326.749.749 0 0 1-.215.734L9.06 8l3.22 3.22a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215L8 9.06l-3.22 3.22a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042L6.94 8 3.72 4.78a.75.75 0 0 1 0-1.06Z"
            />
          </svg>
        </button>
      </div>

      <div class="px-5 py-4">
        {#each SHORTCUT_GROUPS as group}
          <div class="mb-4 last:mb-0">
            <h3 class="mb-2 text-xs font-semibold uppercase text-[var(--panel-muted-fg)]">
              {group.title}
            </h3>
            <div class="space-y-1.5">
              {#each group.shortcuts as shortcut}
                <div class="flex items-center justify-between">
                  <span class="text-sm text-[var(--app-fg)]"
                    >{shortcut.description}</span
                  >
                  <div class="flex gap-1">
                    {#each shortcut.keys as key}
                      <kbd
                        class="rounded border border-[var(--input-border)] bg-[var(--input-muted-bg)] px-1.5 py-0.5 font-mono text-xs text-[var(--panel-muted-fg)] shadow-sm"
                      >
                        {key}
                      </kbd>
                    {/each}
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/each}
      </div>
    </div>
  </div>
{/if}
