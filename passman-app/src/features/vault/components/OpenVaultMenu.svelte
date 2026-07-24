<script>
  import { Icon } from "../../../components/icons";

  let {
    showDropdown = $bindable(false),
    dropdownPosition = $bindable({ x: 0, y: 0 }),
    onpickexisting = null,
    onbuttercupimport = null,
    onkeepassimport = null,
  } = $props();

  function pickExistingVault() {
    onpickexisting?.();
  }

  function toggleOpenDropdown(event) {
    event.stopPropagation();
    const rect = event.currentTarget.getBoundingClientRect();
    const menuWidth = 180;
    const windowWidth = window.innerWidth;

    let x = rect.left;
    if (x + menuWidth > windowWidth) {
      x = windowWidth - menuWidth - 16;
    }

    dropdownPosition = {
      x,
      y: rect.bottom + 4,
    };
    showDropdown = !showDropdown;
  }

  function handleButtercupImport() {
    showDropdown = false;
    onbuttercupimport?.();
  }

  function handleKeePassImport() {
    showDropdown = false;
    onkeepassimport?.();
  }
</script>

<div class="dropdown-button">
  <button class="dropdown-main-btn" onclick={pickExistingVault}>
    <Icon name="folder" size={16} />
    <span class="dropdown-label">Open Vault</span>
  </button>
  <div class="dropdown-separator"></div>
  <button
    class="dropdown-toggle"
    onclick={toggleOpenDropdown}
    title="More import options"
    aria-label="More import options"
  >
    <Icon name="chevron" size={16} direction="down" />
  </button>
</div>
{#if showDropdown}
  <div
    class="dropdown-menu"
    style="left: {dropdownPosition.x}px; top: {dropdownPosition.y}px;"
  >
    <button class="dropdown-item" onclick={handleButtercupImport}>
      Open Buttercup format
    </button>
    <button class="dropdown-item" onclick={handleKeePassImport}>
      Open KeePass format
    </button>
  </div>
{/if}

<style>
  .dropdown-button {
    position: relative;
    display: flex;
    align-items: center;
    padding: 0 0.5rem 0 0.75rem;
    background-color: var(--accent-container);
    color: var(--on-accent-container);
    border-radius: var(--shape-full);
    gap: 0.25rem;
    height: var(--btn-height);
  }

  .dropdown-button::after {
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

  .dropdown-button:hover::after {
    opacity: 0.08;
  }

  .dropdown-button:active::after {
    opacity: 0.12;
  }

  .dropdown-main-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    background: transparent;
    border: none;
    color: inherit;
    cursor: pointer;
    font-size: var(--font-size-sm);
    font-weight: 500;
  }

  .dropdown-separator {
    width: 1px;
    height: 1rem;
    background-color: currentColor;
    opacity: 0.4;
  }

  .dropdown-toggle {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.75rem;
    height: 1.75rem;
    padding: 0;
    background: transparent;
    border: none;
    color: inherit;
    cursor: pointer;
    border-radius: var(--shape-full);
    transition: background-color var(--motion-duration-short-2)
      var(--motion-easing-standard);
  }

  .dropdown-toggle:hover {
    background-color: var(--accent-hover);
  }

  .dropdown-menu {
    position: fixed;
    background-color: var(--card-bg);
    border: none;
    border-radius: var(--shape-sm);
    box-shadow:
      0 0 1px 0 rgba(0, 0, 0, 0.3),
      0 1px 2px 0 rgba(0, 0, 0, 0.3),
      0 2px 6px 2px rgba(0, 0, 0, 0.15);
    z-index: 9999;
    min-width: auto;
    width: fit-content;
    padding: var(--space-1);
  }

  .dropdown-item {
    width: 100%;
    padding: var(--space-2) var(--space-3);
    text-align: left;
    background: transparent;
    border: none;
    color: var(--text-color);
    cursor: pointer;
    font-size: var(--font-size-sm);
    font-weight: 400;
    border-radius: var(--shape-xs);
    display: flex;
    align-items: center;
    gap: 0.5rem;
    white-space: nowrap;
  }

  .dropdown-item:hover {
    background-color: var(--hover-bg);
  }
</style>
