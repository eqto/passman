use passman_core::{TrashGroup, VaultEntry, VaultFile};

#[derive(Clone, serde::Serialize)]
pub struct VaultFileDTO {
    pub path: String,
    pub name: String,
    pub groups: Vec<String>,
    pub tags: Vec<String>,
    pub entries: Vec<VaultEntry>,
    pub trash: Vec<TrashGroup>,
}

pub fn vault_to_dto(vault: &VaultFile) -> VaultFileDTO {
    VaultFileDTO {
        path: vault.path.clone(),
        name: vault.payload.vault_metadata.name.clone(),
        groups: vault.payload.groups.clone(),
        tags: vault.payload.tags.clone(),
        entries: vault.payload.entries.clone(),
        trash: vault.payload.trash.clone(),
    }
}
