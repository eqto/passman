<script>
  import Chip from "../../../components/form/Chip.svelte";
  import { GroupTitle } from "../index";

  export let tags = [];
  export let selectedTags = [];
  export let onSelectTag = (tag) => {};
  export let onAddTag = () => {};
  export let onContextMenu = (e, tag) => {};
</script>

<GroupTitle title="Tags" showButton={true} onButtonClick={onAddTag} />

{#if tags.length === 0}
  <p class="empty-state">No tags.</p>
{:else}
  <div class="tags">
    {#each tags as tag}
      <Chip
        size="medium"
        active={selectedTags.includes(tag)}
        on:click={() => onSelectTag(tag)}
        on:contextmenu={(e) => onContextMenu(e, tag)}
        on:keydown={(e) => {
          if (e.key === "Enter" || e.key === " ") {
            e.preventDefault();
            onSelectTag(tag);
          }
        }}
      >
        {tag}
      </Chip>
    {/each}
  </div>
{/if}

<style>
  .tags {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    padding: 0 0.5rem;
  }

  :global(.chip:hover) {
    background-color: var(--accent-color);
    color: #ffffff;
    border-color: var(--accent-color);
  }
</style>
