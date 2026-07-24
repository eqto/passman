package config

import (
	"encoding/json"
	"os"
	"path/filepath"
)

type VaultConfig struct {
	ID   string `json:"id"`
	Name string `json:"name"`
	Path string `json:"path"`
}

type AppConfig struct {
	Vaults []VaultConfig `json:"vaults"`
}

func ConfigDir() (string, error) {
	dir, err := os.UserConfigDir()
	if err != nil {
		return "", err
	}
	return filepath.Join(dir, "passman"), nil
}

func ConfigPath() (string, error) {
	dir, err := ConfigDir()
	if err != nil {
		return "", err
	}
	if _, err := os.Stat(dir); os.IsNotExist(err) {
		if err := os.MkdirAll(dir, 0700); err != nil {
			return "", err
		}
	}
	return filepath.Join(dir, "vaults.json"), nil
}

func LoadConfig() (*AppConfig, error) {
	path, err := ConfigPath()
	if err != nil {
		return nil, err
	}
	data, err := os.ReadFile(path)
	if os.IsNotExist(err) {
		return &AppConfig{}, nil
	}
	if err != nil {
		return nil, err
	}
	var cfg AppConfig
	if err := json.Unmarshal(data, &cfg); err != nil {
		return nil, err
	}
	return &cfg, nil
}

func SaveConfig(cfg *AppConfig) error {
	path, err := ConfigPath()
	if err != nil {
		return err
	}
	data, err := json.MarshalIndent(cfg, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(path, data, 0600)
}

func AddVault(id, name, path string) error {
	cfg, err := LoadConfig()
	if err != nil {
		return err
	}
	cfg.Vaults = append(cfg.Vaults, VaultConfig{ID: id, Name: name, Path: path})
	return SaveConfig(cfg)
}

func RemoveVault(id string) error {
	cfg, err := LoadConfig()
	if err != nil {
		return err
	}
	var remaining []VaultConfig
	for _, v := range cfg.Vaults {
		if v.ID != id {
			remaining = append(remaining, v)
		}
	}
	cfg.Vaults = remaining
	return SaveConfig(cfg)
}

func UpdateVault(id, name, path string) error {
	cfg, err := LoadConfig()
	if err != nil {
		return err
	}
	for i, v := range cfg.Vaults {
		if v.ID == id {
			cfg.Vaults[i].Name = name
			cfg.Vaults[i].Path = path
			break
		}
	}
	return SaveConfig(cfg)
}
