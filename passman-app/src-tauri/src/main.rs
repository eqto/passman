#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;

use commands::*;
use tauri::Manager;

fn main() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init());

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(tauri_plugin_mcp_bridge::init());
    }

    builder
        .setup(|app| {
            app.manage(AppState::new(app.handle().clone()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_vaults,
            create_vault,
            open_vault,
            register_and_open_vault,
            close_vault,
            delete_vault,
            rename_vault,
            reorder_vaults,
            convert_buttercup_vault,
            list_groups,
            add_group,
            delete_group,
            reorder_groups,
            merge_groups,
            move_group_to_vault,
            copy_group_to_vault,
            add_tag,
            list_entries,
            add_entry,
            update_entry,
            delete_entry,
            restore_trash_entry,
            delete_trash_entry,
            list_trash,
            generate_password,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
