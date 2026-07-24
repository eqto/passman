package keepass

import (
	"fmt"
	"os"
	"strings"
	"time"

	keepasslib "github.com/tobischo/gokeepasslib/v3"
	w "github.com/tobischo/gokeepasslib/v3/wrappers"
)

type KeePassError struct {
	Msg string
}

func (e *KeePassError) Error() string { return e.Msg }

type KeePassVault struct {
	Name    string
	UUID    *string
	Groups  []KeePassGroup
	Entries []KeePassEntry
	Trash   KeePassTrash
}

type KeePassTrash struct {
	Groups  []KeePassGroup
	Entries []KeePassEntry
}

type KeePassGroup struct {
	ID       string
	Name     string
	ParentID *string
}

type KeePassCustomField struct {
	ID        string
	Label     string
	FieldType string
	Value     string
}

type KeePassEntry struct {
	ID        string
	GroupID   *string
	Title     string
	Username  string
	Password  string
	URL       string
	Notes     string
	Tags      []string
	Fields    []KeePassCustomField
	DeletedAt *time.Time
}

var standardFields = map[string]bool{
	"Title":    true,
	"UserName": true,
	"Password": true,
	"URL":      true,
	"Notes":    true,
	"otp":      true,
}

func uuidToString(u keepasslib.UUID) string {
	text, err := u.MarshalText()
	if err != nil {
		return ""
	}
	return string(text)
}

func DecryptKeePassFile(path, password string) (*KeePassVault, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	db := keepasslib.NewDatabase()
	db.Credentials = keepasslib.NewPasswordCredentials(password)

	decoder := keepasslib.NewDecoder(file)
	if err := decoder.Decode(db); err != nil {
		return nil, &KeePassError{Msg: err.Error()}
	}

	if err := db.UnlockProtectedEntries(); err != nil {
		return nil, &KeePassError{Msg: err.Error()}
	}

	name := "Imported KeePass"
	if db.Content != nil && db.Content.Meta != nil && db.Content.Meta.DatabaseName != "" {
		name = db.Content.Meta.DatabaseName
	}

	root := db.Content.Root

	var rootUUIDPtr *string
	var recycleBinIDs map[string]bool

	if len(root.Groups) > 0 {
		rootGroup := &root.Groups[0]
		rootUUID := uuidToString(rootGroup.UUID)
		if rootUUID != "" {
			rootUUIDPtr = &rootUUID
		}

		recycleBinIDs = map[string]bool{}
		if db.Content.Meta != nil && db.Content.Meta.RecycleBinEnabled.Bool {
			rbID := uuidToString(db.Content.Meta.RecycleBinUUID)
			if rbID != "" {
				collectAllGroupIDs(rootGroup, recycleBinIDs, rbID)
			}
		}
	}

	var groups []KeePassGroup
	var entries []KeePassEntry
	var trashGroups []KeePassGroup
	var trashEntries []KeePassEntry

	if len(root.Groups) > 0 {
		rootGroup := &root.Groups[0]
		for i := range rootGroup.Groups {
			subgroup := &rootGroup.Groups[i]
			gid := uuidToString(subgroup.UUID)
			isTrash := recycleBinIDs[gid]

			if isTrash {
				collectTrash(subgroup, nil, recycleBinIDs, &trashGroups, &trashEntries)
			} else {
				collectNormal(subgroup, nil, recycleBinIDs, &groups, &entries)
			}
		}

		for i := range rootGroup.Entries {
			entry := &rootGroup.Entries[i]
			eid := uuidToString(entry.UUID)
			entries = append(entries, mapEntry(entry, nil, eid))
		}
	}

	return &KeePassVault{
		Name:    name,
		UUID:    rootUUIDPtr,
		Groups:  groups,
		Entries: entries,
		Trash: KeePassTrash{
			Groups:  trashGroups,
			Entries: trashEntries,
		},
	}, nil
}

func collectAllGroupIDs(group *keepasslib.Group, ids map[string]bool, targetUUID string) {
	gid := uuidToString(group.UUID)
	if gid == targetUUID || targetUUID == "" {
		ids[gid] = true
		for i := range group.Groups {
			collectAllGroupIDs(&group.Groups[i], ids, "")
		}
		return
	}
	for i := range group.Groups {
		collectAllGroupIDs(&group.Groups[i], ids, targetUUID)
	}
}

func collectNormal(group *keepasslib.Group, parentID *string, recycleBinIDs map[string]bool, groups *[]KeePassGroup, entries *[]KeePassEntry) {
	gid := uuidToString(group.UUID)
	*groups = append(*groups, KeePassGroup{
		ID:       gid,
		Name:     group.Name,
		ParentID: parentID,
	})

	for i := range group.Entries {
		entry := &group.Entries[i]
		eid := uuidToString(entry.UUID)
		*entries = append(*entries, mapEntry(entry, &gid, eid))
	}

	for i := range group.Groups {
		subgid := uuidToString(group.Groups[i].UUID)
		if recycleBinIDs[subgid] {
			continue
		}
		collectNormal(&group.Groups[i], &gid, recycleBinIDs, groups, entries)
	}
}

func collectTrash(group *keepasslib.Group, parentID *string, recycleBinIDs map[string]bool, groups *[]KeePassGroup, entries *[]KeePassEntry) {
	gid := uuidToString(group.UUID)
	*groups = append(*groups, KeePassGroup{
		ID:       gid,
		Name:     group.Name,
		ParentID: parentID,
	})

	for i := range group.Entries {
		entry := &group.Entries[i]
		eid := uuidToString(entry.UUID)
		*entries = append(*entries, mapEntry(entry, &gid, eid))
	}

	for i := range group.Groups {
		collectTrash(&group.Groups[i], &gid, recycleBinIDs, groups, entries)
	}
}

func mapEntry(entry *keepasslib.Entry, groupID *string, eid string) KeePassEntry {
	title := entry.GetTitle()
	username := entry.GetContent("UserName")
	password := entry.GetPassword()
	url := entry.GetContent("URL")
	notes := entry.GetContent("Notes")

	var fields []KeePassCustomField
	for _, val := range entry.Values {
		if standardFields[val.Key] {
			continue
		}
		value := val.Value.Content
		if value == "" {
			continue
		}
		fieldType := "text"
		if val.Key == "otp" {
			fieldType = "otp"
		}
		fields = append(fields, KeePassCustomField{
			ID:        fmt.Sprintf("%s-cf-%d", eid, len(fields)),
			Label:     val.Key,
			FieldType: fieldType,
			Value:     value,
		})
	}

	var tags []string
	if entry.Tags != "" {
		for _, t := range strings.Split(entry.Tags, ";") {
			t = strings.TrimSpace(t)
			if t != "" {
				tags = append(tags, t)
			}
		}
	}

	return KeePassEntry{
		ID:       eid,
		GroupID:  groupID,
		Title:    title,
		Username: username,
		Password: password,
		URL:      url,
		Notes:    notes,
		Tags:     tags,
		Fields:   fields,
	}
}

func init() {
	_ = w.BoolWrapper{}
}
