use crate::{
  lang::*, ui::menu::installopt::disk::InstMenu, ui::menu::installopt::diskinst::formate::formate,
};
use std::io::{self, Write};
use std::process::Command;

pub fn diskmngterm(lang: &str) {
  loop {
    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    std::io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");
    let command = input.trim();
    if command == "help" {
      let localized_strings = load_localization(lang).expect("Failed to load localization");
      println!("{}", localized_strings.help_menu1.clone(),);
    } else if command == "list" {
      let _ = Command::new("lsblk").status();
    } else if command == "cfdisk" {
      let _ = Command::new("cfdisk").status();
    } else if command == "exit" {
      InstMenu::diskmenu(lang);
    } else if command == "format" {
      formate(lang);
    }
  }
}
