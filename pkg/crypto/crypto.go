package crypto

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"

	"golang.org/x/crypto/argon2"
)

const (
	KeySize   = 32
	NonceSize = 12
	TagSize   = 16
	SaltSize  = 16
)

type SecurityLevel string

const (
	SecurityLevelLow    SecurityLevel = "low"
	SecurityLevelMedium SecurityLevel = "medium"
	SecurityLevelSecure SecurityLevel = "secure"
	SecurityLevelBest   SecurityLevel = "best"
)

func ParseSecurityLevel(s string) (SecurityLevel, error) {
	switch s {
	case "low":
		return SecurityLevelLow, nil
	case "medium":
		return SecurityLevelMedium, nil
	case "secure":
		return SecurityLevelSecure, nil
	case "best":
		return SecurityLevelBest, nil
	default:
		return "", fmt.Errorf("unknown security level: %s", s)
	}
}

func (l SecurityLevel) Label() string {
	return string(l)
}

func (l SecurityLevel) KdfParams() KdfParams {
	salt := RandomBytes(SaltSize)
	var memoryKB, iterations, parallelism uint32
	switch l {
	case SecurityLevelLow:
		memoryKB, iterations, parallelism = 32768, 2, 2
	case SecurityLevelMedium:
		memoryKB, iterations, parallelism = 65536, 3, 4
	case SecurityLevelSecure:
		memoryKB, iterations, parallelism = 131072, 4, 4
	case SecurityLevelBest:
		memoryKB, iterations, parallelism = 262144, 6, 8
	}
	return KdfParams{
		Salt:       [SaltSize]byte(salt),
		MemoryKB:   memoryKB,
		Iterations: iterations,
		Parallelism: parallelism,
	}
}

type KdfParams struct {
	Salt        [SaltSize]byte
	MemoryKB    uint32
	Iterations  uint32
	Parallelism uint32
}

func DefaultKdfParams() KdfParams {
	return SecurityLevelMedium.KdfParams()
}

func RandomBytes(n int) []byte {
	b := make([]byte, n)
	if _, err := rand.Read(b); err != nil {
		panic(err)
	}
	return b
}

func DeriveKey(password string, params *KdfParams) ([KeySize]byte, error) {
	key := argon2.IDKey(
		[]byte(password),
		params.Salt[:],
		params.Iterations,
		params.MemoryKB,
		uint8(params.Parallelism),
		KeySize,
	)
	var result [KeySize]byte
	copy(result[:], key)
	return result, nil
}

type Ciphertext struct {
	Nonce []byte
	Bytes []byte
}

func Encrypt(plaintext []byte, key *[KeySize]byte) Ciphertext {
	block, err := aes.NewCipher(key[:])
	if err != nil {
		panic(err)
	}
	gcm, err := cipher.NewGCM(block)
	if err != nil {
		panic(err)
	}
	nonce := make([]byte, gcm.NonceSize())
	if _, err := rand.Read(nonce); err != nil {
		panic(err)
	}
	encrypted := gcm.Seal(nil, nonce, plaintext, nil)
	return Ciphertext{Nonce: nonce, Bytes: encrypted}
}

func Decrypt(ciphertext []byte, key *[KeySize]byte, nonce []byte) ([]byte, error) {
	block, err := aes.NewCipher(key[:])
	if err != nil {
		return nil, err
	}
	gcm, err := cipher.NewGCM(block)
	if err != nil {
		return nil, err
	}
	plaintext, err := gcm.Open(nil, nonce, ciphertext, nil)
	if err != nil {
		return nil, errors.New("incorrect password")
	}
	return plaintext, nil
}

// KdfParamsJSON is the JSON-serializable form of KdfParams (salt is base64-encoded).
type KdfParamsJSON struct {
	Salt        string `json:"salt"`
	MemoryKB    uint32 `json:"memory_kb"`
	Iterations  uint32 `json:"iterations"`
	Parallelism uint32 `json:"parallelism"`
}

func KdfParamsToJSON(p *KdfParams) KdfParamsJSON {
	return KdfParamsJSON{
		Salt:        base64.StdEncoding.EncodeToString(p.Salt[:]),
		MemoryKB:    p.MemoryKB,
		Iterations:  p.Iterations,
		Parallelism: p.Parallelism,
	}
}

func KdfParamsFromJSON(j *KdfParamsJSON) (KdfParams, error) {
	saltBytes, err := base64.StdEncoding.DecodeString(j.Salt)
	if err != nil {
		return KdfParams{}, err
	}
	if len(saltBytes) != SaltSize {
		return KdfParams{}, errors.New("invalid salt length")
	}
	var p KdfParams
	copy(p.Salt[:], saltBytes)
	p.MemoryKB = j.MemoryKB
	p.Iterations = j.Iterations
	p.Parallelism = j.Parallelism
	return p, nil
}

// MarshalJSON for KdfParams so it can be embedded directly if needed.
func (p KdfParams) MarshalJSON() ([]byte, error) {
	return json.Marshal(KdfParamsToJSON(&p))
}

// UnmarshalJSON for KdfParams.
func (p *KdfParams) UnmarshalJSON(data []byte) error {
	var j KdfParamsJSON
	if err := json.Unmarshal(data, &j); err != nil {
		return err
	}
	params, err := KdfParamsFromJSON(&j)
	if err != nil {
		return err
	}
	*p = params
	return nil
}
