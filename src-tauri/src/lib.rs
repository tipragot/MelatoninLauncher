use winreg::enums::*;
use winreg::RegKey;
use vdf_parser::*;
use std::env;
use std::fs;
use std;
use std::path::Path;

#[tauri::command]
fn get_steam_installed_apps() -> Vec<String> {
    // Get the Steam path from the registry 
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let steam_registry_info = hklm.open_subkey("SOFTWARE\\WOW6432Node\\Valve\\Steam").expect("Can't find Steam in registry.");
    let steam_path: String = steam_registry_info.get_value("InstallPath").expect("InstallPath is not readable.");

    let library_folder_content = fs::read_to_string(Path::new(&steam_path).join("steamapps").join("libraryfolders.vdf"))
        .expect("Can't access library folder file.");

    let library_folder_parsed = vdf_parser::parse_vdf_text(&library_folder_content).expect("Error handling library folder VDF file");
    
    match library_folder_parsed.value {
        vdf_parser::VdfValue::Block(block) => {
            todo!("Iterate libraries")
        }
        vdf_parser::VdfValue::String(_) => {todo!("Handle error")}
    }

    let mut games: Vec<String> = Vec::new();

    games.push("Hello".to_string());

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
