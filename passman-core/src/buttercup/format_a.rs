use super::types::{
    ButtercupCustomField, ButtercupEntry, ButtercupGroup, ButtercupTrash, ButtercupVault,
};
use std::collections::HashMap;

/// Parse a quoted string from the input, handling escape sequences.
/// Returns the parsed string and the number of bytes consumed (including quotes).
fn parse_quoted_string(input: &str) -> Option<(String, usize)> {
    let bytes = input.as_bytes();
    if bytes.is_empty() || bytes[0] != b'"' {
        return None;
    }

    let mut result = String::new();
    let mut i = 1;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => {
                if i + 1 < bytes.len() {
                    // Handle escape sequences
                    match bytes[i + 1] {
                        b'"' => result.push('"'),
                        b'\\' => result.push('\\'),
                        b'n' => result.push('\n'),
                        b'r' => result.push('\r'),
                        b't' => result.push('\t'),
                        // Unicode escape \uXXXX
                        b'u' => {
                            if i + 5 < bytes.len() {
                                let hex = &input[i + 2..i + 6];
                                if let Ok(code) = u32::from_str_radix(hex, 16) {
                                    if let Some(ch) = char::from_u32(code) {
                                        result.push(ch);
                                    }
                                }
                                i += 6;
                                continue;
                            }
                        }
                        // Other escaped characters - just push the char
                        c => result.push(c as char),
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            }
            b'"' => {
                return Some((result, i + 1));
            }
            c => {
                result.push(c as char);
                i += 1;
            }
        }
    }

    None
}

/// Tokenize a format A command line into the command name and its arguments.
fn tokenize_line(line: &str) -> (String, Vec<String>) {
    let line = line.trim();
    if line.is_empty() {
        return (String::new(), Vec::new());
    }

    let mut tokens = Vec::new();
    let mut i = 0;
    let bytes = line.as_bytes();

    // First token is the command (non-quoted)
    while i < bytes.len() && bytes[i] != b' ' {
        i += 1;
    }
    let cmd = line[..i].to_string();

    // Parse remaining tokens
    while i < bytes.len() {
        // Skip whitespace
        while i < bytes.len() && bytes[i] == b' ' {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }

        if bytes[i] == b'"' {
            if let Some((s, consumed)) = parse_quoted_string(&line[i..]) {
                tokens.push(s);
                i += consumed;
            } else {
                // Malformed quote, take rest as-is
                tokens.push(line[i..].to_string());
                break;
            }
        } else {
            // Non-quoted token
            let start = i;
            while i < bytes.len() && bytes[i] != b' ' {
                i += 1;
            }
            tokens.push(line[start..i].to_string());
        }
    }

    (cmd, tokens)
}

