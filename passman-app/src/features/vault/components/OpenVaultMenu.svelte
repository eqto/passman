<script>
  import { Icon } from "../../../components/icons";

  let {
    showDropdown = $bindable(false),
    dropdownPosition = $bindable({ x: 0, y: 0 }),
    onpickexisting = null,
    onbuttercupimport = null,
  } = $props();

  function pickExistingVault() {
    onpickexisting?.();
  }

  function toggleOpenDropdown(event) {
    event.stopPropagation();
    const rect = event.currentTarget.getBoundingClientRect();
    const menuWidth = 120;
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
</script>

<div class="btn-secondary dropdown-button">
  <button class="dropdown-main-btn" onclick={pickExistingVault}
    >Open Vault</button
  >
  <div class="dropdown-separator"></div>
  <button
    class="btn-icon dropdown-toggle"
    onclick={toggleOpenDropdown}
    title="Open options"
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
  </div>
{/if}

<style>
  .dropdown-button {
    position: relative;
    display: flex;
    align-items: center;
    padding: 0.5rem 0.75rem 0.25rem;
    background-color: var(--hover-bg);
    color: var(--text-color);
    border-radius: var(--shape-sm);
    gap: 0.375rem;
    height: 2.25rem;
    line-height: 1.5rem;
  }

  .dropdown-button:hover {
    background-color: var(--hover-bg);
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
    background: transparent;
    border: none;
    padding: 0;
    color: var(--text-color);
    cursor: pointer;
    font-size: 0.875rem;
    font-weight: 500;
    line-height: 1.25;
  }

  .dropdown-separator {
    width: 1px;
    height: 1.25rem;
    background-color: var(--border-color);
  }

  .dropdown-menu {
    position: fixed;
    background-color: var(--sidebar-bg);
    border: none;
    border-radius: var(--shape-sm);
    box-shadow:
      0 0 1px 0 rgba(0, 0, 0, 0.3),
      0 1px 2px 0 rgba(0, 0, 0, 0.3),
      0 2px 6px 2px rgba(0, 0, 0, 0.15);
    z-index: 9999;
    min-width: auto;
    width: fit-content;
  }

  .dropdown-item {
    width: 100%;
    padding: 0.375rem 0.625rem;
    text-align: left;
    background: transparent;
    border: none;
    color: var(--text-color);
    cursor: pointer;
    font-size: 0.8rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .dropdown-item:hover {
    background-color: var(--hover-bg);
  }
</style>
