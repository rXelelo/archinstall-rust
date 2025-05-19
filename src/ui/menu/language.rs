use pancurses::Input;

use crate::{
  lang::load_localization,
  ui::menu::{display::MenuLng, install::menu_install},
};

pub(crate) fn menu_lang() {
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

  let menu_items = vec!["1. English            ", "2. Ukraine            "];

  let menu_win = pancurses::newwin(height, width, starty, startx);
  menu_win.keypad(true);

  let localized_strings = load_localization("en").expect("Failed to load English localization");

  let menu_lng = MenuLng;

  loop {
    menu_lng.displaymenud(
      &menu_win,
      &menu_items,
      highlight,
      &localized_strings.select_language_header,
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
    1 => menu_install("en"),
    2 => menu_install("uk"),
    _ => (),
  }
}
