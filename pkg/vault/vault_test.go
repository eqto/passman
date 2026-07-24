package vault

import (
	"os"
	"path/filepath"
	"testing"
)

func TestCreateAndOpenVault(t *testing.T) {
	tmpDir := t.TempDir()
	path := filepath.Join(tmpDir, "test.pmv")

	v, err := CreateVaultFile(path, "Test Vault", "password123")
	if err != nil {
		t.Fatalf("CreateVaultFile failed: %v", err)
	}

	if v.Payload.Name != "Test Vault" {
		t.Errorf("expected name 'Test Vault', got '%s'", v.Payload.Name)
	}

	if v.Path != path {
		t.Errorf("expected path '%s', got '%s'", path, v.Path)
	}

	if _, err := os.Stat(path); err != nil {
		t.Fatalf("vault file not created: %v", err)
	}

	// Open the vault
	opened, err := OpenVaultFile(path, "password123")
	if err != nil {
		t.Fatalf("OpenVaultFile failed: %v", err)
	}

	if opened.Payload.Name != "Test Vault" {
		t.Errorf("expected opened name 'Test Vault', got '%s'", opened.Payload.Name)
	}
}

func TestCreateAndOpenVaultWrongPassword(t *testing.T) {
	tmpDir := t.TempDir()
	path := filepath.Join(tmpDir, "test.pmv")

	_, err := CreateVaultFile(path, "Test Vault", "correct")
	if err != nil {
		t.Fatalf("CreateVaultFile failed: %v", err)
	}

	_, err = OpenVaultFile(path, "wrong")
	if err == nil {
		t.Fatal("expected error for wrong password, got nil")
	}
}

func TestSaveVaultFile(t *testing.T) {
	tmpDir := t.TempDir()
	path := filepath.Join(tmpDir, "test.pmv")

	v, err := CreateVaultFile(path, "Test Vault", "password")
	if err != nil {
		t.Fatalf("CreateVaultFile failed: %v", err)
	}

	// Add a group
	v.Payload.Groups = append(v.Payload.Groups, Group{
		ID:   "g1",
		Name: "General",
	})

	// Add an entry
	v.Payload.Entries = append(v.Payload.Entries, VaultEntry{
		ID:      "e1",
		Title:   "Test Entry",
		GroupID: &[]string{"g1"}[0],
	})

	if err := SaveVaultFile(v, "password"); err != nil {
		t.Fatalf("SaveVaultFile failed: %v", err)
	}

	// Reopen and verify
	opened, err := OpenVaultFile(path, "password")
	if err != nil {
		t.Fatalf("OpenVaultFile failed: %v", err)
	}

	if len(opened.Payload.Groups) != 1 {
		t.Fatalf("expected 1 group, got %d", len(opened.Payload.Groups))
	}
	if opened.Payload.Groups[0].Name != "General" {
		t.Errorf("expected group name 'General', got '%s'", opened.Payload.Groups[0].Name)
	}

	if len(opened.Payload.Entries) != 1 {
		t.Fatalf("expected 1 entry, got %d", len(opened.Payload.Entries))
	}
	if opened.Payload.Entries[0].Title != "Test Entry" {
		t.Errorf("expected entry title 'Test Entry', got '%s'", opened.Payload.Entries[0].Title)
	}
}
