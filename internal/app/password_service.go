package app

import (
	"crypto/rand"
	"fmt"
	"math/big"
)

type PasswordOptions struct {
	Length        int  `json:"length"`
	Uppercase     bool `json:"uppercase"`
	Lowercase     bool `json:"lowercase"`
	Digits        bool `json:"digits"`
	Space         bool `json:"space"`
	UnderscoreDash bool `json:"underscore_dash"`
	Symbols       bool `json:"symbols"`
}

type PasswordService struct{}

func NewPasswordService() *PasswordService {
	return &PasswordService{}
}

func (s *PasswordService) GeneratePassword(opts PasswordOptions) (string, error) {
	var charset []byte
	if opts.Uppercase {
		charset = append(charset, []byte("ABCDEFGHIJKLMNOPQRSTUVWXYZ")...)
	}
	if opts.Lowercase {
		charset = append(charset, []byte("abcdefghijklmnopqrstuvwxyz")...)
	}
	if opts.Digits {
		charset = append(charset, []byte("0123456789")...)
	}
	if opts.Space {
		charset = append(charset, ' ')
	}
	if opts.UnderscoreDash {
		charset = append(charset, '_', '-')
	}
	if opts.Symbols {
		charset = append(charset, []byte("!@#$%^&*=+?")...)
	}
	if len(charset) == 0 {
		return "", fmt.Errorf("at least one character set must be selected")
	}
	if opts.Length == 0 {
		return "", fmt.Errorf("password length must be greater than 0")
	}

	password := make([]byte, opts.Length)
	for i := 0; i < opts.Length; i++ {
		idx, err := rand.Int(rand.Reader, big.NewInt(int64(len(charset))))
		if err != nil {
			return "", err
		}
		password[i] = charset[idx.Int64()]
	}
	return string(password), nil
}
