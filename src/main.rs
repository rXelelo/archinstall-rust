mod utils;

mod lang;
mod ui;
use std::process::Command;
use ui::menu::language::menu_lang;
use utils::checkos;

fn main() {
  // Check if the system is Linux
  checkos::check();
  let _ = Command::new("setfont").args(["cyr-sun16"]).status();
  // Start the language selection menu
  menu_lang();
}
