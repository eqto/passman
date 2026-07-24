package buttercup

import (
	"encoding/json"
	"os"
	"strings"
)

func DecryptButtercupFile(path, password string) (*ButtercupVault, error) {
	contents, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	return DecryptButtercupVault(string(contents), password)
}

func DecryptButtercupVault(contents, password string) (*ButtercupVault, error) {
	var format Format
	var sigLen int

	if strings.HasPrefix(contents, FormatBSignature) {
		format = FormatB
		sigLen = len(FormatBSignature)
	} else if strings.HasPrefix(contents, FormatASignature) {
		format = FormatA
		sigLen = len(FormatASignature)
	} else {
		return nil, &ButtercupError{Msg: "invalid signature"}
	}

	encryptedText := contents[sigLen:]
	components, err := ParseEncryptedComponents(encryptedText)
	if err != nil {
		return nil, err
	}
	compressed, err := DecryptComponents(components, password)
	if err != nil {
		return nil, err
	}
	decompressed, err := Decompress(compressed)
	if err != nil {
		return nil, err
	}

	switch format {
	case FormatA:
		return ParseFormatA(decompressed)
	case FormatB:
		var raw RawVault
		if err := json.Unmarshal([]byte(decompressed), &raw); err != nil {
			return nil, err
		}

		trashGroupID, trashGroupIDs := IdentifyTrashGroups(&raw)
		groups, trashGroups := BuildGroups(raw.G, trashGroupID, trashGroupIDs)
		entries, trashEntries := BuildEntries(raw.E, trashGroupIDs, trashGroupID)

		var name string
		if v, ok := raw.A["name"]; ok {
			name = v.Value
		}

		return &ButtercupVault{
			Name:    name,
			UUID:    &raw.ID,
			Groups:  groups,
			Entries: entries,
			Trash: ButtercupTrash{
				Groups:  trashGroups,
				Entries: trashEntries,
			},
		}, nil
	}
	return nil, &ButtercupError{Msg: "unknown format"}
}
