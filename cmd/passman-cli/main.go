package main

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/eqto/passman/internal/vimport"
	"github.com/eqto/passman/pkg/buttercup"
	"github.com/eqto/passman/pkg/crypto"
	"github.com/eqto/passman/pkg/keepass"
	"github.com/eqto/passman/pkg/vault"

	"github.com/spf13/cobra"
	"golang.org/x/term"
)

func main() {
	rootCmd := &cobra.Command{
		Use:   "passman-cli",
		Short: "Passman CLI import/export tools",
	}

	rootCmd.AddCommand(
		createCmd(),
		importCmd(),
		exportButtercupCmd(),
		importButtercupCmd(),
		importKeePassCmd(),
		convertCmd(),
		extractCmd(),
	)

	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintf(os.Stderr, "error: %v\n", err)
		os.Exit(1)
	}
}

func createCmd() *cobra.Command {
	var name string
	cmd := &cobra.Command{
		Use:   "create [output]",
		Short: "Create a new empty Passman vault",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			password := promptPassword("New vault password: ")
			if _, err := vault.CreateVaultFile(args[0], name, password); err != nil {
				return err
			}
			fmt.Printf("Created vault: %s\n", args[0])
			return nil
		},
	}
	cmd.Flags().StringVarP(&name, "name", "n", "", "Vault name")
	cmd.MarkFlagRequired("name")
	return cmd
}

func importCmd() *cobra.Command {
	var name string
	cmd := &cobra.Command{
		Use:   "import [input] [output]",
		Short: "Import a JSON file into a .pmv vault",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) error {
			data, err := os.ReadFile(args[0])
			if err != nil {
				return err
			}
			var imported vimport.ImportJSON
			if err := json.Unmarshal(data, &imported); err != nil {
				return err
			}
			if name != "" {
				imported.Name = name
			}
			vaultName := imported.Name
			password := promptPassword("New vault password: ")
			v, err := createAndSaveVault(args[1], vaultName, password, imported)
			if err != nil {
				return err
			}
			fmt.Printf("Imported %d entries into %s\n", len(v.Payload.Entries), args[1])
			return nil
		},
	}
	cmd.Flags().StringVarP(&name, "name", "n", "", "Vault name")
	return cmd
}

func exportButtercupCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "export-buttercup [input] [output]",
		Short: "Export a Buttercup .bcup vault to a Passman JSON file",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) error {
			password := promptPasswordEnv("Buttercup master password: ", "BCUP_PASSWORD")
			bv, err := buttercup.DecryptButtercupFile(args[0], password)
			if err != nil {
				return err
			}
			impJSON := vimport.FromButtercupVault(bv)
			data, err := json.MarshalIndent(impJSON, "", "  ")
			if err != nil {
				return err
			}
			if err := os.WriteFile(args[1], data, 0644); err != nil {
				return err
			}
			fmt.Printf("Exported %d entries to %s\n", len(impJSON.Entries), args[1])
			return nil
		},
	}
	return cmd
}

func importButtercupCmd() *cobra.Command {
	var name string
	cmd := &cobra.Command{
		Use:   "import-buttercup [input] [output]",
		Short: "Import a Buttercup .bcup vault directly into a .pmv vault",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) error {
			bcupPassword := promptPasswordEnv("Buttercup master password: ", "BCUP_PASSWORD")
			pmvPassword := promptPassword("New vault password: ")

			bv, err := buttercup.DecryptButtercupFile(args[0], bcupPassword)
			if err != nil {
				return err
			}
			impJSON := vimport.FromButtercupVault(bv)
			vaultName := name
			if vaultName == "" {
				vaultName = vimport.DeriveVaultName(impJSON.Name, args[0])
			}
			impJSON.Name = vaultName

			v, err := createAndSaveVault(args[1], vaultName, pmvPassword, impJSON)
			if err != nil {
				return err
			}
			fmt.Printf("Imported %d entries from %s into %s\n", len(v.Payload.Entries), args[0], args[1])
			return nil
		},
	}
	cmd.Flags().StringVarP(&name, "name", "n", "", "Vault name")
	return cmd
}

