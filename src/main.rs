use ncurses::*;

fn main() {
    initscr();
    addstr("Hei ncurses");

    let mut quit = false;

    while !quit {
        let key = getch();

        match key as u8 as char {
            'q' => quit = true,
            _ => {}
        }
    }

    endwin();
}
