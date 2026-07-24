package vault

import (
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"time"
)

func CollectChildIDs(groups []Group, parentID string) []string {
	var result []string
	for _, g := range groups {
		if g.ParentID != nil && *g.ParentID == parentID {
			result = append(result, g.ID)
			result = append(result, CollectChildIDs(groups, g.ID)...)
		}
	}
	return result
}

func IsDescendant(groups []Group, groupID, potentialParentID string) bool {
	if groupID == potentialParentID {
		return true
	}
	for _, g := range groups {
		if g.ID == groupID {
			if g.ParentID != nil {
				return IsDescendant(groups, *g.ParentID, potentialParentID)
			}
			break
		}
	}
	return false
}

func MoveGroupToParent(payload *VaultPayload, groupID string, newParentID *string) ([]Group, error) {
	found := false
	for _, g := range payload.Groups {
		if g.ID == groupID {
			found = true
			break
		}
	}
	if !found {
		return nil, fmt.Errorf("group does not exist")
	}

	if newParentID != nil {
		parentExists := false
		for _, g := range payload.Groups {
			if g.ID == *newParentID {
				parentExists = true
				break
			}
		}
		if !parentExists {
			return nil, fmt.Errorf("parent group does not exist")
		}
		if IsDescendant(payload.Groups, *newParentID, groupID) {
			return nil, fmt.Errorf("cannot move group into its own descendant")
		}
	}

	for i := range payload.Groups {
		if payload.Groups[i].ID == groupID {
			payload.Groups[i].ParentID = newParentID
			break
		}
	}
	payload.Touch()
	return payload.Groups, nil
}

func MoveEntriesToTrash(payload *VaultPayload, entries []VaultEntry) {
	if len(entries) == 0 {
		return
	}
	now := time.Now().UTC()
	for _, e := range entries {
		entry := e
		entry.GroupID = nil
		entry.UpdatedAt = now
		payload.Trash.Entries = append(payload.Trash.Entries, entry)
	}
}

func MoveGroupToTrash(payload *VaultPayload, group Group, entries []VaultEntry) {
	now := time.Now().UTC()
	groupID := group.ID
	payload.Trash.Groups = append(payload.Trash.Groups, group)
	for _, e := range entries {
		entry := e
		gid := groupID
		entry.GroupID = &gid
		entry.UpdatedAt = now
		payload.Trash.Entries = append(payload.Trash.Entries, entry)
	}
}

func RandomEntryID() string {
	b := make([]byte, 16)
	rand.Read(b)
	return hex.EncodeToString(b)
}

type GroupDeletionResult struct {
	Groups  []Group
	Entries []VaultEntry
	Trash   Trash
}

func DeleteGroupWithChildren(payload *VaultPayload, groupID string) (*GroupDeletionResult, error) {
	found := false
	for _, g := range payload.Groups {
		if g.ID == groupID {
			found = true
			break
		}
	}
	if !found {
		return nil, fmt.Errorf("group does not exist")
	}

	idsToRemove := map[string]bool{groupID: true}
	for _, id := range CollectChildIDs(payload.Groups, groupID) {
		idsToRemove[id] = true
	}

	var group Group
	for _, g := range payload.Groups {
		if g.ID == groupID {
			group = g
			break
		}
	}

	var remainingGroups []Group
	for _, g := range payload.Groups {
		if !idsToRemove[g.ID] {
			remainingGroups = append(remainingGroups, g)
		}
	}
	payload.Groups = remainingGroups

	var entriesToTrash []VaultEntry
	var remainingEntries []VaultEntry
	for _, e := range payload.Entries {
		if e.GroupID != nil && idsToRemove[*e.GroupID] {
			entriesToTrash = append(entriesToTrash, e)
		} else {
			remainingEntries = append(remainingEntries, e)
		}
	}
	payload.Entries = remainingEntries

	MoveGroupToTrash(payload, group, entriesToTrash)
	payload.Touch()

	return &GroupDeletionResult{
		Groups:  payload.Groups,
		Entries: payload.Entries,
		Trash:   payload.Trash,
	}, nil
}

func MergeGroupsInVault(payload *VaultPayload, sourceID, targetID string) ([]Group, []VaultEntry, error) {
	if sourceID == targetID {
		return nil, nil, fmt.Errorf("cannot merge a group into itself")
	}
	sourceExists := false
	targetExists := false
	for _, g := range payload.Groups {
		if g.ID == sourceID {
			sourceExists = true
		}
		if g.ID == targetID {
			targetExists = true
		}
	}
	if !sourceExists {
		return nil, nil, fmt.Errorf("source group does not exist")
	}
	if !targetExists {
		return nil, nil, fmt.Errorf("target group does not exist")
	}

	var remaining []Group
	for _, g := range payload.Groups {
		if g.ID != sourceID {
			remaining = append(remaining, g)
		}
	}
	payload.Groups = remaining

	now := time.Now().UTC()
	for i := range payload.Entries {
		if payload.Entries[i].GroupID != nil && *payload.Entries[i].GroupID == sourceID {
			payload.Entries[i].GroupID = &targetID
			payload.Entries[i].UpdatedAt = now
		}
	}
	payload.Touch()
	return payload.Groups, payload.Entries, nil
}