func importKeePassCmd() *cobra.Command {
	var name string
	cmd := &cobra.Command{
		Use:   "import-keepass [input] [output]",
		Short: "Import a KeePass .kdbx database directly into a .pmv vault",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) error {
			kdbxPassword := promptPasswordEnv("KeePass master password: ", "KDBX_PASSWORD")
			pmvPassword := promptPassword("New vault password: ")

			kv, err := keepass.DecryptKeePassFile(args[0], kdbxPassword)
			if err != nil {
				return err
			}
			impJSON := vimport.FromKeePassVault(kv)
			vaultName := name
			if vaultName == "" {
				vaultName = vimport.DeriveVaultName(impJSON.Name, args[0])
			}
			impJSON.Name = vaultName

			v, err := createAndSaveVault(args[1], vaultName, pmvPassword, impJSON)
			if err != nil {
				return err
			}
			fmt.Printf("Imported %d entries from %s into %s\n", len(v.Payload.Entries), args[0], args[1])
			return nil
		},
	}
	cmd.Flags().StringVarP(&name, "name", "n", "", "Vault name")
	return cmd
}

func convertCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "convert [input] [output]",
		Short: "Convert a Buttercup .bcup vault to a .pmv vault",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) error {
			password := resolveConvertPassword()

			bv, err := buttercup.DecryptButtercupFile(args[0], password)
			if err != nil {
				return err
			}
			impJSON := vimport.FromButtercupVault(bv)
			vaultName := vimport.DeriveVaultName(impJSON.Name, args[0])
			impJSON.Name = vaultName

			v, err := createAndSaveVault(args[1], vaultName, password, impJSON)
			if err != nil {
				return err
			}
			fmt.Printf("Converted %d entries from %s into %s\n", len(v.Payload.Entries), args[0], args[1])
			return nil
		},
	}
	return cmd
}

func extractCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "extract [input]",
		Short: "Extract a .pmv vault into header.json and payload.json",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			password := promptPassword("Vault password: ")
			v, err := vault.OpenVaultFile(args[0], password)
			if err != nil {
				return err
			}
			dirName := strings.TrimSuffix(filepath.Base(args[0]), filepath.Ext(args[0]))
			if dirName == "" {
				dirName = "extracted"
			}
			if err := os.MkdirAll(dirName, 0755); err != nil {
				return err
			}
			headerData, err := json.MarshalIndent(v.Header, "", "  ")
			if err != nil {
				return err
			}
			payloadData, err := json.MarshalIndent(v.Payload, "", "  ")
			if err != nil {
				return err
			}
			if err := os.WriteFile(filepath.Join(dirName, "header.json"), headerData, 0644); err != nil {
				return err
			}
			if err := os.WriteFile(filepath.Join(dirName, "payload.json"), payloadData, 0644); err != nil {
				return err
			}
			fmt.Printf("Extracted vault to %s/\n", dirName)
			return nil
		},
	}
	return cmd
}

func createAndSaveVault(output, name, password string, imported vimport.ImportJSON) (*vault.VaultFile, error) {
	v, _, err := vault.CreateVaultFileWithLevel(output, name, password, crypto.SecurityLevelMedium)
	if err != nil {
		return nil, err
	}
	vimport.BuildPayload(v, imported)
	if err := vault.SaveVaultFile(v, password); err != nil {
		return nil, err
	}
	return v, nil
}

func promptPassword(prompt string) string {
	if p := os.Getenv("PASSMAN_PASSWORD"); p != "" {
		return p
	}
	fmt.Print(prompt)
	bytes, err := term.ReadPassword(int(os.Stdin.Fd()))
	fmt.Println()
	if err != nil {
		fmt.Fprintf(os.Stderr, "error reading password: %v\n", err)
		os.Exit(1)
	}
	return string(bytes)
}

func promptPasswordEnv(prompt, envVar string) string {
	if p := os.Getenv(envVar); p != "" {
		return p
	}
	fmt.Print(prompt)
	bytes, err := term.ReadPassword(int(os.Stdin.Fd()))
	fmt.Println()
	if err != nil {
		fmt.Fprintf(os.Stderr, "error reading password: %v\n", err)
		os.Exit(1)
	}
	return string(bytes)
}

func resolveConvertPassword() string {
	if p := os.Getenv("PASSMAN_PASSWORD"); p != "" {
		return p
	}
	if p := os.Getenv("BCUP_PASSWORD"); p != "" {
		return p
	}
	fmt.Print("Password: ")
	bytes, err := term.ReadPassword(int(os.Stdin.Fd()))
	fmt.Println()
	if err != nil {
		fmt.Fprintf(os.Stderr, "error reading password: %v\n", err)
		os.Exit(1)
	}
	return string(bytes)
}
