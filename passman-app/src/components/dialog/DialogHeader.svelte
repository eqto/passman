<script>
  let { onclick = null, children } = $props();
</script>

<div class="modal-header">
  <h2>{@render children?.()}</h2>
  <button class="btn-icon" aria-label="Close" {onclick}> × </button>
</div>
