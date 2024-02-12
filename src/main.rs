use colorful::{Color, Colorful};
use core::panic;
use sevenz_rust::*;
use std::io::prelude::*;
use std::io::{stdin, stdout};
use std::{collections::HashMap, io::Read};
use steamlocate::{SteamApp, SteamDir};

const ASSETTO_CORSA_APP_ID: u32 = 244210;

fn pause() {
    let mut stdin = stdin();
    let mut stdout = stdout();
    write!(stdout, "Wciśnij Enter, żeby opuścić kreator instalacji...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    let addon_folder_name: &'static str = "123";
    println!(
        "{}",
        "Szukanie aplikacji steam na komputerze...".color(Color::Cyan)
    );
    let steam_directory_option = SteamDir::locate();
    if steam_directory_option.is_none() {
        println!(
            "{}",
            "Nie znaleziono aplikacji steam na komputerze".color(Color::Red)
        );
        pause();
        panic!("");
    }
    let mut steam_directory = steam_directory_option.unwrap();
    println!(
        "{}",
        "Szukanie gry Assetto Corsa na komputerze...".color(Color::Cyan)
    );
    let apps: &HashMap<u32, Option<SteamApp>> = steam_directory.apps();
    let assetto_corsa = apps.get(&ASSETTO_CORSA_APP_ID);
    if assetto_corsa.is_none() {
        println!(
            "{}",
            "Nie znaleziono gry Assetto Corsa na komputerze".color(Color::Red)
        );
        pause();
        panic!("");
    }
    let assetto_corsa = assetto_corsa.unwrap();
    if assetto_corsa.is_none() {
        println!(
            "{}",
            "Nie znaleziono gry Assetto Corsa na twoim komputerze".color(Color::Red)
        );
        pause();
        panic!("");
    }
    let assetto_corsa = assetto_corsa.as_ref().unwrap();
    let destination_path = (&assetto_corsa.path)
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    let destination_path = format!("{destination_path}\\content\\cars\\{addon_folder_name}");
    println!("{}", "Instalacja modyfikacji...".color(Color::Cyan));
    let decompress_result = decompress_file(format!("{addon_folder_name}.7z"), destination_path);
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
