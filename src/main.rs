mod gui;
mod setter;

fn main() {
    let screen: gui::Screen = gui::Screen::new();
    screen.print_center("WELCOME", 0);
    screen.print_center("PRESS <ENTER> TO CONTINUE!", 1);

    gui::Screen::wait_key('\n');
    gui::Screen::cls();

    gui::Screen::cls();

    let mut line: i32 = 0;
    let mut cons_series: Vec<setter::series::Series> = vec![];
    let src_tb: setter::table::Table = setter::table::Table::default();
    src_tb.print(0, &|text, y| screen.print_ltop(text, y));

    screen.print_bottom("[w] - Up, [s] - Down, [space] - Select gradation", 2);
    screen.print_bottom("[[] - Set min, []] - Set max", 1);
    gui::Screen::refresh_gui();

    loop {
        gui::Screen::cls();
        src_tb.print(line as usize, &|text, y| screen.print_ltop(text, y));
        gui::Screen::refresh_gui();
        match gui::Screen::wait_any_key() {
            119 => {  // 'w'
                line = std::cmp::max(line - 1, 0);
            }
            115 => {  // 's'
                line = std::cmp::min(line + 1, (src_tb.body.len() - 1) as i32);
            }
            32 => {   // Space
                cons_series.push(src_tb.body[line as usize].clone());
            }
            91 => {   // '['
            }
            93 => {   // ']'
            }
            _ => {
                break;
            }
        }
    }

    gui::Screen::cls();

    let mut cons_tb: setter::table::Table = setter::generate_series(&cons_series).unwrap_or(setter::table::Table::empty());
    cons_tb.to_uniq();

    screen.print_top("GENERATED SERIES. PRESS <ENTER> TO CONTINUE.", 0);
    cons_tb.print(0, &|text, y| screen.print_ltop(text, y));

    gui::Screen::wait_key('\n');
    gui::Screen::cls();

    let mut sets = setter::generate_sets(&cons_tb).unwrap_or(setter::table::Table::empty());
    sets.to_uniq();
    sets.filter_series_by_range(0., 100.);

    screen.print_top("POSSIBLE SETS. PRESS <ENTER> TO EXIT AND SAVE. <q> TO EXIT.", 0);
    sets.print(0, &|text, y| screen.print_ltop(text, y));
    gui::Screen::refresh_gui();

    gui::Screen::kill_gui();
}
