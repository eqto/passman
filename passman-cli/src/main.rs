use clap::{Parser, Subcommand};
use passman_core::{
    create_vault_file, save_vault_file, ButtercupError, ButtercupVault, VaultEntry, VaultFile,
    VaultMetadata, PAYLOAD_FORMAT_VERSION,
};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;
use thiserror::Error;

#[derive(Parser)]
#[command(name = "passman-cli")]
#[command(about = "Passman CLI import/export tools")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new empty Passman vault
    Create {
        /// Output path for the .pmv file
        output: String,
        /// Vault name
        #[arg(short, long)]
        name: String,
    },
    /// Import a JSON file (e.g. exported from Buttercup) into a .pmv vault
    Import {
        /// Path to the JSON file to import
        input: String,
        /// Output path for the .pmv file
        output: String,
        /// Vault name
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Export a Buttercup .bcup vault to a Passman JSON file
    ExportButtercup {
        /// Path to the .bcup file
        input: String,
        /// Output path for the JSON file
        output: String,
    },
    /// Import a Buttercup .bcup vault directly into a .pmv vault
    ImportButtercup {
        /// Path to the .bcup file
        input: String,
        /// Output path for the .pmv file
        output: String,
        /// Vault name
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Convert a Buttercup .bcup vault to a .pmv vault
    Convert {
        /// Path to the .bcup file
        input: String,
        /// Output path for the .pmv file
        output: String,
    },
    /// Extract a .pmv vault into a directory containing header.json and payload.json
    Extract {
        /// Path to the .pmv file to extract
        input: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
struct ImportJson {
    #[serde(default = "default_vault_name")]
    name: String,
    #[serde(default)]
    groups: Vec<String>,
    #[serde(default)]
    entries: Vec<ImportEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ImportEntry {
    id: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    title: String,
    #[serde(default)]
    username: String,
    #[serde(default)]
    password: String,
    #[serde(default)]
    url: String,
    #[serde(default)]
    notes: String,
}

fn default_vault_name() -> String {
    "Imported Vault".to_string()
}

#[derive(Debug, Error)]
enum CliError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Vault(#[from] passman_core::VaultError),
    #[error("buttercup error: {0}")]
    Buttercup(#[from] ButtercupError),
    #[error("password prompt error: {0}")]
    PasswordPrompt(String),
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), CliError> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Create { output, name } => {
            let password = prompt_password("New vault password: ")?;
            create_vault_file(&output, &name, &password)?;
            println!("Created vault: {output}");
        }
        Commands::Import {
            input,
            output,
            name,
        } => {
            let json = std::fs::read_to_string(&input)?;
            let mut imported: ImportJson = serde_json::from_str(&json)?;
            if let Some(vault_name) = name {
                imported.name = vault_name;
            }
            let vault_name = imported.name.clone();
            let password = prompt_password("New vault password: ")?;
            let vault = create_and_save_vault(&output, &vault_name, &password, imported)?;
            println!(
                "Imported {} entries into {output}",
                vault.payload.entries.len()
            );
        }
        Commands::ExportButtercup { input, output } => {
            let password = prompt_password_buttercup("Buttercup master password: ")?;
            let vault = passman_core::decrypt_buttercup_file(&input, &password)?;
            let import = ImportJson::from(vault);
            std::fs::write(&output, serde_json::to_string_pretty(&import)?)?;
            println!("Exported {} entries to {output}", import.entries.len());
        }
        Commands::ImportButtercup {
            input,
            output,
            name,
        } => {
            let bcup_password = prompt_password_buttercup("Buttercup master password: ")?;
            let pmv_password = prompt_password("New vault password: ")?;

            let bcup = passman_core::decrypt_buttercup_file(&input, &bcup_password)?;
            let mut import = ImportJson::from(bcup);
            let vault_name = name.unwrap_or_else(|| derive_vault_name(&import.name, &input));
            import.name = vault_name.clone();

            let vault = create_and_save_vault(&output, &vault_name, &pmv_password, import)?;
            println!(
                "Imported {} entries from {input} into {output}",
                vault.payload.entries.len()
            );
        }
        Commands::Convert { input, output } => {
            let password = resolve_convert_password()?;

            let bcup = passman_core::decrypt_buttercup_file(&input, &password)?;
            let mut import = ImportJson::from(bcup);
            let vault_name = derive_vault_name(&import.name, &input);
            import.name = vault_name.clone();

            let vault = create_and_save_vault(&output, &vault_name, &password, import)?;
            println!(
                "Converted {} entries from {input} into {output}",
                vault.payload.entries.len()
            );
        }
        Commands::Extract { input } => {
            let password = prompt_password("Vault password: ")?;
            let vault = passman_core::open_vault_file(&input, &password)?;
            let dir_name = Path::new(&input)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("extracted")
                .to_string();
            std::fs::create_dir_all(&dir_name)?;
            let header_path = Path::new(&dir_name).join("header.json");
            let payload_path = Path::new(&dir_name).join("payload.json");
            std::fs::write(&header_path, serde_json::to_string_pretty(&vault.header)?)?;
            std::fs::write(&payload_path, serde_json::to_string_pretty(&vault.payload)?)?;
            println!("Extracted vault to {dir_name}/");
        }
    }
    Ok(())
}

fn build_payload(vault: &mut VaultFile, imported: ImportJson) {
    let now = chrono::Utc::now();
    vault.payload.vault_metadata = VaultMetadata {
        name: imported.name,
        created_at: now,
        updated_at: now,
        format_version: PAYLOAD_FORMAT_VERSION,
    };

    vault.payload.groups = imported
        .groups
        .into_iter()
        .map(|g| g.trim().to_string())
        .filter(|g| !g.is_empty())
        .collect();

    vault.payload.entries = imported
        .entries
        .into_iter()
        .map(|e| VaultEntry {
            id: e.id,
            title: e.title,
            username: e.username,
            password: e.password,
            url: e.url,
            notes: e.notes,
            tags: e.tags,
            created_at: now,
            updated_at: now,
        })
        .collect();
}

fn create_and_save_vault(
    output: &str,
    name: &str,
    password: &str,
    payload: ImportJson,
) -> Result<VaultFile, CliError> {
    let mut vault = create_vault_file(output, name, password)?;
    build_payload(&mut vault, payload);
    save_vault_file(&vault, password)?;
    Ok(vault)
}

fn resolve_convert_password() -> Result<String, CliError> {
    if let Ok(p) = std::env::var("PASSMAN_PASSWORD") {
        Ok(p)
    } else if let Ok(p) = std::env::var("BCUP_PASSWORD") {
        Ok(p)
    } else {
        rpassword::prompt_password("Password: ")
            .map_err(|e| CliError::PasswordPrompt(e.to_string()))
    }
}

fn derive_vault_name(source_name: &str, input_path: &str) -> String {
    if !source_name.is_empty() {
        source_name.to_string()
    } else {
        Path::new(input_path)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "Imported Buttercup Vault".to_string())
    }
}

fn prompt_password(prompt: &str) -> Result<String, CliError> {
    if let Ok(password) = std::env::var("PASSMAN_PASSWORD") {
        return Ok(password);
    }
    rpassword::prompt_password(prompt).map_err(|e| CliError::PasswordPrompt(e.to_string()))
}

fn prompt_password_buttercup(prompt: &str) -> Result<String, CliError> {
    if let Ok(password) = std::env::var("BCUP_PASSWORD") {
        return Ok(password);
    }
    rpassword::prompt_password(prompt).map_err(|e| CliError::PasswordPrompt(e.to_string()))
}

impl From<ButtercupVault> for ImportJson {
    fn from(vault: ButtercupVault) -> Self {
        ImportJson {
            name: vault.name,
            groups: vault.groups,
            entries: vault
                .entries
                .into_iter()
                .map(|e| ImportEntry {
                    id: e.id,
                    tags: e.tags,
                    title: e.title,
                    username: e.username,
                    password: e.password,
                    url: e.url,
                    notes: e.notes,
                })
                .collect(),
        }
    }
}
