<script>
  export let customFields = [];
  export let onChange = (fields) => {};

  const FIELD_TYPES = [
    { value: "text", label: "Text" },
    { value: "note", label: "Note" },
    { value: "password", label: "Password" },
    { value: "otp", label: "OTP" },
  ];

  let openMenuId = null;

  function addField() {
    const next = [
      ...customFields,
      {
        id: crypto.randomUUID(),
        label: "Click to edit",
        type: "text",
        value: "",
      },
    ];
    onChange(next);
  }

  function updateField(id, patch) {
    const next = customFields.map((f) =>
      f.id === id ? { ...f, ...patch } : f
    );
    onChange(next);
  }

  function removeField(id) {
    const next = customFields.filter((f) => f.id !== id);
    onChange(next);
  }

  function setType(id, type) {
    updateField(id, { type });
    openMenuId = null;
  }

  function toggleMenu(id) {
    openMenuId = openMenuId === id ? null : id;
  }

  function handleClickOutside(event) {
    if (!event.target.closest(".custom-field-menu")) {
      openMenuId = null;
    }
  }
</script>

<svelte:window on:click={handleClickOutside} />

<div class="custom-fields">
  {#each customFields as field (field.id)}
    <div class="custom-field">
      <input
        class="custom-field-name"
        type="text"
        value={field.label}
        on:input={(e) => updateField(field.id, { label: e.target.value })}
      />
      {#if field.type === "note"}
        <textarea
          class="custom-field-value"
          rows="3"
          value={field.value}
          on:input={(e) => updateField(field.id, { value: e.target.value })}
        ></textarea>
      {:else if field.type === "password"}
        <input
          class="custom-field-value"
          type="password"
          value={field.value}
          on:input={(e) => updateField(field.id, { value: e.target.value })}
        />
      {:else}
        <input
          class="custom-field-value"
          type="text"
          value={field.value}
          on:input={(e) => updateField(field.id, { value: e.target.value })}
        />
      {/if}
      <div class="custom-field-menu">
        <button
          class="btn-icon gear-btn"
          type="button"
          aria-label="Field options"
          on:click|stopPropagation={() => toggleMenu(field.id)}
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1.82.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V15a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path></svg>
        </button>
        {#if openMenuId === field.id}
          <div class="menu-dropdown">
            {#each FIELD_TYPES as type}
              <button
                type="button"
                class="menu-item"
                class:active={field.type === type.value}
                on:click|stopPropagation={() => setType(field.id, type.value)}
              >
                {type.label}
              </button>
            {/each}
            <div class="menu-divider"></div>
            <button
              type="button"
              class="menu-item danger"
              on:click|stopPropagation={() => removeField(field.id)}
            >
              Remove
            </button>
          </div>
        {/if}
      </div>
    </div>
  {/each}
  <button class="btn-secondary add-field-btn" type="button" on:click={addField}>
    + Add custom field
  </button>
</div>

<style>
  .custom-fields {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .custom-field {
    display: flex;
    align-items: flex-start;
    gap: 0.5rem;
  }

  .custom-field-name {
    width: 8rem;
    flex-shrink: 0;
  }

  .custom-field-value {
    flex: 1;
    min-width: 0;
  }

  .custom-field textarea {
    resize: vertical;
  }

  .custom-field-menu {
    position: relative;
    flex-shrink: 0;
  }

  .gear-btn {
    width: 2.25rem;
    height: 2.25rem;
    padding: 0;
  }

  .menu-dropdown {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 0.25rem;
    background-color: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
    padding: 0.25rem;
    z-index: 10;
    min-width: 8rem;
  }

  .menu-item {
    width: 100%;
    padding: 0.5rem 0.75rem;
    text-align: left;
    background: transparent;
    border: none;
    border-radius: 0.25rem;
    color: var(--text-color);
    font-size: 0.875rem;
    cursor: pointer;
  }

  .menu-item:hover,
  .menu-item.active {
    background-color: var(--hover-bg);
  }

  .menu-item.danger {
    color: var(--danger-color);
  }

  .menu-divider {
    height: 1px;
    background-color: var(--border-color);
    margin: 0.25rem 0.5rem;
  }

  .add-field-btn {
    align-self: flex-start;
  }
</style>