type PreparedGroupMove struct {
	Entries       []VaultEntry
	Group         *Group
	SourceGroups  []Group
	SourceEntries []VaultEntry
}

func PrepareMoveFromSource(source *VaultPayload, groupID, targetGroupID string) PreparedGroupMove {
	var entriesToMove []VaultEntry
	for _, e := range source.Entries {
		if e.GroupID != nil && *e.GroupID == groupID {
			entriesToMove = append(entriesToMove, e)
		}
	}

	movedIDs := map[string]bool{}
	for _, e := range entriesToMove {
		movedIDs[e.ID] = true
	}

	var sourceGroup *Group
	for _, g := range source.Groups {
		if g.ID == groupID {
			g := g
			sourceGroup = &g
			break
		}
	}
	if sourceGroup == nil {
		fallback := Group{ID: targetGroupID, Name: targetGroupID}
		sourceGroup = &fallback
	}

	var remainingEntries []VaultEntry
	for _, e := range source.Entries {
		if !movedIDs[e.ID] {
			remainingEntries = append(remainingEntries, e)
		}
	}
	source.Entries = remainingEntries

	groupStillUsed := false
	for _, e := range source.Entries {
		if e.GroupID != nil && *e.GroupID == groupID {
			groupStillUsed = true
			break
		}
	}
	if !groupStillUsed {
		var remainingGroups []Group
		for _, g := range source.Groups {
			if g.ID != groupID {
				remainingGroups = append(remainingGroups, g)
			}
		}
		source.Groups = remainingGroups
	}
	source.Touch()

	return PreparedGroupMove{
		Entries:       entriesToMove,
		Group:         sourceGroup,
		SourceGroups:  source.Groups,
		SourceEntries: source.Entries,
	}
}

func ApplyMoveToTarget(target *VaultPayload, targetGroupID string, prepared PreparedGroupMove) ([]Group, []VaultEntry) {
	now := time.Now().UTC()

	targetExists := false
	for _, g := range target.Groups {
		if g.ID == targetGroupID {
			targetExists = true
			break
		}
	}
	if !targetExists && prepared.Group != nil {
		target.Groups = append(target.Groups, *prepared.Group)
	}

	for _, entry := range prepared.Entries {
		e := entry
		e.GroupID = &targetGroupID
		e.UpdatedAt = now
		found := false
		for i := range target.Entries {
			if target.Entries[i].ID == e.ID {
				target.Entries[i] = e
				found = true
				break
			}
		}
		if !found {
			target.Entries = append(target.Entries, e)
		}
	}
	target.Touch()
	return target.Groups, target.Entries
}

type PreparedGroupCopy struct {
	Entries []VaultEntry
	Group   *Group
}

func PrepareCopyFromSource(source *VaultPayload, groupID, targetGroupID string) PreparedGroupCopy {
	var entriesToCopy []VaultEntry
	for _, e := range source.Entries {
		if e.GroupID != nil && *e.GroupID == groupID {
			entriesToCopy = append(entriesToCopy, e)
		}
	}

	var sourceGroup *Group
	for _, g := range source.Groups {
		if g.ID == groupID {
			g := g
			sourceGroup = &g
			break
		}
	}
	if sourceGroup == nil {
		fallback := Group{ID: targetGroupID, Name: targetGroupID}
		sourceGroup = &fallback
	}

	return PreparedGroupCopy{
		Entries: entriesToCopy,
		Group:   sourceGroup,
	}
}

func ApplyCopyToTarget(target *VaultPayload, targetGroupID string, prepared PreparedGroupCopy) ([]Group, []VaultEntry) {
	now := time.Now().UTC()

	targetExists := false
	for _, g := range target.Groups {
		if g.ID == targetGroupID {
			targetExists = true
			break
		}
	}
	if !targetExists && prepared.Group != nil {
		target.Groups = append(target.Groups, *prepared.Group)
	}

	for _, entry := range prepared.Entries {
		copy := entry
		copy.ID = RandomEntryID()
		copy.GroupID = &targetGroupID
		copy.CreatedAt = now
		copy.UpdatedAt = now
		target.Entries = append(target.Entries, copy)
	}
	target.Touch()
	return target.Groups, target.Entries
}
