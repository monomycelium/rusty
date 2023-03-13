use std::{
    error::Error,
    io::{self, Read},
    thread,
    time::Duration, fmt::Display,
};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Terminal,
};

pub struct Task {
    pub completed: bool,
    pub description: String,
}

impl Task {
    pub fn new(description: String) -> Self {
        Task {
            completed: false,
            description,
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", if self.completed {"[x]"} else {"[ ]"}, self.description)
    }
}

pub enum InputMode {
    Normal,
    Editing,
}

struct App {
    pub tasks: Vec<Task>,
    pub state: ListState,
    pub mode: InputMode,
}

impl App {
    pub fn with_items(tasks: Vec<Task>) -> Self {
        App {
            tasks,
            state: ListState::default(),
            mode: InputMode::Normal,
        }
    }

    pub fn next(&mut self) {
        self.state
            .select(Some(if let Some(i) = self.state.selected() {
                (i + self.tasks.len() - 1) % self.tasks.len()
            } else {
                0
            }));
    }

    pub fn previous(&mut self) {
        self.state
            .select(Some(if let Some(i) = self.state.selected() {
                (i + 1) % self.tasks.len()
            } else {
                0
            }));
    }

    pub fn add_task(&mut self, description: String) {
        self.tasks.push(Task::new(description));
    }

    pub fn toggle_task(&mut self) {
        let i: usize = if let Some(i) = self.state.selected() {
            if i > self.tasks.len() - 1 {
                0
            } else {
                i
            }
        } else {
            0
        };

        self.tasks[i].completed = !self.tasks[i].completed;
    }

    pub fn remove_task(&mut self) {
        let i: usize = if let Some(i) = self.state.selected() {
            if i > self.tasks.len() - 1 {
                0
            } else {
                i
            }
        } else {
            0
        };

        self.tasks[i].completed = true;
    }
}

pub fn run(tasks: Vec<Task>) -> Result<(), Box<dyn Error>> {
    let mut terminal = Terminal::new(TermionBackend::new(io::stdout().into_raw_mode()?))?;
    terminal.clear()?;
    let mut asi = termion::async_stdin();

    let mut app: App = App::with_items(tasks);

    loop {
        terminal.draw(|f| {
            let chunks: Vec<Rect> = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Max(99), Constraint::Max(1)].as_ref())
                .split(f.size());

            let items: Vec<ListItem> = app
                .tasks
                .iter()
                .map(|task| {
                    let text: String = if task.completed {
                        format!("[x] {}", task.description)
                    } else {
                        format!("[ ] {}", task.description)
                    };

                    ListItem::new(Text::styled(text, Style::default()))
                })
                .collect();

            let items: List = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("our friendship bucket list")
                        .border_type(BorderType::Rounded),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("> ");

            f.render_stateful_widget(items, chunks[0], &mut app.state);
        })?;

        for key in asi.by_ref().keys() {
            match key? {
                Key::Char('q') => {
                    terminal.clear()?;
                    return Ok(());
                }
                Key::Char('j') | Key::Down => app.next(),
                Key::Char('k') | Key::Up => app.previous(),
                Key::Char(' ') => app.toggle_task(),
                Key::Char('d') => app.remove_task(),
                Key::Char('a') => app.mode = InputMode::Editing,
                _ => {}
            }
        }

        thread::sleep(Duration::from_millis(30));
    }
}
