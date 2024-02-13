use colorful::{Color, Colorful};
use core::panic;
use sevenz_rust::*;
use std::{
    collections::HashMap,
    ffi::OsString,
    io::{prelude::*, stdin, stdout, Read},
    path::Path,
};
use steamlocate::{SteamApp, SteamDir};

const ASSETTO_CORSA_APP_ID: u32 = 244210;
const FOLDER_NAME: &str = env!("FOLDER_NAME");

struct Modification<'a> {
    archive_name: String,
    modification_type_folder_name: Option<&'a str>,
}

impl Modification<'static> {
    fn new() -> Modification<'static> {
        let folder_name = FOLDER_NAME.to_string();
        let modification_type_folder_name: Option<&str> = match folder_name.clone().as_str() {
            "zs_gokart125" => Some("cars"),
            "rs_bydgoszcz_rotax" => Some("tracks"),
            "rs_torun_rotax" => Some("tracks"),
            "rs_autodrom_slomczyn_rotax" => Some("tracks"),
            "rs_tor_poznanl" => Some("tracks"),
            _ => None,
        };
        return Modification {
            modification_type_folder_name,
            archive_name: format!("{}.7z", folder_name),
        };
    }
}

fn pause() {
    let mut stdin = stdin();
    let mut stdout = stdout();
    write!(stdout, "Wciśnij Enter, żeby opuścić kreator instalacji...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    println!(
        "{}",
        "Szukanie aplikacji steam na komputerze...".color(Color::Cyan)
    );
    let steam_directory_option: Option<SteamDir> = SteamDir::locate();
    if steam_directory_option.is_none() {
        println!(
            "{}",
            "Nie znaleziono aplikacji steam na komputerze".color(Color::Red)
        );
        pause();
        panic!("");
    }
    let mut steam_directory: SteamDir = steam_directory_option.unwrap();
    println!(
        "{}",
        "Szukanie gry Assetto Corsa na komputerze...".color(Color::Cyan)
    );
    let apps: &HashMap<u32, Option<SteamApp>> = steam_directory.apps();
    let assetto_corsa: Option<&Option<SteamApp>> = apps.get(&ASSETTO_CORSA_APP_ID);
    if assetto_corsa.is_none() {
        println!(
            "{}",
            "Nie znaleziono gry Assetto Corsa na komputerze".color(Color::Red)
        );
        pause();
        panic!("");
    }
    let assetto_corsa: &Option<SteamApp> = assetto_corsa.unwrap();
    if assetto_corsa.is_none() {
        println!(
            "{}",
            "Nie znaleziono gry Assetto Corsa na twoim komputerze".color(Color::Red)
        );
        pause();
        panic!("");
    }
    let assetto_corsa: &SteamApp = assetto_corsa.as_ref().unwrap();
    let destination_path_result: Result<String, OsString> =
        (&assetto_corsa.path).clone().into_os_string().into_string();
    if destination_path_result.is_err() {
        println!(
            "{}",
            "Wystąpił problem podczas przetwarzania ścieżki do gry Assetto Corsa".color(Color::Red)
        );
        pause();
        panic!("");
    }
    let destination_path: String = destination_path_result.unwrap();
    println!("{}", "Instalacja modyfikacji...".color(Color::Cyan));
    let addon: Modification<'static> = Modification::new();
    if !Path::new(".").join(addon.archive_name.clone()).exists() {
        println!(
            "{} {}{}",
            "Wygląda na to, że w katalogu brakuje pliku".color(Color::Red),
            addon.archive_name.color(Color::Red),
            ". Upewnij się, że wszystkie pliki zostały prawidłowo pobrane.".color(Color::Red)
        );
        pause();
        panic!("");
    }
    if addon.modification_type_folder_name.is_none() {
        println!(
            "{}",
            "Nie odnaleziono folderu odpowiadającego modyfikacji".color(Color::Red)
        );
        pause();
        panic!("");
    }
    let modification_type_folder_name: &str = addon.modification_type_folder_name.unwrap();
    let destination_path_result: Result<String, OsString> = Path::new(&destination_path)
        .join("content")
        .join(modification_type_folder_name)
        .into_os_string()
        .into_string();
    if destination_path_result.is_err() {
        println!(
            "{}",
            "Wystąpił problem podczas ustalania ścieżki końcowej".color(Color::Red)
        );
        pause();
        panic!("");
    }
    let destination_path: String = destination_path_result.unwrap();
    let decompress_result: Result<(), Error> =
        decompress_file(addon.archive_name, destination_path);
    match decompress_result {
        Ok(_) => {
            println!(
                "{}",
                "Zainstalowano modyfikacje pomyślnie".color(Color::Green)
            );
            pause();
        }
        Err(_) => {
            println!(
                "{}",
                "Wystąpił nieoczekiwany błąd podczas instalacji".color(Color::Red)
            );
            pause();
            panic!("");
        }
    }
}
