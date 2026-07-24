<script>
  import { onMount } from "svelte";
  import * as passwordService from "../../bindings/github.com/eqto/passman/internal/app/passwordservice.js";
  import { DEFAULT_PASSWORD_LENGTH } from "../lib/constants.js";
  import { Icon } from "./icons";

  let { onuse = null } = $props();

  let open = $state(false);
  let length = $state(DEFAULT_PASSWORD_LENGTH);
  let options = $state({
    uppercase: true,
    lowercase: true,
    digits: true,
    space: false,
    underscoreDash: true,
    symbols: false,
  });
  let generated = $state("");
  let error = $state("");
  let panelElement;
  let generateBtn;
  let panelTop = $state(0);
  let panelLeft = $state(0);
  const PANEL_WIDTH = 288;

  const optionLabels = {
    uppercase: "Uppercase",
    lowercase: "Lowercase",
    digits: "Digits",
    space: "Space",
    underscoreDash: "Underscore & Dash",
    symbols: "Symbols",
  };

  async function generate() {
    error = "";
    try {
      generated = await passwordService.GeneratePassword({
        length,
        uppercase: options.uppercase ?? true,
        lowercase: options.lowercase ?? true,
        digits: options.digits ?? true,
        space: options.space ?? false,
        underscore_dash: options.underscoreDash ?? true,
        symbols: options.symbols ?? false,
      });
    } catch (e) {
      error = e.toString();
    }
  }

  function updatePanelPosition() {
    if (!generateBtn) return;
    const rect = generateBtn.getBoundingClientRect();
    const vw = window.innerWidth;
    const vh = window.innerHeight;
    const gap = 4;

    let top = rect.bottom + gap;
    let left = rect.right - PANEL_WIDTH;

    if (panelElement) {
      const panelHeight = panelElement.offsetHeight;
      if (top + panelHeight > vh) {
        top = Math.max(gap, rect.top - panelHeight - gap);
      }
    }

    left = Math.max(gap, Math.min(left, vw - PANEL_WIDTH - gap));

    panelTop = top;
    panelLeft = left;
  }

  function toggle() {
    open = !open;
    if (open) {
      generate();
      requestAnimationFrame(() => requestAnimationFrame(updatePanelPosition));
    }
  }

  function usePassword() {
    if (generated) {
      onuse?.(generated);
      open = false;
    }
  }

  function handleWindowClick(event) {
    if (
      open &&
      panelElement &&
      !panelElement.contains(event.target) &&
      !event.target.closest(".password-generator")
    ) {
      open = false;
    }
  }

  function handleKeydown(event) {
    if (open && event.key === "Escape") {
      open = false;
    }
  }

  onMount(() => {
    return () => {
      open = false;
    };
  });
</script>

<svelte:window onclick={handleWindowClick} onkeydown={handleKeydown} />

