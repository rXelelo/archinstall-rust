pub(crate) struct MenuLng;

use pancurses;

impl MenuLng {
  pub fn dispinstmenu(
    &self,
    menu_win: &pancurses::Window,
    items: &[String],
    highlight: usize,
    header: &str,
    err: i32,
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
    match err {
      0 => (),
      1 => {
        menu_win.mvprintw(13, 3, "Error was found");
      }
      _ => (),
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
  pub fn disptimezone(
    &self,
    window: &pancurses::Window,
    menu_items: &Vec<String>,
    global_highlight_idx: usize,
    title: &str,
    pad_offset: usize,
  ) {
    window.mvprintw(0, 1, title);

    let (max_y, max_x) = window.get_max_yx();
    let display_rows = (max_y as usize).saturating_sub(2);

    for i in 0..display_rows {
      let item_index_in_full_list = pad_offset.saturating_add(i);

      if item_index_in_full_list < menu_items.len() {
        let item_text = &menu_items[item_index_in_full_list];

        if item_index_in_full_list == global_highlight_idx {
          window.attron(pancurses::A_REVERSE);
          window.mvprintw(
            (i + 1) as i32,
            1,
            &format!(
              "{:width$}",
              item_text,
              width = (max_x as usize).saturating_sub(2)
            ),
          );
          window.attroff(pancurses::A_REVERSE);
        } else {
          window.mvprintw(
            (i + 1) as i32,
            1,
            &format!(
              "{:width$}",
              item_text,
              width = (max_x as usize).saturating_sub(2)
            ),
          );
        }
      } else {
        window.mvprintw(
          (i + 1) as i32,
          1,
          &format!("{:width$}", "", width = (max_x as usize).saturating_sub(2)),
        );
      }
    }
    window.refresh();
  }
}
