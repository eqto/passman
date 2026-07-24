package buttercup

import (
	"strconv"
	"strings"
	"unicode/utf8"
)

func parseQuotedString(input string) (string, int, bool) {
	if len(input) == 0 || input[0] != '"' {
		return "", 0, false
	}

	var result strings.Builder
	i := 1
	for i < len(input) {
		c := input[i]
		if c == '\\' {
			if i+1 < len(input) {
				switch input[i+1] {
				case '"':
					result.WriteByte('"')
				case '\\':
					result.WriteByte('\\')
				case 'n':
					result.WriteByte('\n')
				case 'r':
					result.WriteByte('\r')
				case 't':
					result.WriteByte('\t')
				case 'u':
					if i+5 < len(input) {
						hex := input[i+2 : i+6]
						if code, err := strconv.ParseUint(hex, 16, 32); err == nil {
							if r := rune(code); utf8.ValidRune(r) {
								result.WriteRune(r)
							}
						}
						i += 6
						continue
					}
				default:
					result.WriteByte(input[i+1])
				}
				i += 2
			} else {
				i++
			}
		} else if c == '"' {
			return result.String(), i + 1, true
		} else {
			result.WriteByte(c)
			i++
		}
	}
	return "", 0, false
}

func tokenizeLine(line string) (string, []string) {
	line = strings.TrimSpace(line)
	if line == "" {
		return "", nil
	}

	var tokens []string
	i := 0

	// First token is the command (non-quoted)
	for i < len(line) && line[i] != ' ' {
		i++
	}
	cmd := line[:i]

	// Parse remaining tokens
	for i < len(line) {
		// Skip whitespace
		for i < len(line) && line[i] == ' ' {
			i++
		}
		if i >= len(line) {
			break
		}

		if line[i] == '"' {
			if s, consumed, ok := parseQuotedString(line[i:]); ok {
				tokens = append(tokens, s)
				i += consumed
			} else {
				tokens = append(tokens, line[i:])
				break
			}
		} else {
			start := i
			for i < len(line) && line[i] != ' ' {
				i++
			}
			tokens = append(tokens, line[start:i])
		}
	}

	return cmd, tokens
}

func ParseFormatA(content string) (*ButtercupVault, error) {
	vaultName := ""
	var vaultUUID *string
	groups := map[string]*ButtercupGroup{}
	var groupOrder []string
	entries := map[string]*ButtercupEntry{}
	var entryOrder []string
	var deletedGroups []string
	var deletedEntries []string

	for _, line := range strings.Split(content, "\n") {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}

		cmd, args := tokenizeLine(line)

		switch cmd {
		case "aid":
			if len(args) >= 1 {
				id := args[0]
				vaultUUID = &id
			}
		case "cmm":
			if len(args) >= 1 && vaultName == "" {
				vaultName = args[0]
			}
		case "fmt":
			// skip
		case "cgr":
			if len(args) >= 2 {
				groupID := args[1]
				var parentID *string
				if args[0] != "0" {
					parentID = &args[0]
				}
				groups[groupID] = &ButtercupGroup{
					ID:       groupID,
					Name:     "",
					ParentID: parentID,
				}
				groupOrder = append(groupOrder, groupID)
			}
		case "tgr":
			if len(args) >= 2 {
				if g, ok := groups[args[0]]; ok {
					g.Name = args[1]
				}
			}
		case "dgr":
			if len(args) >= 1 {
				deletedGroups = append(deletedGroups, args[0])
			}
		case "mgr":
			if len(args) >= 2 {
				if g, ok := groups[args[0]]; ok {
					if args[1] == "0" {
						g.ParentID = nil
					} else {
						g.ParentID = &args[1]
					}
				}
			}
		case "cen":
			if len(args) >= 2 {
				entryID := args[1]
				entries[entryID] = &ButtercupEntry{
					ID:      entryID,
					GroupID: &args[0],
				}
				entryOrder = append(entryOrder, entryID)
			}
		case "den":
			if len(args) >= 1 {
				deletedEntries = append(deletedEntries, args[0])
			}
		case "sep":
			if len(args) >= 3 {
				if e, ok := entries[args[0]]; ok {
					switch args[1] {
					case "title":
						e.Title = args[2]
					case "username":
						e.Username = args[2]
					case "password":
						e.Password = args[2]
					default:
						e.Fields = append(e.Fields, ButtercupCustomField{
							ID:        args[1],
							Label:     args[1],
							FieldType: "text",
							Value:     args[2],
						})
					}
				}
			} else if len(args) == 2 {
				if e, ok := entries[args[0]]; ok {
					switch args[1] {
					case "title":
						e.Title = ""
					case "username":
						e.Username = ""
					case "password":
						e.Password = ""
					}
				}
			}
		case "sem":
			if len(args) >= 3 {
				if e, ok := entries[args[0]]; ok {
					e.Fields = append(e.Fields, ButtercupCustomField{
						ID:        args[1],
						Label:     args[1],
						FieldType: "text",
						Value:     args[2],
					})
				}
			}
		case "pad":
			// no-op
		case "sea":
			if len(args) >= 3 {
				if e, ok := entries[args[0]]; ok {
					switch args[1] {
					case "title":
						e.Title = args[2]
					case "username":
						e.Username = args[2]
					case "password":
						e.Password = args[2]
					default:
						e.Fields = append(e.Fields, ButtercupCustomField{
							ID:        args[1],
							Label:     args[1],
							FieldType: "text",
							Value:     args[2],
						})
					}
				}
			}
		}
	}

	// Remove deleted groups and entries
	for _, id := range deletedGroups {
		delete(groups, id)
	}
	for _, id := range deletedEntries {
		delete(entries, id)
	}

	// Build final slices preserving insertion order
	var finalGroups []ButtercupGroup
	for _, id := range groupOrder {
		if g, ok := groups[id]; ok {
			finalGroups = append(finalGroups, *g)
		}
	}

	var finalEntries []ButtercupEntry
	for _, id := range entryOrder {
		if e, ok := entries[id]; ok {
			finalEntries = append(finalEntries, *e)
		}
	}

	// Partition trash groups
	var trashGroups, normalGroups []ButtercupGroup
	for _, g := range finalGroups {
		if g.Name == "Trash" {
			trashGroups = append(trashGroups, g)
		} else {
			normalGroups = append(normalGroups, g)
		}
	}

	trashGroupIDs := map[string]bool{}
	for _, g := range trashGroups {
		trashGroupIDs[g.ID] = true
	}

	var trashEntries, normalEntries []ButtercupEntry
	for _, e := range finalEntries {
		if e.GroupID != nil && trashGroupIDs[*e.GroupID] {
			trashEntries = append(trashEntries, e)
		} else {
			normalEntries = append(normalEntries, e)
		}
	}

	return &ButtercupVault{
		Name:    vaultName,
		UUID:    vaultUUID,
		Groups:  normalGroups,
		Entries: normalEntries,
		Trash: ButtercupTrash{
			Groups:  trashGroups,
			Entries: trashEntries,
		},
	}, nil
}
