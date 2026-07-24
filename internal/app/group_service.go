package app

import (
	"fmt"
	"sort"
	"strings"

	"github.com/eqto/passman/internal/state"
	"github.com/eqto/passman/pkg/vault"
)

type GroupService struct {
	state *state.AppState
}

func NewGroupService(s *state.AppState) *GroupService {
	return &GroupService{state: s}
}

func (s *GroupService) ListGroups(path string) ([]vault.Group, error) {
	var groups []vault.Group
	err := s.state.WithOpenVault(path, func(ov *state.OpenVault) error {
		groups = ov.Vault.Payload.Groups
		return nil
	})
	return groups, err
}

func (s *GroupService) AddGroup(path string, group vault.Group) ([]vault.Group, error) {
	var groups []vault.Group
	err := s.state.WithOpenVaultSave(path, func(ov *state.OpenVault) error {
		name := strings.TrimSpace(group.Name)
		if name == "" {
			return fmt.Errorf("group name cannot be empty")
		}
		if group.ID == "" {
			return fmt.Errorf("group id cannot be empty")
		}
		for _, g := range ov.Vault.Payload.Groups {
			if g.ID == group.ID {
				return nil
			}
		}
		ov.Vault.Payload.Groups = append(ov.Vault.Payload.Groups, vault.Group{
			ID:       group.ID,
			Name:     name,
			ParentID: group.ParentID,
		})
		ov.Vault.Payload.Touch()
		groups = ov.Vault.Payload.Groups
		return nil
	})
	return groups, err
}

type GroupDeletionResult struct {
	Groups  []vault.Group  `json:"groups"`
	Entries []vault.VaultEntry `json:"entries"`
	Trash   vault.Trash    `json:"trash"`
}

func (s *GroupService) DeleteGroup(path, groupID string) (*GroupDeletionResult, error) {
	var result *GroupDeletionResult
	err := s.state.WithOpenVaultSave(path, func(ov *state.OpenVault) error {
		r, err := vault.DeleteGroupWithChildren(&ov.Vault.Payload, groupID)
		if err != nil {
			return err
		}
		result = &GroupDeletionResult{
			Groups:  r.Groups,
			Entries: r.Entries,
			Trash:   r.Trash,
		}
		return nil
	})
	return result, err
}

func (s *GroupService) ReorderGroups(path string, groups []vault.Group) ([]vault.Group, error) {
	var result []vault.Group
	err := s.state.WithOpenVaultSave(path, func(ov *state.OpenVault) error {
		currentIDs := make([]string, len(ov.Vault.Payload.Groups))
		for i, g := range ov.Vault.Payload.Groups {
			currentIDs[i] = g.ID
		}
		newIDs := make([]string, len(groups))
		for i, g := range groups {
			newIDs[i] = g.ID
		}
		if err := validateReorder(currentIDs, newIDs); err != nil {
			return err
		}
		ov.Vault.Payload.Groups = groups
		ov.Vault.Payload.Touch()
		result = ov.Vault.Payload.Groups
		return nil
	})
	return result, err
}

func (s *GroupService) MergeGroups(path, sourceID, targetID string) ([]vault.Group, []vault.VaultEntry, error) {
	var groups []vault.Group
	var entries []vault.VaultEntry
	err := s.state.WithOpenVaultSave(path, func(ov *state.OpenVault) error {
		g, e, err := vault.MergeGroupsInVault(&ov.Vault.Payload, sourceID, targetID)
		if err != nil {
			return err
		}
		groups = g
		entries = e
		return nil
	})
	return groups, entries, err
}

type MoveGroupToVaultResult struct {
	SourceGroups  []vault.Group     `json:"source_groups"`
	SourceEntries []vault.VaultEntry `json:"source_entries"`
	TargetGroups  []vault.Group     `json:"target_groups"`
	TargetEntries []vault.VaultEntry `json:"target_entries"`
}

func (s *GroupService) MoveGroupToVault(sourcePath, targetPath, groupID, targetGroupID string) (*MoveGroupToVaultResult, error) {
	if sourcePath == targetPath {
		return nil, fmt.Errorf("source and target vault must be different")
	}
	ov, ok := s.state.GetVault(sourcePath)
	if !ok {
		return nil, fmt.Errorf("source vault is not open")
	}
	prepared := vault.PrepareMoveFromSource(&ov.Vault.Payload, groupID, targetGroupID)

	targetOV, ok := s.state.GetVault(targetPath)
	if !ok {
		return nil, fmt.Errorf("target vault is not open")
	}
	targetGroups, targetEntries := vault.ApplyMoveToTarget(&targetOV.Vault.Payload, targetGroupID, prepared)

	sourceGroups := ov.Vault.Payload.Groups
	sourceEntries := ov.Vault.Payload.Entries

	s.state.ScheduleSave(sourcePath)
	s.state.ScheduleSave(targetPath)

	return &MoveGroupToVaultResult{
		SourceGroups:  sourceGroups,
		SourceEntries: sourceEntries,
		TargetGroups:  targetGroups,
		TargetEntries: targetEntries,
	}, nil
}

func (s *GroupService) CopyGroupToVault(sourcePath, targetPath, groupID, targetGroupID string) ([]vault.Group, []vault.VaultEntry, error) {
	if sourcePath == targetPath {
		return nil, nil, fmt.Errorf("source and target vault must be different")
	}
	sourceOV, ok := s.state.GetVault(sourcePath)
	if !ok {
		return nil, nil, fmt.Errorf("source vault is not open")
	}
	prepared := vault.PrepareCopyFromSource(&sourceOV.Vault.Payload, groupID, targetGroupID)

	targetOV, ok := s.state.GetVault(targetPath)
	if !ok {
		return nil, nil, fmt.Errorf("target vault is not open")
	}
	targetGroups, targetEntries := vault.ApplyCopyToTarget(&targetOV.Vault.Payload, targetGroupID, prepared)

	s.state.ScheduleSave(targetPath)
	return targetGroups, targetEntries, nil
}

func (s *GroupService) MoveGroupToParent(path, groupID string, newParentID *string) ([]vault.Group, error) {
	var groups []vault.Group
	err := s.state.WithOpenVaultSave(path, func(ov *state.OpenVault) error {
		g, err := vault.MoveGroupToParent(&ov.Vault.Payload, groupID, newParentID)
		if err != nil {
			return err
		}
		groups = g
		return nil
	})
	return groups, err
}

func (s *GroupService) AddTag(path, tag string) ([]string, error) {
	var tags []string
	err := s.state.WithOpenVaultSave(path, func(ov *state.OpenVault) error {
		trimmed := strings.TrimSpace(tag)
		if trimmed == "" {
			return fmt.Errorf("tag name cannot be empty")
		}
		for _, t := range ov.Vault.Payload.Tags {
			if t == trimmed {
				goto sort
			}
		}
		ov.Vault.Payload.Tags = append(ov.Vault.Payload.Tags, trimmed)
	sort:
		sort.Strings(ov.Vault.Payload.Tags)
		ov.Vault.Payload.Touch()
		tags = ov.Vault.Payload.Tags
		return nil
	})
	return tags, err
}
