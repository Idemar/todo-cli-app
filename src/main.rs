use std::ops::Index;

use ncurses::*;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

type Id = usize;

#[derive(Default)]
struct Ui {
    list_curr: Option<Id>,
}

impl Ui {
    fn start(&mut self) {
        todo!();
    }
    
    fn start_liste(&mut self, id: Id) {
        todo!();
    }

    fn liste_elementer(&mut self, label: &str, id: Id) {
        // let pair = {
        //     if todo_curr == index {
        //         HIGHLIGHT_PAIR
        //     } else {
        //         REGULAR_PAIR
        //     }
        // };
        // attron(COLOR_PAIR(pair));
        // mv(index as i32, 1);
        // addstr(*todo);
        // attroff(COLOR_PAIR(pair));
        todo!();
    }

    fn label(&mut self, label: &str) {
        todo!();
    }

    fn slutt_liste(&mut self) {
        todo!();
    }

    fn slutt(&mut self) {
        todo!();
    }
}
fn main() {
    initscr();
    noecho();

    // Sett markøren til usynlig
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;
    let todos: Vec<String> = vec![
        "Lag todo app".to_string(),
        "Drikk en kopp kaffe".to_string(),
        "Fullfør det du begynte med".to_string(),
    ];

    let dones = Vec::<String>::new();
    let mut done_curr: usize = 0;
    let mut todo_curr: usize = 0;

    let mut ui = Ui::default();

    while !quit {
        ui.start();
        {
            ui.start_liste(todo_curr);

            for (index, todo) in todos.iter().enumerate() {
                ui.liste_elementer(todo, index);
            }
            ui.slutt_liste();

            ui.label("---------------------------------------------------");

            ui.start_liste(done_curr);
            for (Index, done) in dones.iter().enumerate() {
                ui.liste_elementer(&done, Index);
            }
            ui.slutt_liste();
            ui.slutt();
        }

        refresh();

        let key = getch();

        match key as u8 as char {
            // q avslutter programmet
            'q' => quit = true,
            // w flytter opp i lista
            'w' => if todo_curr > 0 {
                todo_curr -=1;
            },
            // s flytter ned i lista
            's' => if todo_curr < todos.len() - 1 {
                todo_curr += 1;
            },
            _ => {}
        }
    }

    endwin();
}
