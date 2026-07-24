package vimport

import (
	"path/filepath"
	"time"

	"github.com/eqto/passman/pkg/buttercup"
	"github.com/eqto/passman/pkg/keepass"
	"github.com/eqto/passman/pkg/vault"
)

type ImportCustomField struct {
	ID        string `json:"id"`
	Label     string `json:"label"`
	Type      string `json:"type"`
	Value     string `json:"value"`
}

type ImportGroup struct {
	ID       string  `json:"id"`
	Name     string  `json:"name"`
	ParentID *string `json:"parent_id,omitempty"`
}

type ImportEntry struct {
	ID        string             `json:"id"`
	GroupID   *string            `json:"group_id,omitempty"`
	Title     string             `json:"title"`
	Username  string             `json:"username"`
	Password  string             `json:"password"`
	URL       string             `json:"url"`
	Notes     string             `json:"notes"`
	Tags      []string           `json:"tags"`
	Fields    []ImportCustomField `json:"fields"`
	DeletedAt *time.Time         `json:"deleted_at,omitempty"`
	History   []vault.HistoryItem `json:"history"`
}

type ImportTrash struct {
	Groups  []ImportGroup `json:"groups"`
	Entries []ImportEntry  `json:"entries"`
}

type ImportJSON struct {
	Name      string       `json:"name"`
	UUID      *string      `json:"uuid,omitempty"`
	CreatedAt time.Time    `json:"created_at"`
	UpdatedAt time.Time    `json:"updated_at"`
	Groups    []ImportGroup `json:"groups"`
	Entries   []ImportEntry `json:"entries"`
	Trash     ImportTrash  `json:"trash"`
}

func DefaultVaultName() string {
	return "Imported Vault"
}

func DeriveVaultName(sourceName, inputPath string) string {
	if sourceName != "" {
		return sourceName
	}
	base := filepath.Base(inputPath)
	stem := base
	if ext := filepath.Ext(base); ext != "" {
		stem = base[:len(base)-len(ext)]
	}
	if stem == "" {
		return "Imported Buttercup Vault"
	}
	return stem
}

func mapImportEntryToVaultEntry(e ImportEntry, now time.Time) vault.VaultEntry {
	var fields []vault.CustomField
	for _, f := range e.Fields {
		fields = append(fields, vault.CustomField{
			ID:    f.ID,
			Label: f.Label,
			Type:  f.Type,
			Value: f.Value,
		})
	}

	if e.URL != "" {
		fields = append(fields, vault.CustomField{
			ID:    e.ID + "-cf-url",
			Label: "URL",
			Type:  "text",
			Value: e.URL,
		})
	}
	if e.Notes != "" {
		fields = append(fields, vault.CustomField{
			ID:    e.ID + "-cf-notes",
			Label: "Notes",
			Type:  "note",
			Value: e.Notes,
		})
	}

	return vault.VaultEntry{
		ID:        e.ID,
		Title:     e.Title,
		Username:  e.Username,
		Password:  e.Password,
		Tags:      e.Tags,
		GroupID:   e.GroupID,
		Fields:    fields,
		CreatedAt: now,
		UpdatedAt: now,
		DeletedAt: e.DeletedAt,
		History:   e.History,
	}
}

func mapImportGroupToGroup(g ImportGroup) vault.Group {
	return vault.Group{
		ID:       g.ID,
		Name:     g.Name,
		ParentID: g.ParentID,
	}
}

func BuildPayload(v *vault.VaultFile, imported ImportJSON) {
	now := time.Now().UTC()
	if imported.Name != "" {
		v.Payload.Name = imported.Name
	}
	v.Payload.UUID = imported.UUID
	v.Payload.CreatedAt = imported.CreatedAt
	v.Payload.UpdatedAt = now

	var groups []vault.Group
	for _, g := range imported.Groups {
		mg := mapImportGroupToGroup(g)
		if mg.Name != "" {
			groups = append(groups, mg)
		}
	}
	v.Payload.Groups = groups

	var entries []vault.VaultEntry
	for _, e := range imported.Entries {
		entries = append(entries, mapImportEntryToVaultEntry(e, now))
	}
	v.Payload.Entries = entries

	var trashGroups []vault.Group
	for _, g := range imported.Trash.Groups {
		mg := mapImportGroupToGroup(g)
		if mg.Name != "" {
			trashGroups = append(trashGroups, mg)
		}
	}

	var trashEntries []vault.VaultEntry
	for _, e := range imported.Trash.Entries {
		trashEntries = append(trashEntries, mapImportEntryToVaultEntry(e, now))
	}

	v.Payload.Trash = vault.Trash{
		Groups:  trashGroups,
		Entries: trashEntries,
	}
}

