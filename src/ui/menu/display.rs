pub(crate) struct MenuLng;

use pancurses;

impl MenuLng {
  pub fn dispinstmenu(
    &self,
    menu_win: &pancurses::Window,
    items: &[String],
    highlight: usize,
    header: &str,
  ) {
    menu_win.border(0, 0, 0, 0, 0, 0, 0, 0);
    menu_win.mvprintw(1, 3, header);

    for (i, item) in items.iter().enumerate() {
      if highlight == i + 1 {
        menu_win.attron(pancurses::A_BOLD);
        menu_win.mvprintw(3 + i as i32, 3, item);
        menu_win.attroff(pancurses::A_BOLD);
      } else {
        menu_win.mvprintw(3 + i as i32, 3, item);
      }
    }
    menu_win.refresh();
  }
  pub fn displaymenud(
    &self,
    menu_win: &pancurses::Window,
    items: &Vec<&str>,
    highlight: usize,
    header: &str,
  ) {
    menu_win.border(0, 0, 0, 0, 0, 0, 0, 0);
    menu_win.mvprintw(1, 3, header);

    for (i, item) in items.iter().enumerate() {
      if highlight == i + 1 {
        menu_win.attron(pancurses::A_REVERSE);
        menu_win.mvprintw(3 + i as i32, 3, item);
        menu_win.attroff(pancurses::A_REVERSE);
      } else {
        menu_win.mvprintw(3 + i as i32, 3, item);
      }
    }
    menu_win.refresh();
  }
}
