<script>
  export let group = "";
  export let onDelete;
  export let onCancel;
</script>

<div class="modal-overlay">
  <div class="modal">
    <h2>Delete Group</h2>
    <p>Delete group "{group}"? Entries in this group will keep their other tags.</p>
    <div class="modal-actions">
      <button class="modal-cancel-btn" on:click={onCancel}>
        Cancel
      </button>
      <button class="btn-primary" on:click={() => onDelete(group)}>
        Delete
      </button>
    </div>
  </div>
</div>
