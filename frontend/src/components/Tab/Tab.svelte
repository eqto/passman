<script>
  import { getContext, onMount } from "svelte";
  import { nextTabId } from "./tab-id.js";

  let { name = null, label = null, title = null, children } = $props();

  const tabs = getContext("tabs");
  let id;

  onMount(() => {
    id = name ?? nextTabId();
    tabs.registerTab({ id, name: label ?? name, title, content: children });
    return () => tabs.unregisterTab(id);
  });
</script>
