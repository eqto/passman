/**
 * Split an entry's tags into group tags and free-form tags.
 *
 * In Passman, an entry's `tags` array stores both group membership (tags that
 * match the vault's group list) and free-form labels. This helper returns the
 * two sets separately so callers don't re-implement the filter logic.
 *
 * @param {string[]} entryTags
 * @param {string[]} groups
 * @returns {{ groupTags: string[]; freeTags: string[] }}
 */
export function splitTags(entryTags, groups) {
  const groupSet = new Set(groups || []);
  const groupTags = [];
  const freeTags = [];
  for (const tag of entryTags || []) {
    if (groupSet.has(tag)) {
      groupTags.push(tag);
    } else {
      freeTags.push(tag);
    }
  }
  return { groupTags, freeTags };
}

/**
 * Return only the free-form tags from an entry, excluding group membership tags.
 */
export function freeTags(entryTags, groups) {
  return splitTags(entryTags, groups).freeTags;
}

/**
 * Return only the group membership tags from an entry.
 */
export function groupTags(entryTags, groups) {
  return splitTags(entryTags, groups).groupTags;
}
