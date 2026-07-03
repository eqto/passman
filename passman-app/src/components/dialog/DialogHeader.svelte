<script>
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();
</script>

<div class="modal-header">
  <h2><slot /></h2>
  <button class="btn-icon" aria-label="Close" on:click={() => dispatch("close")}>
    ×
  </button>
</div>
