package app

import (
	"fmt"
	"strings"

	"github.com/eqto/passman/internal/config"
	"github.com/eqto/passman/internal/state"
	"github.com/eqto/passman/internal/vimport"
	"github.com/eqto/passman/pkg/buttercup"
	"github.com/eqto/passman/pkg/crypto"
	"github.com/eqto/passman/pkg/keepass"
	"github.com/eqto/passman/pkg/vault"
)

type VaultService struct {
	state *state.AppState
}

func NewVaultService(s *state.AppState) *VaultService {
	return &VaultService{state: s}
}

func (s *VaultService) ListVaults() (*config.AppConfig, error) {
	return config.LoadConfig()
}

func (s *VaultService) CreateVault(id, name, path, password string, securityLevel *string) (*config.VaultConfig, error) {
	if vault.VaultExists(path) {
		return nil, fmt.Errorf("vault file already exists")
	}
	level, err := parseSecurityLevel(securityLevel)
	if err != nil {
		return nil, err
	}
	v, vaultKey, err := vault.CreateVaultFileWithLevel(path, name, password, level)
	if err != nil {
		return nil, err
	}
	if err := config.AddVault(id, name, path); err != nil {
		return nil, err
	}
	s.state.InsertVault(path, v, vaultKey)
	return &config.VaultConfig{ID: id, Name: name, Path: path}, nil
}

func (s *VaultService) OpenVault(path, password string) (*vault.VaultFileDTO, error) {
	dto, _, err := s.openVaultInner(path, password)
	return dto, err
}

func (s *VaultService) RegisterAndOpenVault(id, path, password string) (*vault.VaultFileDTO, error) {
	dto, _, err := s.openVaultInner(path, password)
	if err != nil {
		return nil, err
	}
	if err := config.AddVault(id, dto.Name, path); err != nil {
		return nil, err
	}
	return dto, nil
}

func (s *VaultService) openVaultInner(path, password string) (*vault.VaultFileDTO, [crypto.KeySize]byte, error) {
	v, vaultKey, err := vault.OpenVaultFileWithKey(path, password)
	if err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}
	dto := vault.VaultToDTO(v)
	s.state.InsertVault(path, v, vaultKey)
	return &dto, vaultKey, nil
}

func (s *VaultService) CloseVault(path string) error {
	s.state.RemoveVault(path)
	return nil
}

func (s *VaultService) DeleteVault(id, path string) error {
	if err := config.RemoveVault(id); err != nil {
		return err
	}
	s.state.RemoveVault(path)
	return nil
}

func (s *VaultService) RenameVault(id, name string) (*config.VaultConfig, error) {
	cfg, err := config.LoadConfig()
	if err != nil {
		return nil, err
	}
	var path string
	for _, v := range cfg.Vaults {
		if v.ID == id {
			path = v.Path
			break
		}
	}
	if path == "" {
		return nil, fmt.Errorf("vault not found")
	}
	if !s.state.IsOpen(path) {
		return nil, fmt.Errorf("vault must be unlocked to rename")
	}

	var updated config.VaultConfig
	for i, v := range cfg.Vaults {
		if v.ID == id {
			cfg.Vaults[i].Name = name
			updated = cfg.Vaults[i]
			break
		}
	}
	if err := config.SaveConfig(cfg); err != nil {
		return nil, err
	}

	_ = s.state.WithOpenVault(path, func(ov *state.OpenVault) error {
		ov.Vault.Payload.Name = name
		ov.Vault.Payload.Touch()
		return nil
	})
	s.state.ScheduleSave(path)
	return &updated, nil
}

func (s *VaultService) ReorderVaults(ids []string) ([]config.VaultConfig, error) {
	cfg, err := config.LoadConfig()
	if err != nil {
		return nil, err
	}
	currentIDs := make([]string, len(cfg.Vaults))
	for i, v := range cfg.Vaults {
		currentIDs[i] = v.ID
	}
	if err := validateReorder(currentIDs, ids); err != nil {
		return nil, err
	}
	var ordered []config.VaultConfig
	for _, id := range ids {
		for _, v := range cfg.Vaults {
			if v.ID == id {
				ordered = append(ordered, v)
				break
			}
		}
	}
	cfg.Vaults = ordered
	if err := config.SaveConfig(cfg); err != nil {
		return nil, err
	}
	return cfg.Vaults, nil
}

