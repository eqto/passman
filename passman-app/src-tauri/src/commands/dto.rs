use passman_core::{Group, Trash, VaultEntry, VaultFile};

#[derive(Clone, serde::Serialize)]
pub struct VaultFileDTO {
    pub path: String,
    pub name: String,
    pub groups: Vec<Group>,
    pub tags: Vec<String>,
    pub entries: Vec<VaultEntry>,
    pub trash: Trash,
}

pub fn vault_to_dto(vault: &VaultFile) -> VaultFileDTO {
    VaultFileDTO {
        path: vault.path.clone(),
        name: vault.payload.name.clone(),
        groups: vault.payload.groups.clone(),
        tags: vault.payload.tags.clone(),
        entries: vault.payload.entries.clone(),
        trash: vault.payload.trash.clone(),
    }
}
