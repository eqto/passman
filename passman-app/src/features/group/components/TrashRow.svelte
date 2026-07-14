<script>
  import { Icon } from "../../../components/icons";

  let { trashMode = false, onTrashClick } = $props();
</script>

<div class="group-row trash-row" class:selected={trashMode}>
  <div
    class="group-item"
    role="button"
    tabindex="0"
    onclick={onTrashClick}
    onkeydown={(e) => {
      if (e.key === "Enter" || e.key === " ") {
        e.preventDefault();
        onTrashClick();
      }
    }}
  >
    <span class="group-icon trash-icon">
      <Icon name="trash" size={18} />
    </span>
    <span class="group-name">Trash</span>
  </div>
</div>

<style>
  .group-row {
    display: flex;
    align-items: center;
    cursor: grab;
  }

  .group-row:hover {
    background-color: var(--hover-bg);
  }

  .group-row.selected {
    background-color: var(--selected-bg);
  }

  .group-row.selected .group-item {
    color: var(--selected-text);
  }

  .group-row:not(.selected) .group-item {
    color: var(--muted-color);
  }

  .group-row:not(.selected):hover .group-item {
    color: var(--text-color);
  }

  .group-item {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: flex-start;
    gap: 0.35rem;
    padding: 1rem 0.5rem;
    background: transparent;
    border: none;
    border-radius: 0;
    cursor: pointer;
    text-align: left;
    font-size: 0.875rem;
    font-weight: 400;
    color: var(--text-color);
    line-height: 1.5;
  }

  .group-icon {
    font-size: 1rem;
    opacity: 0.8;
  }

  .trash-icon {
    display: inline-flex;
    align-items: center;
    vertical-align: middle;
    transform: translateY(-2px);
    color: var(--muted-color);
    opacity: 0.9;
  }

  .group-row.selected .trash-icon {
    color: var(--selected-text);
  }

  .trash-row .group-name {
    transform: translateY(1px);
  }

  .group-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
