mod gui;
mod setter;

fn main() {
    let screen: gui::Screen = gui::init_gui();
    gui::print_center("WELCOME", 0, &screen);
    gui::print_center("PRESS <ENTER> TO CONTINUE!", 1, &screen);

    gui::wait_key('\n');
    gui::cls();

    gui::cls();
    // gui::print_center("USED GRADS AND SOURCE. PRESS <ENTER> TO CONTINUE\n", -(screen_sizes.y / 2));

    let mut line: usize = 0;
    let mut tb: setter::table::Table = setter::table::Table::default();
    tb.print("AVAILABLE GRADATIONS", line);

    gui::print_bottom("[w] - Up, [s] - Down, [space] - Select gradation", 2, &screen);
    gui::print_bottom("[[] - Set min, []] - Set max", 1, &screen);

    loop {
        match gui::wait_any_key() {
            119 => {  // 'w'
            }
            115 => {  // 's'
            }
            32 => {   // Space
                break;
            }
            91 => {   // '['
            }
            93 => {   // ']'
            }
            _ => {
            }
        }
    }

    gui::kill_gui();
}
