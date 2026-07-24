package buttercup

import (
	"strconv"
	"strings"
	"time"
)

func isTrashGroup(group *RawGroup) bool {
	if v, ok := group.A["bc_group_role"]; ok {
		return strings.EqualFold(v.Value, "trash")
	}
	return false
}

func datetimeFromMillis(ts uint64) time.Time {
	return time.UnixMilli(int64(ts)).UTC()
}

func getFieldType(attributes map[string]RawValue, property string) string {
	key := "BC_ENTRY_FIELD_TYPE:" + property
	if v, ok := attributes[key]; ok && v.Value != "" {
		return v.Value
	}
	return "text"
}

func getProperty(properties map[string]RawValue, name string) string {
	if v, ok := properties[name]; ok {
		return v.Value
	}
	return ""
}

func IdentifyTrashGroups(raw *RawVault) (*string, map[string]bool) {
	var trashGroupID *string
	for _, g := range raw.G {
		if isTrashGroup(&g) {
			id := g.ID
			trashGroupID = &id
			break
		}
	}

	trashGroupIDs := map[string]bool{}
	if trashGroupID != nil {
		trashGroupIDs[*trashGroupID] = true
		collectDescendants(raw.G, *trashGroupID, trashGroupIDs)
	}
	return trashGroupID, trashGroupIDs
}

func collectDescendants(groups []RawGroup, parentID string, ids map[string]bool) {
	for _, g := range groups {
		if g.G == parentID && !ids[g.ID] {
			ids[g.ID] = true
			collectDescendants(groups, g.ID, ids)
		}
	}
}

var standardProperties = map[string]bool{
	"title":    true,
	"username": true,
	"password": true,
	"URL":      true,
	"notes":    true,
}

func extractCustomFields(entry *RawEntry) []ButtercupCustomField {
	var fields []ButtercupCustomField
	for prop, rv := range entry.P {
		if standardProperties[prop] || rv.Value == "" {
			continue
		}
		if rv.Deleted != nil {
			continue
		}
		fieldType := getFieldType(entry.A, prop)
		fields = append(fields, ButtercupCustomField{
			ID:        entry.ID + "-cf-" + strconv.Itoa(len(fields)),
			Label:     prop,
			FieldType: fieldType,
			Value:     rv.Value,
		})
	}
	return fields
}

func extractHistory(entry *RawEntry) []HistoryItem {
	var history []HistoryItem
	for prop, rv := range entry.P {
		for _, hist := range rv.History {
			history = append(history, HistoryItem{
				Property:  prop,
				Value:     hist.Value,
				UpdatedAt: datetimeFromMillis(hist.Updated),
			})
		}
	}
	return history
}

func BuildGroups(rawGroups []RawGroup, trashGroupID *string, trashGroupIDs map[string]bool) ([]ButtercupGroup, []ButtercupGroup) {
	var groups, trashGroups []ButtercupGroup
	seenGroupIDs := map[string]bool{}

	for _, group := range rawGroups {
		if group.T == "" || seenGroupIDs[group.ID] {
			continue
		}
		seenGroupIDs[group.ID] = true

		var parentID *string
		if group.G != "" {
			parentID = &group.G
		}

		bcGroup := ButtercupGroup{
			ID:       group.ID,
			Name:     group.T,
			ParentID: parentID,
		}

		if trashGroupID != nil && *trashGroupID == bcGroup.ID {
			continue
		}

		if trashGroupIDs[bcGroup.ID] {
			trashGroup := bcGroup
			if trashGroupID != nil && trashGroup.ParentID != nil && *trashGroup.ParentID == *trashGroupID {
				trashGroup.ParentID = nil
			}
			trashGroups = append(trashGroups, trashGroup)
		} else {
			groups = append(groups, bcGroup)
		}
	}
	return groups, trashGroups
}

func BuildEntries(rawEntries []RawEntry, trashGroupIDs map[string]bool, trashGroupID *string) ([]ButtercupEntry, []ButtercupEntry) {
	var entries, trashEntries []ButtercupEntry

	for _, entry := range rawEntries {
		var groupID *string
		if entry.G != "" {
			gid := entry.G
			groupID = &gid
		}
		id := entry.ID

		fields := extractCustomFields(&entry)

		url := getProperty(entry.P, "URL")
		if url != "" {
			fields = append(fields, ButtercupCustomField{
				ID:        id + "-cf-url",
				Label:     "URL",
				FieldType: "text",
				Value:     url,
			})
		}
		notes := getProperty(entry.P, "notes")
		if notes != "" {
			fields = append(fields, ButtercupCustomField{
				ID:        id + "-cf-notes",
				Label:     "Notes",
				FieldType: "note",
				Value:     notes,
			})
		}

		var deletedAt *time.Time
		if entry.Deleted != nil {
			t := datetimeFromMillis(*entry.Deleted)
			deletedAt = &t
		}

		bcEntry := ButtercupEntry{
			ID:        id,
			GroupID:   groupID,
			Title:     getProperty(entry.P, "title"),
			Username:  getProperty(entry.P, "username"),
			Password:  getProperty(entry.P, "password"),
			Fields:    fields,
			DeletedAt: deletedAt,
			History:   extractHistory(&entry),
		}

		isInTrashGroup := groupID != nil && trashGroupIDs[*groupID]
		isTrash := entry.Deleted != nil || isInTrashGroup

		if isTrash {
			trashEntry := bcEntry
			if trashGroupID != nil && trashEntry.GroupID != nil && *trashEntry.GroupID == *trashGroupID {
				trashEntry.GroupID = nil
			}
			trashEntries = append(trashEntries, trashEntry)
		} else {
			entries = append(entries, bcEntry)
		}
	}
	return entries, trashEntries
}
