use crate::{
  lang::load_localization, ui::menu::display::MenuLng, ui::menu::install::menu_install,
  ui::menu::installopt::diskinst::diskmg::diskmngterm,
  ui::menu::installopt::diskinst::mount::mount,
};
use pancurses::Input;

pub struct InstMenu;

impl InstMenu {
  pub fn diskmenu(lang: &str) {
    let localized_strings = load_localization(lang).expect("Failed to load localization");
    let window = pancurses::initscr();
    pancurses::curs_set(0);
    pancurses::noecho();
    pancurses::cbreak();
    ncurses::keypad(ncurses::stdscr(), true);
    let (screen_height, screen_width) = window.get_max_yx();

    let height = 10;
    let width = 40;
    let startx = (screen_width - width) / 2;
    let starty = (screen_height - height) / 2;

    let mut highlight = 1;
    let mut choice = 0;

    let menu_items = vec![
      localized_strings.disk_management.clone(),
      localized_strings.mounting.clone(),
      "".to_string(),
      localized_strings.exit_option.clone(),
    ];
    let menu_win = pancurses::newwin(height, width, starty, startx);
    menu_win.keypad(true);

    let menu_lng = MenuLng;

    loop {
      let menu_items_str: Vec<&str> = menu_items.iter().map(|s| s.as_str()).collect();
      menu_lng.displaymenud(
        &menu_win,
        &menu_items_str,
        highlight,
        &localized_strings.disk_config,
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
        diskmngterm(lang);
      }
      2 => {
        mount(lang);
      }
      _ => menu_install(lang),
    }
  }
}
