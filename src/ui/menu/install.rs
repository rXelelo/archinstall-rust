use std::process::{Command, exit};

use pancurses::Input;

use crate::{
  lang::load_localization,
  ui::menu::{
    display::MenuLng, installopt::bootloader::bootloader, installopt::disk::InstMenu,
    installopt::root::rootget, installopt::user::usrname, installopt::user::usrpsw,
    instscript::installer::install_arch, language::menu_lang, val::InstallValue,
  },
};
pub fn menu_install(lang: &str) {
  let mut installvalue = InstallValue::default();
  loop {
    let localized_strings = load_localization(lang).expect("Failed to load localization");
    let _ = ncurses::setlocale(ncurses::LcCategory::all, "uk_UA.UTF-8");
    let window = pancurses::initscr();
    pancurses::curs_set(0);
    pancurses::noecho();
    pancurses::cbreak();
    ncurses::keypad(ncurses::stdscr(), true);

    let (screen_height, screen_width) = window.get_max_yx();

    let width = 28;
    let height = 12;
    let startx = (screen_width - width) / 2;
    let starty = (screen_height - height) / 2;

    let mut highlight = 1;
    let mut choice = 0;

    let menu_items = vec![
      localized_strings.language_menu_option.clone(),
      localized_strings.disk_config.clone(),
      localized_strings.bootloader_select.clone(),
      localized_strings.root_password.clone(),
      localized_strings.setup_user.clone(),
      "".to_string(),
      localized_strings.install_archlinux.clone(),
      localized_strings.exit_option.clone(),
    ];

    let menu_win = pancurses::newwin(height, width, starty, startx);
    menu_win.keypad(true);

    let menu_lng = MenuLng;

    loop {
      menu_lng.dispinstmenu(
        &menu_win,
        &menu_items,
        highlight,
        &localized_strings.setup_header,
      );

      match menu_win.getch() {
        Some(Input::KeyUp) => loop {
          highlight = if highlight == 1 {
            menu_items.len()
          } else {
            highlight - 1
          };
          if !menu_items[highlight - 1].is_empty() {
            break;
          }
        },
        Some(Input::KeyDown) => loop {
          highlight = if highlight == menu_items.len() {
            1
          } else {
            highlight + 1
          };
          if !menu_items[highlight - 1].is_empty() {
            break;
          }
        },
        Some(Input::Character('\n')) => {
          choice = highlight;
          break;
        }
        _ => (),
      }

      if choice != 0 {
        break;
      }
    }
    window.clear();
    window.refresh();
    drop(menu_win);
    ncurses::endwin();
    match choice {
      1 => {
        // Language menu
        menu_lang();
      }
      2 => {
        // disk
        InstMenu::diskmenu(lang);
      }
      3 => {
        installvalue.loader = bootloader(lang);
      }
      4 => {
        println!("Write root password");
        installvalue.rootpsw = rootget();
      }
      5 => {
        println!("Write user name");
        installvalue.username = usrname();
        println!("{}", installvalue.username);
        println!("Write user password");
        installvalue.userpsw = usrpsw();
      }
      7 => {
        let _ = Command::new("clear").status();
        install_arch(
          installvalue.loader,
          installvalue.rootpsw,
          installvalue.username,
          installvalue.userpsw
        );
        break;
      }
      8 => {
        exit(0);
      }
      _ => (),
    }
  }
}