/// Parse format A delta commands and build a ButtercupVault.
pub fn parse_format_a(content: &str) -> Result<ButtercupVault, super::ButtercupError> {
    let mut vault_name = String::new();
    let mut vault_uuid: Option<String> = None;
    let mut groups: HashMap<String, ButtercupGroup> = HashMap::new();
    let mut group_order: Vec<String> = Vec::new();
    let mut entries: HashMap<String, ButtercupEntry> = HashMap::new();
    let mut entry_order: Vec<String> = Vec::new();
    let mut deleted_groups: Vec<String> = Vec::new();
    let mut deleted_entries: Vec<String> = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (cmd, args) = tokenize_line(line);

        match cmd.as_str() {
            "aid" => {
                if let Some(id) = args.first() {
                    vault_uuid = Some(id.clone());
                }
            }
            "cmm" => {
                // Comment - sometimes contains vault name
                if let Some(comment) = args.first() {
                    if vault_name.is_empty() {
                        vault_name = comment.clone();
                    }
                }
            }
            "fmt" => {
                // Format identifier - skip
            }
            "cgr" => {
                // cgr <parent_id> <group_id>
                if args.len() >= 2 {
                    let group_id = args[1].clone();
                    let parent_id = if args[0] == "0" {
                        None
                    } else {
                        Some(args[0].clone())
                    };
                    groups.insert(
                        group_id.clone(),
                        ButtercupGroup {
                            id: group_id.clone(),
                            name: String::new(),
                            parent_id,
                        },
                    );
                    group_order.push(group_id);
                }
            }
            "tgr" => {
                // tgr <group_id> "title"
                if args.len() >= 2 {
                    if let Some(group) = groups.get_mut(&args[0]) {
                        group.name = args[1].clone();
                    }
                }
            }
            "dgr" => {
                // dgr <group_id>
                if let Some(id) = args.first() {
                    deleted_groups.push(id.clone());
                }
            }
            "mgr" => {
                // mgr <group_id> <new_parent_id>
                if args.len() >= 2 {
                    if let Some(group) = groups.get_mut(&args[0]) {
                        group.parent_id = if args[1] == "0" {
                            None
                        } else {
                            Some(args[1].clone())
                        };
                    }
                }
            }
            "cen" => {
                // cen <group_id> <entry_id>
                if args.len() >= 2 {
                    let entry_id = args[1].clone();
                    entries.insert(
                        entry_id.clone(),
                        ButtercupEntry {
                            id: entry_id.clone(),
                            group_id: Some(args[0].clone()),
                            title: String::new(),
                            username: String::new(),
                            password: String::new(),
                            fields: Vec::new(),
                            deleted_at: None,
                            history: Vec::new(),
                        },
                    );
                    entry_order.push(entry_id);
                }
            }
            "den" => {
                // den <entry_id>
                if let Some(id) = args.first() {
                    deleted_entries.push(id.clone());
                }
            }
            "sep" => {
                // sep <entry_id> <property> "value"
                if args.len() >= 3 {
                    if let Some(entry) = entries.get_mut(&args[0]) {
                        match args[1].as_str() {
                            "title" => entry.title = args[2].clone(),
                            "username" => entry.username = args[2].clone(),
                            "password" => entry.password = args[2].clone(),
                            _ => {
                                entry.fields.push(ButtercupCustomField {
                                    id: args[1].clone(),
                                    label: args[1].clone(),
                                    field_type: "text".to_string(),
                                    value: args[2].clone(),
                                });
                            }
                        }
                    }
                } else if args.len() == 2 {
                    // sep <entry_id> <property> (clearing the property)
                    if let Some(entry) = entries.get_mut(&args[0]) {
                        match args[1].as_str() {
                            "title" => entry.title.clear(),
                            "username" => entry.username.clear(),
                            "password" => entry.password.clear(),
                            _ => {}
                        }
                    }
                }
            }
            "sem" => {
                // sem <entry_id> <meta_key> "meta_value"
                if args.len() >= 3 {
                    if let Some(entry) = entries.get_mut(&args[0]) {
                        entry.fields.push(ButtercupCustomField {
                            id: args[1].clone(),
                            label: args[1].clone(),
                            field_type: "text".to_string(),
                            value: args[2].clone(),
                        });
                    }
                }
            }
            "pad" => {
                // Padding - no-op
            }
            "sea" => {
                // Set entry attribute (same as sep in some versions)
                if args.len() >= 3 {
                    if let Some(entry) = entries.get_mut(&args[0]) {
                        match args[1].as_str() {
                            "title" => entry.title = args[2].clone(),
                            "username" => entry.username = args[2].clone(),
                            "password" => entry.password = args[2].clone(),
                            _ => {
                                entry.fields.push(ButtercupCustomField {
                                    id: args[1].clone(),
                                    label: args[1].clone(),
                                    field_type: "text".to_string(),
                                    value: args[2].clone(),
                                });
                            }
                        }
                    }
                }
            }
            _ => {
                // Unknown command - skip
            }
        }
    }

    // Remove deleted groups and entries
    for id in &deleted_groups {
        groups.remove(id);
    }
    for id in &deleted_entries {
        entries.remove(id);
    }

    // Build final vectors preserving insertion order
    let final_groups: Vec<ButtercupGroup> = group_order
        .into_iter()
        .filter_map(|id| groups.get(&id).cloned())
        .collect();

    let final_entries: Vec<ButtercupEntry> = entry_order
        .into_iter()
        .filter_map(|id| entries.get(&id).cloned())
        .collect();

    // Identify trash groups (groups with "Trash" name or in trash)
    let (trash_groups, normal_groups): (Vec<ButtercupGroup>, Vec<ButtercupGroup>) = final_groups
        .into_iter()
        .partition(|g| g.name == "Trash");

    let (trash_entries, normal_entries): (Vec<ButtercupEntry>, Vec<ButtercupEntry>) =
        final_entries
            .into_iter()
            .partition(|e| {
                if let Some(gid) = &e.group_id {
                    trash_groups.iter().any(|g| &g.id == gid)
                } else {
                    false
                }
            });

    Ok(ButtercupVault {
        name: vault_name,
        uuid: vault_uuid,
        groups: normal_groups,
        entries: normal_entries,
        trash: ButtercupTrash {
            groups: trash_groups,
            entries: trash_entries,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_format_a() {
        let content = r#"aid test-vault-id
cmm "Test vault"
fmt "buttercup/a"
cgr 0 group-1
tgr group-1 "General"
cen group-1 entry-1
sep entry-1 title "My Entry"
sep entry-1 username "user@test.com"
sep entry-1 password "secret123"
sem entry-1 url "https://example.com"
pad pad-1"#;

        let vault = parse_format_a(content).unwrap();
        assert_eq!(vault.name, "Test vault");
        assert_eq!(vault.uuid, Some("test-vault-id".to_string()));
        assert_eq!(vault.groups.len(), 1);
        assert_eq!(vault.groups[0].name, "General");
        assert_eq!(vault.entries.len(), 1);
        assert_eq!(vault.entries[0].title, "My Entry");
        assert_eq!(vault.entries[0].username, "user@test.com");
        assert_eq!(vault.entries[0].password, "secret123");
        assert_eq!(vault.entries[0].fields.len(), 1);
        assert_eq!(vault.entries[0].fields[0].label, "url");
    }

    #[test]
    fn test_parse_with_deleted() {
        let content = r#"aid test-vault-id
cmm "Test"
cgr 0 group-1
tgr group-1 "Main"
cgr 0 group-2
tgr group-2 "ToDelete"
dgr group-2
cen group-1 entry-1
sep entry-1 title "Keep"
cen group-1 entry-2
sep entry-2 title "Delete"
den entry-2"#;

        let vault = parse_format_a(content).unwrap();
        assert_eq!(vault.groups.len(), 1);
        assert_eq!(vault.groups[0].name, "Main");
        assert_eq!(vault.entries.len(), 1);
        assert_eq!(vault.entries[0].title, "Keep");
    }

    #[test]
    fn test_parse_quoted_string_escapes() {
        let input = r#""hello \"world\" \\ test""#;
        let (s, _) = parse_quoted_string(input).unwrap();
        assert_eq!(s, r#"hello "world" \ test"#);
    }

    #[test]
    fn test_parse_trash_group() {
        let content = r#"aid test-vault-id
cmm "Test"
cgr 0 group-1
tgr group-1 "General"
cgr 0 group-trash
tgr group-trash "Trash"
cen group-1 entry-1
sep entry-1 title "Normal"
cen group-trash entry-2
sep entry-2 title "Trashed""#;

        let vault = parse_format_a(content).unwrap();
        assert_eq!(vault.groups.len(), 1);
        assert_eq!(vault.groups[0].name, "General");
        assert_eq!(vault.entries.len(), 1);
        assert_eq!(vault.entries[0].title, "Normal");
        assert_eq!(vault.trash.groups.len(), 1);
        assert_eq!(vault.trash.entries.len(), 1);
        assert_eq!(vault.trash.entries[0].title, "Trashed");
    }
}