func (s *VaultService) ConvertButtercupVault(bcupPath, password, outputPath, id string, securityLevel *string) (*vault.VaultFileDTO, error) {
	bcup, err := buttercup.DecryptButtercupFile(bcupPath, password)
	if err != nil {
		return nil, err
	}
	impJSON := vimport.FromButtercupVault(bcup)
	vaultName := vimport.DeriveVaultName(impJSON.Name, bcupPath)
	level, err := parseSecurityLevel(securityLevel)
	if err != nil {
		return nil, err
	}
	v, vaultKey, err := vault.CreateVaultFileWithLevel(outputPath, vaultName, password, level)
	if err != nil {
		return nil, err
	}
	vimport.BuildPayload(v, impJSON)
	if err := vault.SaveVaultFile(v, password); err != nil {
		return nil, err
	}
	if err := config.AddVault(id, vaultName, outputPath); err != nil {
		return nil, err
	}
	dto := vault.VaultToDTO(v)
	s.state.InsertVault(outputPath, v, vaultKey)
	return &dto, nil
}

func (s *VaultService) ConvertKeepassVault(kdbxPath, password, outputPath, id string, securityLevel *string) (*vault.VaultFileDTO, error) {
	kdbx, err := keepass.DecryptKeePassFile(kdbxPath, password)
	if err != nil {
		return nil, err
	}
	impJSON := vimport.FromKeePassVault(kdbx)
	vaultName := vimport.DeriveVaultName(impJSON.Name, kdbxPath)
	level, err := parseSecurityLevel(securityLevel)
	if err != nil {
		return nil, err
	}
	v, vaultKey, err := vault.CreateVaultFileWithLevel(outputPath, vaultName, password, level)
	if err != nil {
		return nil, err
	}
	vimport.BuildPayload(v, impJSON)
	if err := vault.SaveVaultFile(v, password); err != nil {
		return nil, err
	}
	if err := config.AddVault(id, vaultName, outputPath); err != nil {
		return nil, err
	}
	dto := vault.VaultToDTO(v)
	s.state.InsertVault(outputPath, v, vaultKey)
	return &dto, nil
}

func (s *VaultService) ChangeSecurityLevel(path, password, newLevel string) error {
	level, err := parseSecurityLevel(&newLevel)
	if err != nil {
		return err
	}
	ov, ok := s.state.GetVault(path)
	if !ok {
		return fmt.Errorf("vault is not open")
	}
	newHeader, newVaultKey, err := vault.ChangeKdfParams(ov.Vault, password, level)
	if err != nil {
		return err
	}
	_ = s.state.WithOpenVault(path, func(ov *state.OpenVault) error {
		ov.Vault.Header = *newHeader
		ov.Key = make([]byte, len(newVaultKey))
		copy(ov.Key, newVaultKey[:])
		return nil
	})
	return nil
}

func parseSecurityLevel(level *string) (crypto.SecurityLevel, error) {
	if level == nil || *level == "" {
		return crypto.SecurityLevelMedium, nil
	}
	return crypto.ParseSecurityLevel(strings.ToLower(*level))
}

func validateReorder(current, reordered []string) error {
	if len(current) != len(reordered) {
		return fmt.Errorf("invalid list")
	}
	currentSet := map[string]bool{}
	for _, id := range current {
		currentSet[id] = true
	}
	newSet := map[string]bool{}
	for _, id := range reordered {
		newSet[id] = true
	}
	if len(newSet) != len(currentSet) {
		return fmt.Errorf("invalid list")
	}
	for id := range currentSet {
		if !newSet[id] {
			return fmt.Errorf("invalid list")
		}
	}
	return nil
}
