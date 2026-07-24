package buttercup

import "time"

const (
	FormatASignature = "b~>buttercup/a"
	FormatBSignature = "b~>buttercup/b"
	DefaultAlgorithm = "cbc"
	PasswordKeySize  = 32
	HMACKeySize      = 32
)

type Format int

const (
	FormatA Format = iota
	FormatB
)

type ButtercupError struct {
	Msg string
}

func (e *ButtercupError) Error() string { return e.Msg }

type ButtercupVault struct {
	Name    string
	UUID    *string
	Groups  []ButtercupGroup
	Entries []ButtercupEntry
	Trash   ButtercupTrash
}

type ButtercupTrash struct {
	Groups  []ButtercupGroup
	Entries []ButtercupEntry
}

type ButtercupGroup struct {
	ID       string
	Name     string
	ParentID *string
}

type ButtercupCustomField struct {
	ID        string
	Label     string
	FieldType string
	Value     string
}

type ButtercupEntry struct {
	ID        string
	GroupID   *string
	Title     string
	Username  string
	Password  string
	Fields    []ButtercupCustomField
	DeletedAt *time.Time
	History   []HistoryItem
}

type HistoryItem struct {
	Property  string
	Value     string
	UpdatedAt time.Time
}

type EncryptedComponents struct {
	Content string
	IV      string
	Salt    string
	Auth    string
	Rounds  uint32
	Method  string
}

// RawVault structures for format B JSON parsing
type RawVault struct {
	ID string            `json:"_id"`
	A  map[string]RawValue `json:"a"`
	G  []RawGroup         `json:"g"`
	E  []RawEntry         `json:"e"`
	C  string             `json:"_c"`
}

type RawGroup struct {
	ID string            `json:"id"`
	G  string            `json:"g"`
	T  string            `json:"t"`
	A  map[string]RawValue `json:"a"`
}

type RawEntry struct {
	ID      string            `json:"id"`
	G       string            `json:"g"`
	P       map[string]RawValue `json:"p"`
	A       map[string]RawValue `json:"a"`
	Deleted *uint64           `json:"deleted"`
}

type RawValue struct {
	Value   string          `json:"value"`
	Deleted *uint64         `json:"deleted"`
	History []RawHistoryItem `json:"history"`
}

type RawHistoryItem struct {
	Value   string `json:"value"`
	Updated uint64 `json:"updated"`
}
