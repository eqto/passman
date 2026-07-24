package vault

import (
	"time"

	"github.com/eqto/passman/pkg/crypto"
)

const (
	Magic             = "PMV "
	Version    uint32 = 1
	PayloadFormatVersion uint32 = 1
)

type VaultError struct {
	Msg string
}

func (e *VaultError) Error() string { return e.Msg }

func newError(msg string) *VaultError { return &VaultError{Msg: msg} }

type VaultHeader struct {
	Version        uint32                 `json:"version"`
	Cipher         string                 `json:"cipher"`
	KDF            string                 `json:"kdf"`
	KdfParams      KdfParamsJSON          `json:"kdf_params"`
	EncryptedDEK   string                 `json:"encrypted_dek"`
	DekNonce       string                 `json:"dek_nonce"`
	PayloadNonce   string                 `json:"payload_nonce"`
	CreatedAt      time.Time              `json:"created_at"`
	UpdatedAt      time.Time              `json:"updated_at"`
}

// KdfParamsJSON is re-exported here for convenience.
type KdfParamsJSON = crypto.KdfParamsJSON

type Trash struct {
	Groups  []Group      `json:"groups"`
	Entries []VaultEntry `json:"entries"`
}

type CustomField struct {
	ID        string `json:"id"`
	Label     string `json:"label"`
	Type      string `json:"type"`
	Value     string `json:"value"`
}

type HistoryItem struct {
	Property  string    `json:"property"`
	Value     string    `json:"value"`
	UpdatedAt time.Time `json:"updated_at"`
}

type Group struct {
	ID       string  `json:"id"`
	Name     string  `json:"name"`
	ParentID *string `json:"parent_id,omitempty"`
}

type VaultEntry struct {
	ID        string         `json:"id"`
	Title     string         `json:"title"`
	Username  string         `json:"username"`
	Password  string         `json:"password"`
	Tags      []string       `json:"tags"`
	Fields    []CustomField  `json:"fields"`
	GroupID   *string        `json:"group_id,omitempty"`
	CreatedAt time.Time      `json:"created_at"`
	UpdatedAt time.Time      `json:"updated_at"`
	DeletedAt *time.Time     `json:"deleted_at,omitempty"`
	History   []HistoryItem  `json:"history"`
}

type VaultPayload struct {
	Name      string       `json:"name"`
	UUID      *string      `json:"uuid,omitempty"`
	CreatedAt time.Time    `json:"created_at"`
	UpdatedAt time.Time    `json:"updated_at"`
	Groups    []Group      `json:"groups"`
	Tags      []string     `json:"tags"`
	Entries   []VaultEntry `json:"entries"`
	Trash     Trash        `json:"trash"`
}

func (p *VaultPayload) Touch() {
	p.UpdatedAt = time.Now().UTC()
}

type VaultFile struct {
	Header     VaultHeader
	Payload    VaultPayload
	Path       string
	NeedsSave  bool
}

// VaultFileDTO is the serializable representation sent to the frontend.
type VaultFileDTO struct {
	Path    string       `json:"path"`
	Name    string       `json:"name"`
	Groups  []Group      `json:"groups"`
	Tags    []string     `json:"tags"`
	Entries []VaultEntry `json:"entries"`
	Trash   Trash        `json:"trash"`
}

func VaultToDTO(v *VaultFile) VaultFileDTO {
	return VaultFileDTO{
		Path:    v.Path,
		Name:    v.Payload.Name,
		Groups:  v.Payload.Groups,
		Tags:    v.Payload.Tags,
		Entries: v.Payload.Entries,
		Trash:   v.Payload.Trash,
	}
}
