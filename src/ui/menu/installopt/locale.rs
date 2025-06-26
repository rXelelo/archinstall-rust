use pancurses::Input;

use crate::{
  lang::load_localization,
  utils::editor::edit,
  ui::menu::{
    display::MenuLng, install::menu_install
  },
};
pub fn locale(lang: &str , err: i32) {
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
    let height = 15;
    let startx = (screen_width - width) / 2;
    let starty = (screen_height - height) / 2;

    let mut highlight = 1;
    let mut choice = 0;

    let menu_items = vec![
      localized_strings.keyboard_layout.clone(),
      localized_strings.locale_language.clone(),
      "".to_string(),
      "Back".to_string()
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
        err
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
        // layout
        let _ = edit("conf/locale.csv", 2, 8, "test");
      }
      2 => {
        // locale lang

      }
      4 => {
        menu_install(lang, err);
      }
      _ => (),
    }
  }
}
