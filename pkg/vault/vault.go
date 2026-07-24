package vault

import (
	"encoding/base64"
	"encoding/json"
	"errors"
	"time"

	"github.com/eqto/passman/pkg/crypto"
)

func CreateVaultFile(path, name, password string) (*VaultFile, error) {
	vault, _, err := CreateVaultFileWithKey(path, name, password)
	return vault, err
}

func CreateVaultFileWithKey(path, name, password string) (*VaultFile, [crypto.KeySize]byte, error) {
	return CreateVaultFileWithLevel(path, name, password, crypto.SecurityLevelMedium)
}

func CreateVaultFileWithLevel(path, name, password string, level crypto.SecurityLevel) (*VaultFile, [crypto.KeySize]byte, error) {
	kdfParams := level.KdfParams()
	vaultKey, err := crypto.DeriveKey(password, &kdfParams)
	if err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}
	dek := crypto.RandomBytes(crypto.KeySize)
	var dekArray [crypto.KeySize]byte
	copy(dekArray[:], dek)

	encryptedDEK := crypto.Encrypt(dek, &vaultKey)

	now := time.Now().UTC()
	payload := VaultPayload{
		Name:      name,
		CreatedAt: now,
		UpdatedAt: now,
	}
	payloadJSON, err := json.Marshal(payload)
	if err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}
	encryptedPayload := crypto.Encrypt(payloadJSON, &dekArray)

	header := VaultHeader{
		Version:      Version,
		Cipher:       "AES-256-GCM",
		KDF:          "Argon2id",
		KdfParams:    crypto.KdfParamsToJSON(&kdfParams),
		EncryptedDEK: base64.StdEncoding.EncodeToString(encryptedDEK.Bytes),
		DekNonce:     base64.StdEncoding.EncodeToString(encryptedDEK.Nonce),
		PayloadNonce: base64.StdEncoding.EncodeToString(encryptedPayload.Nonce),
		CreatedAt:    now,
		UpdatedAt:    now,
	}

	if err := WriteVaultFile(path, &header, encryptedPayload.Bytes); err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}

	vault := &VaultFile{
		Header:    header,
		Payload:   payload,
		Path:      path,
		NeedsSave: false,
	}
	return vault, vaultKey, nil
}

func OpenVaultFile(path, password string) (*VaultFile, error) {
	vault, _, err := OpenVaultFileWithKey(path, password)
	return vault, err
}

func OpenVaultFileWithKey(path, password string) (*VaultFile, [crypto.KeySize]byte, error) {
	header, encryptedPayload, err := ReadVaultFile(path)
	if err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}
	kdfParams, err := crypto.KdfParamsFromJSON(&header.KdfParams)
	if err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}
	vaultKey, err := crypto.DeriveKey(password, &kdfParams)
	if err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}

	encryptedDEK, err := base64.StdEncoding.DecodeString(header.EncryptedDEK)
	if err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}
	dekNonce, err := base64.StdEncoding.DecodeString(header.DekNonce)
	if err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}
	dek, err := crypto.Decrypt(encryptedDEK, &vaultKey, dekNonce)
	if err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}
	if len(dek) != crypto.KeySize {
		return nil, [crypto.KeySize]byte{}, errors.New("invalid file format")
	}
	var dekArray [crypto.KeySize]byte
	copy(dekArray[:], dek)

	payloadNonce, err := base64.StdEncoding.DecodeString(header.PayloadNonce)
	if err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}
	payloadJSON, err := crypto.Decrypt(encryptedPayload, &dekArray, payloadNonce)
	if err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}
	var payload VaultPayload
	if err := json.Unmarshal(payloadJSON, &payload); err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}

	vault := &VaultFile{
		Header:    *header,
		Payload:   payload,
		Path:      path,
		NeedsSave: false,
	}
	return vault, vaultKey, nil
}

func SaveVaultFile(vault *VaultFile, password string) error {
	kdfParams, err := crypto.KdfParamsFromJSON(&vault.Header.KdfParams)
	if err != nil {
		return err
	}
	vaultKey, err := crypto.DeriveKey(password, &kdfParams)
	if err != nil {
		return err
	}
	return SaveVaultFileWithKey(vault, vaultKey[:])
}

func SaveVaultFileWithKey(vault *VaultFile, vaultKey []byte) error {
	if len(vaultKey) != crypto.KeySize {
		return errors.New("invalid key size")
	}
	var vaultKeyArray [crypto.KeySize]byte
	copy(vaultKeyArray[:], vaultKey)

	encryptedDEK, err := base64.StdEncoding.DecodeString(vault.Header.EncryptedDEK)
	if err != nil {
		return err
	}
	dekNonce, err := base64.StdEncoding.DecodeString(vault.Header.DekNonce)
	if err != nil {
		return err
	}
	dek, err := crypto.Decrypt(encryptedDEK, &vaultKeyArray, dekNonce)
	if err != nil {
		return err
	}
	if len(dek) != crypto.KeySize {
		return errors.New("invalid file format")
	}
	var dekArray [crypto.KeySize]byte
	copy(dekArray[:], dek)

	payloadJSON, err := json.Marshal(vault.Payload)
	if err != nil {
		return err
	}
	encryptedPayload := crypto.Encrypt(payloadJSON, &dekArray)

	header := vault.Header
	header.UpdatedAt = time.Now().UTC()
	header.PayloadNonce = base64.StdEncoding.EncodeToString(encryptedPayload.Nonce)

	return WriteVaultFile(vault.Path, &header, encryptedPayload.Bytes)
}

func ChangeKdfParams(vault *VaultFile, password string, newLevel crypto.SecurityLevel) (*VaultHeader, [crypto.KeySize]byte, error) {
	oldKdfParams, err := crypto.KdfParamsFromJSON(&vault.Header.KdfParams)
	if err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}
	oldVaultKey, err := crypto.DeriveKey(password, &oldKdfParams)
	if err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}

	encryptedDEK, err := base64.StdEncoding.DecodeString(vault.Header.EncryptedDEK)
	if err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}
	dekNonce, err := base64.StdEncoding.DecodeString(vault.Header.DekNonce)
	if err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}
	dek, err := crypto.Decrypt(encryptedDEK, &oldVaultKey, dekNonce)
	if err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}

	newKdfParams := newLevel.KdfParams()
	newVaultKey, err := crypto.DeriveKey(password, &newKdfParams)
	if err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}
	reEncryptedDEK := crypto.Encrypt(dek, &newVaultKey)

	newHeader := vault.Header
	newHeader.KdfParams = crypto.KdfParamsToJSON(&newKdfParams)
	newHeader.EncryptedDEK = base64.StdEncoding.EncodeToString(reEncryptedDEK.Bytes)
	newHeader.DekNonce = base64.StdEncoding.EncodeToString(reEncryptedDEK.Nonce)
	newHeader.UpdatedAt = time.Now().UTC()

	_, encryptedPayload, err := ReadVaultFile(vault.Path)
	if err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}
	if err := WriteVaultFile(vault.Path, &newHeader, encryptedPayload); err != nil {
		return nil, [crypto.KeySize]byte{}, err
	}

	return &newHeader, newVaultKey, nil
}
