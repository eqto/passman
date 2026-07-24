package state

import (
	"sync"

	"github.com/eqto/passman/pkg/vault"
)

type OpenVault struct {
	Vault *vault.VaultFile
	Key   []byte
}

type SaveJob struct {
	Vault *vault.VaultFile
	Key   []byte
}

type AppState struct {
	mu        sync.RWMutex
	openVaults map[string]*OpenVault
	saveCh    chan<- SaveJob
}

func NewAppState(saveCh chan<- SaveJob) *AppState {
	return &AppState{
		openVaults: map[string]*OpenVault{},
		saveCh:     saveCh,
	}
}

func (s *AppState) InsertVault(path string, v *vault.VaultFile, key [32]byte) {
	s.mu.Lock()
	defer s.mu.Unlock()
	keyCopy := make([]byte, len(key))
	copy(keyCopy, key[:])
	s.openVaults[path] = &OpenVault{
		Vault: v,
		Key:   keyCopy,
	}
}

func (s *AppState) RemoveVault(path string) {
	s.mu.Lock()
	defer s.mu.Unlock()
	delete(s.openVaults, path)
}

func (s *AppState) IsOpen(path string) bool {
	s.mu.RLock()
	defer s.mu.RUnlock()
	_, ok := s.openVaults[path]
	return ok
}

func (s *AppState) GetVault(path string) (*OpenVault, bool) {
	s.mu.RLock()
	defer s.mu.RUnlock()
	ov, ok := s.openVaults[path]
	return ov, ok
}

func (s *AppState) WithOpenVault(path string, fn func(ov *OpenVault) error) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	ov, ok := s.openVaults[path]
	if !ok {
		return errVaultNotOpen
	}
	return fn(ov)
}

func (s *AppState) WithOpenVaultSave(path string, fn func(ov *OpenVault) error) error {
	err := s.WithOpenVault(path, fn)
	if err == nil {
		s.ScheduleSave(path)
	}
	return err
}

func (s *AppState) ScheduleSave(path string) {
	s.mu.RLock()
	ov, ok := s.openVaults[path]
	s.mu.RUnlock()
	if !ok || ov.Key == nil {
		return
	}
	job := SaveJob{
		Vault: ov.Vault,
		Key:   make([]byte, len(ov.Key)),
	}
	copy(job.Key, ov.Key)
	select {
	case s.saveCh <- job:
	default:
	}
}

var errVaultNotOpen = &stateError{"no vault is open"}

type stateError struct{ msg string }

func (e *stateError) Error() string { return e.msg }
