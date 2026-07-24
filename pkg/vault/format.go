package vault

import (
	"encoding/binary"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
)

func VaultExists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}

func ReadVaultFile(path string) (*VaultHeader, []byte, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, nil, err
	}
	_, rest, err := readMagicAndVersion(data)
	if err != nil {
		return nil, nil, err
	}
	headerJSON, rest, err := readHeader(rest)
	if err != nil {
		return nil, nil, err
	}
	payload, err := readPayload(rest)
	if err != nil {
		return nil, nil, err
	}
	return headerJSON, payload, nil
}

func readMagicAndVersion(data []byte) (uint32, []byte, error) {
	magicLen := len(Magic)
	versionLen := 2
	if len(data) < magicLen+versionLen {
		return 0, nil, newError("invalid file format")
	}
	if string(data[0:magicLen]) != Magic {
		return 0, nil, newError("invalid magic bytes")
	}
	fileVersion := uint32(binary.LittleEndian.Uint16(data[magicLen : magicLen+versionLen]))
	if fileVersion != Version {
		return 0, nil, fmt.Errorf("unsupported version: %d", fileVersion)
	}
	return fileVersion, data[magicLen+versionLen:], nil
}

func readHeader(data []byte) (*VaultHeader, []byte, error) {
	if len(data) < 2 {
		return nil, nil, newError("invalid file format")
	}
	headerLen := int(binary.LittleEndian.Uint16(data[0:2]))
	headerStart := 2
	headerEnd := headerStart + headerLen
	if headerEnd > len(data) {
		return nil, nil, newError("invalid file format")
	}
	var header VaultHeader
	if err := json.Unmarshal(data[headerStart:headerEnd], &header); err != nil {
		return nil, nil, err
	}
	if header.Version != Version {
		return nil, nil, fmt.Errorf("unsupported version: %d", header.Version)
	}
	return &header, data[headerEnd:], nil
}

func readPayload(data []byte) ([]byte, error) {
	if len(data) < 8 {
		return nil, newError("invalid file format")
	}
	payloadLen := int(binary.LittleEndian.Uint64(data[0:8]))
	payloadStart := 8
	payloadEnd := payloadStart + payloadLen
	if payloadEnd > len(data) {
		return nil, newError("invalid file format")
	}
	return data[payloadStart:payloadEnd], nil
}

func WriteVaultFile(path string, header *VaultHeader, encryptedPayload []byte) error {
	headerJSON, err := json.Marshal(header)
	if err != nil {
		return err
	}
	headerLen := uint16(len(headerJSON))
	payloadLen := uint64(len(encryptedPayload))

	data := make([]byte, 0, len(Magic)+2+2+len(headerJSON)+8+len(encryptedPayload))
	data = append(data, []byte(Magic)...)
	data = append(data, byte(Version), byte(Version>>8))
	data = append(data, byte(headerLen), byte(headerLen>>8))
	data = append(data, headerJSON...)
	for i := 0; i < 8; i++ {
		data = append(data, byte(payloadLen>>(i*8)))
	}
	data = append(data, encryptedPayload...)

	tempPath := filepath.Join(filepath.Dir(path), filepath.Base(path)+".tmp")
	if err := os.WriteFile(tempPath, data, 0600); err != nil {
		return err
	}
	if err := os.Rename(tempPath, path); err != nil {
		_ = os.Remove(tempPath)
		return err
	}
	return nil
}
