use pancurses::Input;
use std::cmp::{max, min};

use crate::{lang::load_localization, ui::menu::display::MenuLng, ui::menu::installopt::tzlist::gettimezone};

pub fn timezone(lang: &str) -> String {
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
  let display_rows = (height as usize).saturating_sub(2);

  let startx = (screen_width - width) / 2;
  let starty = (screen_height - height) / 2;

  let mut highlight: usize = 0;
  let mut pad_offset: usize = 0;

  let menu_items = gettimezone();

  let total_items = menu_items.len();

  let menu_win = pancurses::newwin(height, width, starty, startx);
  menu_win.keypad(true);

  let menu_lng = MenuLng;

  if total_items > 0 {
  } else {
    window.mvprintw(
      starty + height / 2,
      startx + (width - "No items.".len() as i32) / 2,
      "No items.",
    );
    window.refresh();
    window.getch();
    drop(menu_win);
    ncurses::endwin();
    return String::from("No selection (empty list)");
  }
  let mut choice: Option<usize> = None;
  loop {
    menu_win.clear();
    menu_win.border(0, 0, 0, 0, 0, 0, 0, 0);

    menu_lng.disptimezone(
      &menu_win,
      &menu_items,
      highlight,
      &localized_strings.bootloader_select,
      pad_offset,
    );

    menu_win.refresh();

    match menu_win.getch() {
      Some(Input::KeyUp) => {
        if highlight == 0 {
          highlight = total_items.saturating_sub(1);
          pad_offset = total_items.saturating_sub(display_rows);
        } else {
          highlight = highlight.saturating_sub(1);
          if highlight < pad_offset {
            pad_offset = pad_offset.saturating_sub(1);
          }
        }
      }
      Some(Input::KeyDown) => {
        if highlight >= total_items.saturating_sub(1) {
          highlight = 0;
          pad_offset = 0;
        } else {
          highlight = highlight.saturating_add(1);
          if highlight >= pad_offset.saturating_add(display_rows) {
            pad_offset = pad_offset.saturating_add(1);
          }
        }
      }
      Some(Input::KeyPPage) => {
        pad_offset = pad_offset.saturating_sub(display_rows);
        highlight = min(
          highlight,
          pad_offset.saturating_add(display_rows).saturating_sub(1),
        );
        highlight = max(highlight, pad_offset);
      }
      Some(Input::KeyNPage) => {
        pad_offset = min(
          total_items.saturating_sub(display_rows),
          pad_offset.saturating_add(display_rows),
        );
        highlight = min(
          highlight,
          pad_offset.saturating_add(display_rows).saturating_sub(1),
        );
        highlight = max(highlight, pad_offset);
      }
      Some(Input::Character('\n')) => {
        choice = Some(highlight);
        break;
      }
      _ => (),
    }
  }

  window.clear();
  window.refresh();
  drop(menu_win);
  ncurses::endwin();

  match choice {
    Some(index) if index < total_items => menu_items[index].clone(),
    _ => String::from("No selection"),
  }
}
