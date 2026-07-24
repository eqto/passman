package buttercup

import (
	"bytes"
	"compress/gzip"
	"crypto/aes"
	"crypto/cipher"
	"crypto/hmac"
	"crypto/sha256"
	"encoding/base64"
	"encoding/hex"
	"fmt"
	"io"
	"strconv"
	"strings"

	"golang.org/x/crypto/pbkdf2"
)

func ParseEncryptedComponents(encryptedText string) (*EncryptedComponents, error) {
	parts := strings.Split(encryptedText, "$")
	if len(parts) < 5 {
		return nil, &ButtercupError{Msg: "invalid encrypted format"}
	}

	rounds, err := strconv.ParseUint(parts[4], 10, 32)
	if err != nil {
		return nil, &ButtercupError{Msg: fmt.Sprintf("invalid rounds: %s", parts[4])}
	}

	method := DefaultAlgorithm
	if len(parts) >= 6 {
		method = strings.ToLower(parts[5])
	}

	return &EncryptedComponents{
		Content: parts[0],
		IV:      parts[1],
		Salt:    parts[2],
		Auth:    parts[3],
		Rounds:  uint32(rounds),
		Method:  method,
	}, nil
}

func DecryptComponents(components *EncryptedComponents, password string) (string, error) {
	switch components.Method {
	case "cbc":
		return decryptCBC(components, password)
	case "gcm":
		return decryptGCM(components, password)
	default:
		return "", &ButtercupError{Msg: fmt.Sprintf("unsupported algorithm: %s", components.Method)}
	}
}

func decodeCommonComponents(c *EncryptedComponents) ([]byte, []byte, []byte, error) {
	contentBytes, err := base64.StdEncoding.DecodeString(c.Content)
	if err != nil {
		return nil, nil, nil, err
	}
	ivBytes, err := hex.DecodeString(c.IV)
	if err != nil {
		return nil, nil, nil, err
	}
	authBytes, err := hex.DecodeString(c.Auth)
	if err != nil {
		return nil, nil, nil, err
	}
	return contentBytes, ivBytes, authBytes, nil
}

func deriveKey(password string, salt []byte, rounds uint32, length int) []byte {
	return pbkdf2.Key([]byte(password), salt, int(rounds), length, sha256.New)
}

func decryptCBC(c *EncryptedComponents, password string) (string, error) {
	saltBytes := []byte(c.Salt)
	derived := deriveKey(password, saltBytes, c.Rounds, PasswordKeySize+HMACKeySize)
	key := derived[:PasswordKeySize]
	hmacKey := derived[PasswordKeySize:]

	contentBytes, ivBytes, authBytes, err := decodeCommonComponents(c)
	if err != nil {
		return "", err
	}

	mac := hmac.New(sha256.New, hmacKey)
	mac.Write([]byte(c.Content))
	mac.Write([]byte(c.IV))
	mac.Write([]byte(c.Salt))
	expectedHMAC := mac.Sum(nil)
	if !hmac.Equal(expectedHMAC, authBytes) {
		return "", &ButtercupError{Msg: "authentication failed"}
	}

	block, err := aes.NewCipher(key)
	if err != nil {
		return "", &ButtercupError{Msg: err.Error()}
	}
	if len(ivBytes) != aes.BlockSize {
		return "", &ButtercupError{Msg: "invalid IV length"}
	}
	mode := cipher.NewCBCDecrypter(block, ivBytes)

	plaintext := make([]byte, len(contentBytes))
	mode.CryptBlocks(plaintext, contentBytes)

	plaintext, err = pkcs7Unpad(plaintext, aes.BlockSize)
	if err != nil {
		return "", &ButtercupError{Msg: err.Error()}
	}

	return string(plaintext), nil
}

func decryptGCM(c *EncryptedComponents, password string) (string, error) {
	saltBytes := []byte(c.Salt)
	key := deriveKey(password, saltBytes, c.Rounds, PasswordKeySize)

	contentBytes, ivBytes, authTagBytes, err := decodeCommonComponents(c)
	if err != nil {
		return "", err
	}

	block, err := aes.NewCipher(key)
	if err != nil {
		return "", &ButtercupError{Msg: err.Error()}
	}
	gcm, err := cipher.NewGCM(block)
	if err != nil {
		return "", &ButtercupError{Msg: err.Error()}
	}

	fullCiphertext := append(contentBytes, authTagBytes...)
	aad := []byte(c.IV + c.Salt)

	plaintext, err := gcm.Open(nil, ivBytes, fullCiphertext, aad)
	if err != nil {
		return "", &ButtercupError{Msg: err.Error()}
	}

	return string(plaintext), nil
}

func Decompress(compressed string) (string, error) {
	compressedBytes, err := base64.StdEncoding.DecodeString(compressed)
	if err != nil {
		return "", err
	}
	r, err := gzip.NewReader(bytes.NewReader(compressedBytes))
	if err != nil {
		return "", &ButtercupError{Msg: err.Error()}
	}
	defer r.Close()
	var buf bytes.Buffer
	if _, err := io.Copy(&buf, r); err != nil {
		return "", &ButtercupError{Msg: err.Error()}
	}
	return buf.String(), nil
}

func pkcs7Unpad(data []byte, blockSize int) ([]byte, error) {
	if len(data) == 0 {
		return nil, fmt.Errorf("invalid padding")
	}
	padding := int(data[len(data)-1])
	if padding < 1 || padding > blockSize {
		return nil, fmt.Errorf("invalid padding")
	}
	if len(data) < padding {
		return nil, fmt.Errorf("invalid padding")
	}
	return data[:len(data)-padding], nil
}
