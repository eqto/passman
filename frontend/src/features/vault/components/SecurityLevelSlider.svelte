<script>
  let { value = "medium", disabled = false } = $props();

  const levels = [
    {
      key: "low",
      label: "Low",
      time: "~0.3s",
      description:
        "Faster unlock. Recommended for mobile and low-end devices. Suitable for low-sensitivity vaults with a strong password.",
    },
    {
      key: "medium",
      label: "Medium",
      time: "~0.8s",
      recommended: true,
      description:
        "Balanced speed and protection. Recommended for most desktop users. May be slow on older mobile devices.",
    },
    {
      key: "secure",
      label: "Secure",
      time: "~2s",
      description:
        "Strong brute-force resistance for sensitive credentials. Desktop only — may crash on low-memory mobile devices.",
    },
    {
      key: "best",
      label: "Best",
      time: "~5s",
      description:
        "Maximum protection. Slower unlock — best for your most critical vaults. Desktop recommended — requires 256MB+ free memory.",
    },
  ];

  let selected = $derived(levels.find((l) => l.key === value) || levels[1]);
</script>

<div class="security-slider" class:disabled>
  <div class="slider-track">
    {#each levels as level, i (level.key)}
      <button
        type="button"
        class="slider-option"
        class:active={value === level.key}
        class:first={i === 0}
        class:last={i === levels.length - 1}
        onclick={() => (value = level.key)}
        {disabled}
      >
        <span class="option-label">{level.label}</span>
        <span class="option-time">{level.time}</span>
        {#if level.recommended}
          <span class="recommended-badge">Recommended</span>
        {/if}
      </button>
    {/each}
  </div>
  <p class="slider-description">{selected.description}</p>
</div>

<style>
  .security-slider {
    display: flex;
    flex-direction: column;
    gap: var(--space-2, 0.5rem);
  }

  .slider-track {
    display: flex;
    gap: 2px;
    background-color: var(--input-border);
    border-radius: var(--shape-sm, 8px);
    overflow: hidden;
    padding: 2px;
  }

  .slider-option {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
    padding: var(--space-2, 0.5rem) var(--space-1, 0.25rem);
    border: none;
    background-color: var(--input-bg);
    color: var(--text-color);
    cursor: pointer;
    border-radius: var(--shape-xs, 6px);
    transition:
      background-color 0.15s ease,
      color 0.15s ease;
    position: relative;
  }

  .slider-option:hover:not(.active):not(:disabled) {
    background-color: var(--hover-bg);
  }

  .slider-option.active {
    background-color: var(--accent-color);
    color: var(--on-primary);
  }

  .slider-option:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .option-label {
    font-size: var(--font-size-sm, 0.875rem);
    font-weight: 600;
  }

  .option-time {
    font-size: var(--font-size-xs, 0.75rem);
    opacity: 0.7;
  }

  .recommended-badge {
    font-size: 0.625rem;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    background-color: var(--accent-container);
    color: var(--on-accent-container);
    padding: 1px 6px;
    border-radius: var(--shape-full, 9999px);
    margin-top: 2px;
  }

  .slider-option.active .recommended-badge {
    background-color: rgba(255, 255, 255, 0.25);
    color: var(--on-primary);
  }

  .slider-description {
    font-size: var(--font-size-sm, 0.875rem);
    color: var(--muted-color);
    margin: 0;
    line-height: 1.4;
    min-height: 2.5em;
  }

  .security-slider.disabled .slider-description {
    opacity: 0.5;
  }
</style>
