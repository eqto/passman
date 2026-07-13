<script>
  import { toast } from "../../stores/toast.js";
</script>

{#if $toast.visible}
  <div class="toast">{$toast.message}</div>
{/if}

<style>
  .toast {
    position: fixed;
    bottom: 1rem;
    left: 50%;
    transform: translateX(-50%);
    background-color: var(--toast-bg);
    color: var(--toast-text);
    padding: var(--space-3) var(--space-4);
    border-radius: var(--shape-sm);
    box-shadow:
      0 0 1px 0 rgba(0, 0, 0, 0.3),
      0 1px 3px 0 rgba(0, 0, 0, 0.3),
      0 4px 8px 3px rgba(0, 0, 0, 0.15);
    font-size: var(--font-size-sm);
    z-index: 300;
    pointer-events: none;
  }
</style>
