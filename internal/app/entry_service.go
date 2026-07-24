package app

import (
	"fmt"
	"time"

	"github.com/eqto/passman/internal/state"
	"github.com/eqto/passman/pkg/vault"
)

type EntryService struct {
	state *state.AppState
}

func NewEntryService(s *state.AppState) *EntryService {
	return &EntryService{state: s}
}

type EntryMutationResult struct {
	Entry vault.VaultEntry `json:"entry"`
}

type EntryDeletionResult struct {
	Entries []vault.VaultEntry `json:"entries"`
	Trash   vault.Trash        `json:"trash"`
}

type TrashMutationResult struct {
	GroupID   *string            `json:"group_id"`
	GroupName string             `json:"group_name"`
	Groups    []vault.Group      `json:"groups"`
	Entries   []vault.VaultEntry `json:"entries"`
	Trash     vault.Trash        `json:"trash"`
}

func (s *EntryService) ListEntries(path string) ([]vault.VaultEntry, error) {
	var entries []vault.VaultEntry
	err := s.state.WithOpenVault(path, func(ov *state.OpenVault) error {
		entries = ov.Vault.Payload.Entries
		return nil
	})
	return entries, err
}

func (s *EntryService) AddEntry(path string, entry vault.VaultEntry) (*EntryMutationResult, error) {
	var result *EntryMutationResult
	err := s.state.WithOpenVaultSave(path, func(ov *state.OpenVault) error {
		for _, e := range ov.Vault.Payload.Entries {
			if e.ID == entry.ID {
				return fmt.Errorf("an entry with this id already exists")
			}
		}
		ov.Vault.Payload.Entries = append(ov.Vault.Payload.Entries, entry)
		ov.Vault.Payload.Touch()
		result = &EntryMutationResult{Entry: entry}
		return nil
	})
	return result, err
}

func (s *EntryService) UpdateEntry(path string, entry vault.VaultEntry) (*EntryMutationResult, error) {
	var result *EntryMutationResult
	err := s.state.WithOpenVaultSave(path, func(ov *state.OpenVault) error {
		found := false
		for i := range ov.Vault.Payload.Entries {
			if ov.Vault.Payload.Entries[i].ID == entry.ID {
				ov.Vault.Payload.Entries[i] = entry
				found = true
				break
			}
		}
		if !found {
			return fmt.Errorf("entry not found")
		}
		ov.Vault.Payload.Touch()
		result = &EntryMutationResult{Entry: entry}
		return nil
	})
	return result, err
}

func (s *EntryService) DeleteEntry(path, id string) (*EntryDeletionResult, error) {
	var result *EntryDeletionResult
	err := s.state.WithOpenVaultSave(path, func(ov *state.OpenVault) error {
		// First try to delete from trash
		for i, e := range ov.Vault.Payload.Trash.Entries {
			if e.ID == id {
				ov.Vault.Payload.Trash.Entries = append(ov.Vault.Payload.Trash.Entries[:i], ov.Vault.Payload.Trash.Entries[i+1:]...)
				ov.Vault.Payload.Touch()
				result = &EntryDeletionResult{
					Entries: ov.Vault.Payload.Entries,
					Trash:   ov.Vault.Payload.Trash,
				}
				return nil
			}
		}

		var entryToTrash *vault.VaultEntry
		var remaining []vault.VaultEntry
		for _, e := range ov.Vault.Payload.Entries {
			if e.ID == id {
				eCopy := e
				entryToTrash = &eCopy
			} else {
				remaining = append(remaining, e)
			}
		}
		ov.Vault.Payload.Entries = remaining
		if entryToTrash != nil {
			vault.MoveEntriesToTrash(&ov.Vault.Payload, []vault.VaultEntry{*entryToTrash})
		}
		ov.Vault.Payload.Touch()
		result = &EntryDeletionResult{
			Entries: ov.Vault.Payload.Entries,
			Trash:   ov.Vault.Payload.Trash,
		}
		return nil
	})
	return result, err
}

func (s *EntryService) RestoreTrashEntry(path, id string) (*TrashMutationResult, error) {
	var result *TrashMutationResult
	err := s.state.WithOpenVaultSave(path, func(ov *state.OpenVault) error {
		payload := &ov.Vault.Payload

		idx := -1
		for i, e := range payload.Trash.Entries {
			if e.ID == id {
				idx = i
				break
			}
		}
		if idx == -1 {
			return fmt.Errorf("entry not found in trash")
		}

		entry := payload.Trash.Entries[idx]
		payload.Trash.Entries = append(payload.Trash.Entries[:idx], payload.Trash.Entries[idx+1:]...)

		groupID := entry.GroupID
		groupName := ""
		if groupID != nil {
			for _, g := range payload.Trash.Groups {
				if g.ID == *groupID {
					groupName = g.Name
					break
				}
			}
		}
		entry.UpdatedAt = time.Now().UTC()

		if groupID != nil {
			restoreGroupIfMissing(payload, *groupID, groupName)
		}

		exists := false
		for _, e := range payload.Entries {
			if e.ID == entry.ID {
				exists = true
				break
			}
		}
		if !exists {
			payload.Entries = append(payload.Entries, entry)
		}

		payload.Touch()
		result = &TrashMutationResult{
			GroupID:   groupID,
			GroupName: groupName,
			Groups:    payload.Groups,
			Entries:   payload.Entries,
			Trash:     payload.Trash,
		}
		return nil
	})
	return result, err
}

func (s *EntryService) DeleteTrashEntry(path, id string) (vault.Trash, error) {
	var trash vault.Trash
	err := s.state.WithOpenVaultSave(path, func(ov *state.OpenVault) error {
		var remaining []vault.VaultEntry
		for _, e := range ov.Vault.Payload.Trash.Entries {
			if e.ID != id {
				remaining = append(remaining, e)
			}
		}
		ov.Vault.Payload.Trash.Entries = remaining
		ov.Vault.Payload.Touch()
		trash = ov.Vault.Payload.Trash
		return nil
	})
	return trash, err
}

func (s *EntryService) ListTrash(path string) (vault.Trash, error) {
	var trash vault.Trash
	err := s.state.WithOpenVault(path, func(ov *state.OpenVault) error {
		trash = ov.Vault.Payload.Trash
		return nil
	})
	return trash, err
}

func restoreGroupIfMissing(payload *vault.VaultPayload, gid, groupName string) {
	for _, g := range payload.Groups {
		if g.ID == gid {
			return
		}
	}
	for _, tg := range payload.Trash.Groups {
		if tg.ID == gid {
			payload.Groups = append(payload.Groups, tg)
			return
		}
	}
	payload.Groups = append(payload.Groups, vault.Group{
		ID:   gid,
		Name: groupName,
	})
}
