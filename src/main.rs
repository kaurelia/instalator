use std::collections::HashMap;
use sevenz_rust::*;
use steamlocate::{SteamDir, SteamApp};

const ASSETTO_CORSA_APP_ID: u32 = 244210;

fn main() {
    let addon_folder_name: &'static str = "123";
    let mut steam_directory = SteamDir::locate().unwrap();
    let apps: &HashMap<u32, Option<SteamApp>> = steam_directory.apps();
    let assetto_corsa = apps.get(&ASSETTO_CORSA_APP_ID);
    if assetto_corsa.is_none() {
        panic!("Asseto corsa not found");
    }
    let assetto_corsa = assetto_corsa.unwrap();
    if assetto_corsa.is_none() {
        panic!("Asseto corsa not found");
    }
    let assetto_corsa = assetto_corsa.as_ref().unwrap();
    let destination_path = (&assetto_corsa.path).clone().into_os_string().into_string().unwrap();
    let destination_path = format!("{destination_path}\\content\\cars\\{addon_folder_name}");
    println!("{:?}", destination_path);
    let result = decompress_file(format!("{addon_folder_name}.7z"), destination_path);
    println!("{result:#?}");
    //println!("{:?}", apps);
    //println!("Hello, world!");
}