func mapButtercupGroup(g buttercup.ButtercupGroup) ImportGroup {
	return ImportGroup{
		ID:       g.ID,
		Name:     g.Name,
		ParentID: g.ParentID,
	}
}

func mapButtercupEntry(e buttercup.ButtercupEntry) ImportEntry {
	var fields []ImportCustomField
	for _, f := range e.Fields {
		fields = append(fields, ImportCustomField{
			ID:    f.ID,
			Label: f.Label,
			Type:  f.FieldType,
			Value: f.Value,
		})
	}
	return ImportEntry{
		ID:        e.ID,
		GroupID:   e.GroupID,
		Title:     e.Title,
		Username:  e.Username,
		Password:  e.Password,
		Tags:      nil,
		Fields:    fields,
		DeletedAt: e.DeletedAt,
		History:   convertButtercupHistory(e.History),
	}
}

func FromButtercupVault(bv *buttercup.ButtercupVault) ImportJSON {
	now := time.Now().UTC()

	var groups []ImportGroup
	for _, g := range bv.Groups {
		groups = append(groups, mapButtercupGroup(g))
	}
	var entries []ImportEntry
	for _, e := range bv.Entries {
		entries = append(entries, mapButtercupEntry(e))
	}
	var trashGroups []ImportGroup
	for _, g := range bv.Trash.Groups {
		trashGroups = append(trashGroups, mapButtercupGroup(g))
	}
	var trashEntries []ImportEntry
	for _, e := range bv.Trash.Entries {
		trashEntries = append(trashEntries, mapButtercupEntry(e))
	}

	return ImportJSON{
		Name:      bv.Name,
		UUID:      bv.UUID,
		CreatedAt: now,
		UpdatedAt: now,
		Groups:    groups,
		Entries:   entries,
		Trash: ImportTrash{
			Groups:  trashGroups,
			Entries: trashEntries,
		},
	}
}

func mapKeePassGroup(g keepass.KeePassGroup) ImportGroup {
	return ImportGroup{
		ID:       g.ID,
		Name:     g.Name,
		ParentID: g.ParentID,
	}
}

func mapKeePassEntry(e keepass.KeePassEntry) ImportEntry {
	var fields []ImportCustomField
	for _, f := range e.Fields {
		fields = append(fields, ImportCustomField{
			ID:    f.ID,
			Label: f.Label,
			Type:  f.FieldType,
			Value: f.Value,
		})
	}
	return ImportEntry{
		ID:        e.ID,
		GroupID:   e.GroupID,
		Title:     e.Title,
		Username:  e.Username,
		Password:  e.Password,
		URL:       e.URL,
		Notes:     e.Notes,
		Tags:      e.Tags,
		Fields:    fields,
		DeletedAt: e.DeletedAt,
	}
}

func FromKeePassVault(kv *keepass.KeePassVault) ImportJSON {
	now := time.Now().UTC()

	var groups []ImportGroup
	for _, g := range kv.Groups {
		groups = append(groups, mapKeePassGroup(g))
	}
	var entries []ImportEntry
	for _, e := range kv.Entries {
		entries = append(entries, mapKeePassEntry(e))
	}
	var trashGroups []ImportGroup
	for _, g := range kv.Trash.Groups {
		trashGroups = append(trashGroups, mapKeePassGroup(g))
	}
	var trashEntries []ImportEntry
	for _, e := range kv.Trash.Entries {
		trashEntries = append(trashEntries, mapKeePassEntry(e))
	}

	return ImportJSON{
		Name:      kv.Name,
		UUID:      kv.UUID,
		CreatedAt: now,
		UpdatedAt: now,
		Groups:    groups,
		Entries:   entries,
		Trash: ImportTrash{
			Groups:  trashGroups,
			Entries: trashEntries,
		},
	}
}

func convertButtercupHistory(history []buttercup.HistoryItem) []vault.HistoryItem {
	var result []vault.HistoryItem
	for _, h := range history {
		result = append(result, vault.HistoryItem{
			Property:  h.Property,
			Value:     h.Value,
			UpdatedAt: h.UpdatedAt,
		})
	}
	return result
}