<div class="password-generator">
  <button
    bind:this={generateBtn}
    class="btn-icon generate-btn"
    type="button"
    aria-label="Generate password"
    onclick={toggle}
  >
    <Icon name="key" size={18} />
  </button>

  {#if open}
    <div
      class="password-generator-panel"
      bind:this={panelElement}
      style="top: {panelTop}px; left: {panelLeft}px;"
    >
      <div class="generated-password" title={generated}>
        {generated}
      </div>

      <div class="options-section">
        <div class="options-header">
          <span class="options-label">Options</span>
          <span class="length-value">{length}</span>
        </div>
        <input
          class="length-slider"
          type="range"
          min="6"
          max="64"
          bind:value={length}
          oninput={generate}
        />

        <div class="option-list">
          {#each Object.entries(optionLabels) as [key, label]}
            <label class="option-item">
              <input
                type="checkbox"
                bind:checked={options[key]}
                onchange={generate}
              />
              <span>{label}</span>
            </label>
          {/each}
        </div>
      </div>

      {#if error}
        <p class="generator-error">{error}</p>
      {/if}

      <div class="generator-actions">
        <button class="btn-secondary" type="button" onclick={generate}>
          Generate
        </button>
        <button class="btn-primary" type="button" onclick={usePassword}>
          Use This
        </button>
      </div>
    </div>
  {/if}
</div>

<style>
  .password-generator {
    position: relative;
    display: inline-flex;
    align-items: center;
  }

  .generate-btn {
    width: var(--btn-height);
    height: var(--btn-height);
    padding: 0;
    flex-shrink: 0;
    background-color: var(--hover-bg);
    color: var(--text-color);
    border-radius: var(--shape-sm);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    transition: background-color var(--motion-duration-short-2)
      var(--motion-easing-standard);
  }

  .generate-btn:hover {
    background-color: var(--hover-bg);
  }

  .generate-btn::after {
    content: "";
    position: absolute;
    inset: 0;
    border-radius: inherit;
    background-color: currentColor;
    opacity: 0;
    pointer-events: none;
    transition: opacity var(--motion-duration-short-2)
      var(--motion-easing-standard);
  }

  .generate-btn:hover::after {
    opacity: 0.08;
  }

  .generate-btn:active::after {
    opacity: 0.12;
  }

  .password-generator-panel {
    position: fixed;
    width: 18rem;
    max-width: calc(100vw - 2rem);
    background-color: var(--card-bg);
    border: none;
    border-radius: var(--shape-md);
    box-shadow:
      0 0 1px 0 rgba(0, 0, 0, 0.3),
      0 1px 3px 0 rgba(0, 0, 0, 0.3),
      0 4px 8px 3px rgba(0, 0, 0, 0.15);
    padding: var(--space-3);
    z-index: 100;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .generated-password {
    width: 100%;
    padding: 0.375rem 0.5rem;
    background-color: var(--input-bg);
    border: 1px solid var(--input-border);
    border-radius: var(--shape-xs);
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
      monospace;
    font-size: 0.8125rem;
    word-break: break-all;
    color: var(--text-color);
    min-height: 1.875rem;
    display: flex;
    align-items: center;
  }

  .options-section {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .options-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .options-label {
    font-size: 0.6875rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--muted-color);
  }

  .length-value {
    font-size: 0.8125rem;
    font-weight: 600;
    color: var(--text-color);
    min-width: 1.25rem;
    text-align: right;
  }

  .length-slider {
    width: 100%;
    height: 1rem;
    cursor: pointer;
    appearance: none;
    background: transparent;
  }

  .length-slider::-webkit-slider-runnable-track {
    height: 0.25rem;
    background-color: var(--input-border);
    border-radius: var(--shape-full);
  }

  .length-slider::-moz-range-track {
    height: 0.25rem;
    background-color: var(--input-border);
    border-radius: var(--shape-full);
  }

  .length-slider::-webkit-slider-thumb {
    appearance: none;
    width: 1.25rem;
    height: 1.25rem;
    margin-top: -0.5rem;
    background-color: var(--accent-color);
    border: none;
    border-radius: var(--shape-full);
    cursor: pointer;
  }

  .length-slider::-moz-range-thumb {
    width: 1.25rem;
    height: 1.25rem;
    background-color: var(--accent-color);
    border: none;
    border-radius: var(--shape-full);
    cursor: pointer;
  }

  .option-list {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.125rem;
  }

  .option-item {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.125rem 0.25rem;
    border-radius: var(--shape-xs);
    cursor: pointer;
    font-size: 0.8125rem;
    color: var(--text-color);
  }

  .option-item:hover {
    background-color: var(--hover-bg);
  }

  .option-item input[type="checkbox"] {
    width: 1.125rem;
    height: 1.125rem;
    accent-color: var(--accent-color);
    cursor: pointer;
    flex-shrink: 0;
  }

  .generator-actions {
    display: flex;
    gap: 0.375rem;
    margin-top: 0.125rem;
  }

  .generator-actions button {
    flex: 1;
    justify-content: center;
  }

  .generator-error {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--danger-color);
  }
</style>
