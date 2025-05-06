use ncurses::*;

fn main() {
    initscr();

    let mut quit = false;
    let todos = vec![
        "Lag todo app",
        "Drikk en kopp kaffe",
        "Fullfør det du begynte med",
    ];

    let todo_curr: usize = 0;

    while !quit {
        for (index, todo) in todos.iter().enumerate() {
            if todo_curr == index {}
            mv(index as i32, 1);
            addstr(todo);
        }

        refresh();

        let key = getch();

        match key as u8 as char {
            'q' => quit = true,
            _ => {}
        }
    }

    endwin();
}
