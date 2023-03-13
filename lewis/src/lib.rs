use chrono::{Local, NaiveDate};
use figlet_rs::FIGfont;
use std::{
    env,
    error::Error,
    fmt::Display,
    fs::{self, OpenOptions},
    io::{self, BufWriter, Read, Write},
    path::Path,
    str::FromStr,
    thread,
    time::Duration,
};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};

pub struct Task {
    pub completed: bool,
    pub description: String,
}

impl Task {
    pub fn from(description: String) -> Self {
        Task {
            completed: false,
            description,
        }
    }

    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            if self.completed { "[x]" } else { "[ ]" },
            self.description
        )
    }
}

impl FromStr for Task {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..3] {
            "[ ]" => Ok(Task {
                completed: false,
                description: s[4..].to_string(),
            }),
            "[x]" => Ok(Task {
                completed: true,
                description: s[4..].to_string(),
            }),
            _ => Err("failed to parse tasks".into()),
        }
    }
}

pub enum InputMode {
    Normal,
    Editing,
}

struct App {
    pub tasks: Vec<Task>,
    pub state: ListState,
    pub input: String,
    pub mode: InputMode,
}

impl Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in self.tasks.iter() {
            writeln!(f, "{}", item)?;
        }

        Ok(())
    }
}

impl FromStr for App {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s
            .lines()
            .map(|line| line.parse::<Task>())
            .collect::<Result<_, _>>()?;

        Ok(App::from(items))
    }
}

impl App {
    pub fn from(tasks: Vec<Task>) -> Self {
        App {
            tasks,
            state: ListState::default(),
            input: String::new(),
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

    pub fn remove_task(&mut self) {
        let len: usize = self.tasks.len();

        if len > 0 {
            if let Some(i) = self.state.selected() {
                if len - 1 == i {
                    self.previous();
                }

                self.tasks.remove(i);
            }
        }
    }

    pub fn toggle_task(&mut self) {
        let i: usize = if let Some(i) = self.state.selected() {
            if i >= self.tasks.len() {
                0
            } else {
                i
            }
        } else {
            0
        };

        self.tasks[i].toggle();
    }

    pub fn save<P>(&self, path: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        fs::write(path, self.to_string())?;

        Ok(())
    }

    pub fn load<P>(path: P) -> Result<Self, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let mut data: String = String::new();
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?
            .read_to_string(&mut data)?;

        Ok(data.parse()?)
    }
}

pub fn greet_lewis() -> Result<(), Box<dyn Error>> {
    let mut out = BufWriter::new(io::stdout());

    let font: FIGfont = FIGfont::from_file("resources/starwars.flf")?;
    let figure: Option<figlet_rs::FIGure> = font.convert("lewis!");
    if let Some(thing) = figure {
        writeln!(out, "{}", thing)?;
    }

    writeln!(out, "a birthday present from mashrafi, written in Rust.")?;
    out.flush()?;
    thread::sleep(Duration::from_millis(500));

    if let Some(date) = NaiveDate::from_ymd_opt(2023, 03, 31) {
        if Local::now().date_naive() <= date {
            writeln!(out, "\nHappy birthday, Lewis!")?;
            out.flush()?;
            thread::sleep(Duration::from_millis(500));

            write!(out, "I wrote you this program to keep track of a bucket list that I've created for us.\nYou can add and remove items, and mark them as complete or incomplete. Are you ready? (press enter) ")?;
            out.flush()?;

            let mut buffer: Vec<u8> = Vec::new();
            io::stdin().read(&mut buffer)?;
        }
    }

    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    greet_lewis()?;

    let mut terminal = Terminal::new(TermionBackend::new(io::stdout().into_raw_mode()?))?;
    terminal.clear()?;
    let mut asi = termion::async_stdin();

    let app_path = Path::new(
        &env::var("XDG_CONFIG_HOME").unwrap_or(env::var("HOME").unwrap_or(".".to_string())),
    )
    .join(".bucket_list");
    let mut app: App = if app_path.exists() {
        App::load(&app_path)?
    } else {
        App::load(&Path::new("resources/bucket_list"))?
    };

    loop {
        terminal.draw(|f| {
            let chunks: Vec<Rect> = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(85),
                        Constraint::Percentage(10),
                        Constraint::Min(2),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let items: Vec<ListItem> = app
                .tasks
                .iter()
                .map(|task| -> ListItem {
                    ListItem::new(Text::styled(task.to_string(), Style::default()))
                })
                .collect();

            let items: List = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("our friendship bucket list"),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("> ");

            let message = match app.mode {
                InputMode::Normal => {
                    vec![
                        Span::raw("press "),
                        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to quit, "),
                        Span::styled("space", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to toggle, "),
                        Span::styled("a", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to add task, and "),
                        Span::styled("d", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to delete task."),
                    ]
                }
                InputMode::Editing => {
                    vec![
                        Span::raw("enter task to add. press "),
                        Span::styled("escape", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to stop editing, and "),
                        Span::styled("enter", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to write the task."),
                    ]
                }
            };

            let message =
                Paragraph::new(Text::from(Spans::from(message))).wrap(Wrap { trim: true });

            f.render_stateful_widget(items, chunks[0], &mut app.state);
            f.render_widget(message, chunks[1]);

            match app.mode {
                InputMode::Editing => {
                    f.render_widget(Paragraph::new(app.input.as_ref()), chunks[2]);
                    f.set_cursor(chunks[2].x + app.input.len() as u16, chunks[2].y);
                }
                _ => {}
            }
        })?;

        for key in asi.by_ref().keys() {
            match app.mode {
                InputMode::Normal => match key? {
                    Key::Char('q') => {
                        terminal.clear()?;
                        app.save(&app_path)?;
                        return Ok(());
                    }
                    Key::Char('j') | Key::Down => app.previous(),
                    Key::Char('k') | Key::Up => app.next(),
                    Key::Char(' ') => app.toggle_task(),
                    Key::Char('d') => app.remove_task(),
                    Key::Char('a') => app.mode = InputMode::Editing,
                    _ => {}
                },
                InputMode::Editing => match key? {
                    Key::Char('\n') => {
                        let input: String = app.input.drain(..).collect();
                        if input.len() > 0 {
                            app.tasks.push(Task::from(input));
                        };
                        app.mode = InputMode::Normal;
                    }
                    Key::Char(c) => {
                        app.input.push(c);
                    }
                    Key::Backspace => {
                        app.input.pop();
                    }
                    Key::Esc => {
                        app.mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }

        thread::sleep(Duration::from_millis(40));
    }
}
