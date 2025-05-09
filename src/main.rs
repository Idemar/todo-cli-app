use ncurses::*;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

type Id = usize;

#[derive(Default)]
struct Ui {
    list_curr: Option<Id>,
    row: usize,
    col: usize,
}

impl Ui {
    fn start(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }
    
    fn start_liste(&mut self, id: Id) {
        assert!(self.list_curr.is_none(), "nestede lister er ikke tillatt");
        self.list_curr = Some(id);
    }

    fn liste_elementer(&mut self, label: &str, id: Id) -> bool {
        let id_curr = self.list_curr.expect("Ikke lov å lager list elemnet utenfor list");

        self.label(label, {
            if id_curr == id {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            }
        });
        return false;
    }

    fn label(&mut self, text: &str, pair: i16) {
        mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair));
        addstr(text);
        attroff(COLOR_PAIR(pair));
        self.row += 1;
    }

    fn slutt_liste(&mut self) {
        self.list_curr = None;
    }

    fn slutt(&mut self) { 
    }
}

enum Fokus {
    Todo,
    Ferdig,
}

impl Fokus {
    fn bytt(&self) -> Self {
        match self {
            Fokus::Todo => Fokus::Ferdig,
            Fokus::Ferdig => Fokus::Todo,
        }
    }
}

fn list_opp(list_curr: &mut usize) {
    if *list_curr > 0 {
        *list_curr -= 1;
    }
}

fn list_ned(list: &Vec<String>, list_curr: &mut usize) {
    if *list_curr + 1 < list.len() {
        *list_curr += 1;
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
    let mut todos: Vec<String> = vec![
        "Lag todo app".to_string(),
        "Drikk en kopp kaffe".to_string(),
        "Fullfør det du begynte med".to_string(),
    ];

    let mut dones = vec![
        "Stå opp".to_string(),
        "Spis frokost".to_string(),
    ];

    let mut done_curr: usize = 0;
    let mut todo_curr: usize = 0;
    let mut fokus = Fokus::Todo;

    let mut ui = Ui::default();

    while !quit {
        erase();

        ui.start(0, 0);
        {
            match fokus {
                Fokus::Todo => {
                    ui.label("[TODO] FERDIG ", REGULAR_PAIR);
                    ui.label("------------- ", REGULAR_PAIR);
                    ui.start_liste(todo_curr);

                    for (index, todo) in todos.iter().enumerate() {
                    ui.liste_elementer(&format!("- [ ] {}",todo), index);
                    }
                    ui.slutt_liste();
                },
                Fokus::Ferdig => {
                    ui.label(" TODO [FERDIG]", REGULAR_PAIR);
                    ui.label(" -------------", REGULAR_PAIR);
                    ui.start_liste(done_curr);
                    for (index, done) in dones.iter().enumerate() {
                     ui.liste_elementer(&format!("- [x] {}",done), index);
                    }
                    ui.slutt_liste();
                }
            }
        }
        ui.slutt();

        refresh();

        let key = getch();

        match key as u8 as char {
            // q avslutter programmet
            'q' => quit = true,
            // w flytter opp i lista
            'w' => match fokus {
                    Fokus::Todo => list_opp(&mut todo_curr),
                    Fokus::Ferdig => list_opp(&mut done_curr),
                }
            // s flytter ned i lista
            's' => match fokus {
                    Fokus::Todo => list_ned(&todos, &mut todo_curr),
                    Fokus::Ferdig => list_ned(&dones, &mut done_curr),
                }
            '\n' => match fokus {
                Fokus::Todo => if todo_curr < todos.len() {
                    dones.push(todos.remove(todo_curr));
                    if todo_curr >= todos.len() && todos.len() > 0 {
                        todo_curr = todos.len() - 1;
                    }
                }
                Fokus::Ferdig => if done_curr < dones.len() {
                    todos.push(dones.remove(done_curr));
                    if done_curr >= dones.len() && dones.len() > 0 {
                        done_curr = dones.len() - 1;
                    }
                }
            }
            '\t' => {
                fokus = fokus.bytt();
            }
            _ => {}
        }
    }

    endwin();
}
