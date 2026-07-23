/**
 * Split an entry's tags into group tags and free-form tags.
 *
 * With the hierarchical group format, entries use `group_id` for group
 * membership and `tags` for free-form labels only. This helper now returns
 * all tags as free-form tags and an empty groupTags list.
 *
 * @param {string[]} entryTags
 * @returns {{ groupTags: string[]; freeTags: string[] }}
 */
export function splitTags(entryTags) {
  return { groupTags: [], freeTags: entryTags || [] };
}

/**
 * Return only the free-form tags from an entry.
 */
export function freeTags(entryTags) {
  return (entryTags || []).slice().sort((a, b) => a.localeCompare(b));
}

/**
 * Return only the group membership tags from an entry. Always empty in the
 * new format because group membership is stored on `group_id`.
 */
export function groupTags() {
  return [];
}
