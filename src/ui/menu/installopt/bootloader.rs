use pancurses::Input;

use crate::{lang::load_localization, ui::menu::display::MenuLng};

pub fn bootloader(lang: &str) -> String {
  let localized_strings = load_localization(lang).expect("Failed to load localization");
  let _ = ncurses::setlocale(ncurses::LcCategory::all, "uk_UA.UTF-8");
  let window = pancurses::initscr();
  pancurses::curs_set(0);
  pancurses::noecho();
  pancurses::cbreak();
  ncurses::keypad(ncurses::stdscr(), true);

  let (screen_height, screen_width) = window.get_max_yx();

  let width = 28;
  let height = 7;
  let startx = (screen_width - width) / 2;
  let starty = (screen_height - height) / 2;

  let mut highlight = 1;
  let mut choice = 0;

  let menu_items = vec!["Grub".to_string(), "Lima".to_string()];

  let menu_win = pancurses::newwin(height, width, starty, startx);
  menu_win.keypad(true);

  let menu_lng = MenuLng;

  loop {
    menu_lng.dispinstmenu(
      &menu_win,
      &menu_items,
      highlight,
      &localized_strings.bootloader_select,
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
      let loader = String::from("grub");
      return loader;
    }
    2 => {
      let loader = String::from("lima");
      return loader;
    }
    _ => {
      let loader = String::from("Nothing");
      return loader;
    }
  }
}
