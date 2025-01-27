use std;
use std::env;
use std::fs;
use std::path::Path;

#[cfg(target_os = "windows")]
use {winreg::enums::*, winreg::RegKey};

fn get_steam_path() -> String {
    #[cfg(target_os = "windows")]
    return {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let steam_registry_info = hklm
            .open_subkey("SOFTWARE\\WOW6432Node\\Valve\\Steam")
            .expect("Can't find Steam in registry.");
        let steam_path: String = steam_registry_info
            .get_value("InstallPath")
            .expect("InstallPath is not readable.");
        steam_path
    };
    #[cfg(target_os = "linux")]
    return {
        let mut steam_path = std::env::var("HOME").expect("Impossible de trouver le dossier home.");
        steam_path.push_str("/.local/share/Steam");
        if Path::exists(Path::new(&steam_path)) {
            steam_path
        } else {
            todo!("Mettre une erreur");
        }
    };
}

#[tauri::command]
fn get_steam_installed_apps() -> Vec<String> {
    // Get the Steam path from the registry
    let steam_path = get_steam_path();

    let library_folder_content = fs::read_to_string(
        Path::new(&steam_path)
            .join("steamapps")
            .join("libraryfolders.vdf"),
    )
    .expect("Can't access library folder file.");

    let library_folder_parsed = vdf_parser::parse_vdf_text(&library_folder_content)
        .expect("Error handling library folder VDF file.");

    let mut games: Vec<String> = Vec::new();

    match library_folder_parsed.value {
        vdf_parser::VdfValue::Block(library_folder_block) => {
            for library in library_folder_block.values() {
                match &library.value {
                    vdf_parser::VdfValue::Block(library_block) => match library_block.get("apps") {
                        Some(app) => match &app.value {
                            vdf_parser::VdfValue::Block(game_id) => {
                                for game in game_id.keys() {
                                    games.push(game.to_string());
                                }
                            }
                            vdf_parser::VdfValue::String(_) => {
                                panic!("Library VDF file is invalid.");
                            }
                        },
                        None => panic!("Library VDF file is invalid."),
                    },
                    vdf_parser::VdfValue::String(_) => {
                        panic!("Library VDF file is invalid.");
                    }
                }
            }
        }
        vdf_parser::VdfValue::String(_) => {
            panic!("Library VDF file is invalid.");
        }
    }

    games
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_steam_installed_apps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
