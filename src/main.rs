use ncurses::*;

fn main() {
    initscr();
    addstr("Hei ncurses");

    let quit = false;

    while !quit {
        let key = getch();
    }

    endwin();
}
