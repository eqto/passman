<script>
  let { onkeydown = null, children } = $props();
</script>

<div class="modal-overlay">
  <div class="modal" role="dialog" aria-modal="true" tabindex="-1" {onkeydown}>
    {@render children?.()}
  </div>
</div>
