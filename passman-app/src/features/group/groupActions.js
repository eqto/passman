import { moveGroupToParent, reorderGroups } from "./store.js";
import { showToast } from "../../stores/toast.js";

export function isDescendant(groupId, potentialParentId, groups) {
  if (groupId === potentialParentId) return true;
  const group = groups.find((g) => g.id === groupId);
  if (!group || !group.parent_id) return false;
  return isDescendant(group.parent_id, potentialParentId, groups);
}

export async function onReorderGroups(items, { source, target } = {}) {
  const normalizeParent = (id) => (id && id !== "0" ? id : null);
  const sourceParent = normalizeParent(source?.parent_id);
  const targetParent = normalizeParent(target?.parent_id);
  if (source && target && sourceParent !== targetParent) {
    if (isDescendant(target.id, source.id, items)) {
      showToast("Cannot move a group into its own descendant");
      return;
    }
    const updatedGroups = await moveGroupToParent(source.id, targetParent);
    if (!updatedGroups) return;
    const reordered = items.map(
      (item) => updatedGroups.find((g) => g.id === item.id) ?? item,
    );
    await reorderGroups(reordered);
    return;
  }
  reorderGroups(items);
}

export async function handleDropInto(source, target, groups) {
  const sourceId = source.id;
  const targetId = target.id;

  if (sourceId === targetId) return;

  if (isDescendant(targetId, sourceId, groups)) {
    showToast("Cannot drop a group into its own descendant");
    return;
  }

  try {
    await moveGroupToParent(sourceId, targetId);
  } catch (e) {
    console.error("Failed to move group to parent:", e);
    showToast("Failed to move group");
  }
}
