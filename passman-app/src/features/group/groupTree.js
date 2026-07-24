export function buildTree(groups, parentId = null, depth = 0) {
  const result = [];
  for (const group of groups) {
    const isRoot = !group.parent_id || group.parent_id === "0";
    if (group.parent_id === parentId || (parentId === null && isRoot)) {
      result.push({
        group,
        depth,
        children: buildTree(groups, group.id, depth + 1),
      });
    }
  }
  return result;
}
