use crate::{lang::*, ui::menu::installopt::diskinst::diskmg::diskmngterm};
use std::io::{self, Write};
use std::process::Command;

pub fn formate(lang: &str) {
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
      println!("{}", localized_strings.help_menu2.clone(),);
    } else if command == "exit" {
      diskmngterm(lang);
    } else if command == "list" {
      println!("Listing all available disks:");
      let _ = Command::new("lsblk").status();
    } else {
      let parts: Vec<&str> = command.split_whitespace().collect();
      if parts.is_empty() {
        println!("Empty command. Type 'help' for available commands.");
        continue;
      }

      let cmd_name = parts[0];
      let args = &parts[1..];

      println!("Executing: {} {}", cmd_name, args.join(" "));

      let _ = Command::new(cmd_name).args(args).status();
    }
  }
}
