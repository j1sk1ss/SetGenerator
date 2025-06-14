use ncurses;

pub struct Screen {
    x: i32,
    y: i32
}

pub fn init_gui() -> Screen {
    ncurses::initscr();
    ncurses::noecho();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::clear();
    ncurses::refresh();

    let stdscr = ncurses::stdscr();
    let mut max_y = 0;
    let mut max_x = 0;
    ncurses::getmaxyx(stdscr, &mut max_y, &mut max_x);

    return Screen {
        x: max_x,
        y: max_y
    };
}

pub fn cls() -> i32 {
    return ncurses::clear();
}

pub fn print_top(text: &str, y_offset: i32, scr: &Screen) {
    let x = (scr.x - text.len() as i32) / 2;
    ncurses::mvprintw(y_offset, x, text);
}

pub fn print_center(text: &str, y_offset: i32, scr: &Screen) {
    let x = (scr.x - text.len() as i32) / 2;
    let y = scr.y / 2 + y_offset;
    ncurses::mvprintw(y, x, text);
}


pub fn print_bottom(text: &str, y_offset: i32, scr: &Screen) {
    let x = (scr.x - text.len() as i32) / 2;
    ncurses::mvprintw(scr.y - y_offset, x, text);
}

pub fn wait_key(expected: char) {
    loop {
        let ch: i32 = ncurses::getch();
        if ch == expected as i32 {
            break;
        }
    }
}

pub fn wait_any_key() -> i32 {
    return ncurses::getch()
}

pub fn kill_gui() -> i32 {
    return ncurses::endwin();
}

