<script>
  import { getContext, onMount } from "svelte";
  import { nextTabId } from "./tab-id.js";

  let { name = null, title = null, label = null, children } = $props();

  const tabs = getContext("tabs");
  let id;

  onMount(() => {
    id = name ?? nextTabId();
    tabs.registerTab({ id, name, title, label, content: children });
    return () => tabs.unregisterTab(id);
  });
</script>
