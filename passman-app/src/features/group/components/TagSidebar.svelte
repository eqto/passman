<script>
  import Chip from "../../../components/form/Chip.svelte";
  import { GroupTitle } from "../index";

  let {
    tags = [],
    selectedTags = [],
    onSelectTag = (tag) => {},
    onContextMenu = (e, tag) => {},
  } = $props();
</script>

<GroupTitle title="Tags" showButton={false} />

{#if tags.length === 0}
  <p class="empty-state">No tags.</p>
{:else}
  <div class="tags">
    {#each tags as tag}
      <Chip
        size="medium"
        active={selectedTags.includes(tag)}
        onclick={() => onSelectTag(tag)}
        oncontextmenu={(e) => onContextMenu(e, tag)}
        onkeydown={(e) => {
          if (e.key === "Enter" || e.key === " ") {
            e.preventDefault();
            onSelectTag(tag);
          }
        }}
        text={tag}
      />
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
</style>
