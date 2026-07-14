<script>
  import { getContext } from "svelte";
  import { nextTabId } from "./tab-id.js";

  let { name = null, title = null, children } = $props();

  const tabs = getContext("tabs");
  const id = name ?? nextTabId();

  $effect.pre(() => {
    tabs.registerTab({ id, name, title, content: children });
    return () => tabs.unregisterTab(id);
  });
</script>
