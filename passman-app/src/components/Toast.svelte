<script>
  import { toast } from "../stores/toast.js";
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
    background-color: #1f2937;
    color: white;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
    font-size: 0.875rem;
    z-index: 300;
    pointer-events: none;
  }
</style>
