// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{io, cmp::Ordering, char::from_digit};
use rand::Rng;

// Very helpful post here: https://stackoverflow.com/questions/53971954/why-cant-a-range-of-char-be-collected
//33..=47 are special chars
//48..=57 are numbers
//65..=90 are uppercase
//97..=122 are lowercase

#[tauri::command]
fn greet(len: u8, lwr: bool, upr: bool, numbr: bool, specl: bool) -> char {
  let rando_number: u8 = rand::thread_rng().gen_range(33..=122);
  let numbo: char = rando_number as char;
  numbo
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
