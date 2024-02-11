// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{io, cmp::Ordering, char::from_digit};
use rand::Rng;

#[tauri::command]
fn genPass(
  value: u8,
  lwrcase:bool,
  uprcase:bool,
  nmbrcase:bool,
  spclcase:bool 
  ) -> String {
    let mut options = Vec::new();
    match lwrcase {
      true => options.push('a'),
      false => {}
    }
    match uprcase {
      true => options.push('b'),
      false => {}
    }
    match nmbrcase {
      true => options.push('c'),
      false => {}
    }
    match spclcase {
      true => options.push('d'),
      false => {}
    }

    let mut answer = String::new();
    for _x in 0..value {
      let rand_option: usize = rand::thread_rng().gen_range(0..options.len());
      match options[rand_option] {
        'a' => {
          let rando_number: u8 = rand::thread_rng().gen_range(97..=122);
          let numbo: char = rando_number as char;
          answer.push(numbo);
        },
        'b' => {
          let rando_number: u8 = rand::thread_rng().gen_range(65..=90);
          let numbo: char = rando_number as char;
          answer.push(numbo);
        },
        'c' => {
          let rando_number: u8 = rand::thread_rng().gen_range(48..=57);
          let numbo: char = rando_number as char;
          answer.push(numbo);
        },
        'd' => {
          let rando_number: u8 = rand::thread_rng().gen_range(33..=47);
          let numbo: char = rando_number as char;
          answer.push(numbo);
        },
        _ => panic!()
      }
    }
    answer
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![genPass])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


// Very helpful post here: https://stackoverflow.com/questions/53971954/why-cant-a-range-of-char-be-collected
//33..=47 are special chars
//48..=57 are numbers
//65..=90 are uppercase
//97..=122 are lowercase

/*
static LWRCASE: [char; 26] = ['a', 
  'b', 'c', 'd', 'e', 'f', 
  'g', 'h', 'i', 'j', 'k', 
  'l', 'm', 'n', 'o', 'p', 
  'q', 'r', 's', 't', 'u', 
  'v', 'w', 'x', 'y', 'z'];

static UPRCASE: [char; 26] = ['A', 
  'B', 'C', 'D', 'E', 'F', 
  'G', 'H', 'I', 'J', 'K', 
  'L', 'M', 'N', 'O', 'P', 
  'Q', 'R', 'S', 'T', 'U', 
  'V', 'W', 'X', 'Y', 'Z'];

static NMBRCASE: [char; 10] = [
  '0', '1', '2', '3', '4', 
  '5', '6', '7', '8', '9'];

static SPCLCASE: [char; 24] = [ 
  '+', '-', '=', '&', '!', 
  '(', ')', '{', '}', '[', 
  ']', '^', '~', '*', '?', 
  ':', '%', '$', '#', '@', 
  ';', '<', '>', '_'];
*/