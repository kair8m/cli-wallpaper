use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};

struct App<'a> {
    items: Vec<Vec<&'a str>>,
    selected_row: usize,
    selected_col: usize,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: vec![
                vec!["aurora", "beach", "tokyo"],
                vec!["chihuahuan", "cliffs", "colony"],
                vec!["desert", "earth", "exodus"],
                vec!["factory", "firewatch", "forest"],
                vec!["gradient", "home", "island"],
                vec!["lake", "lakeside", "market"],
                vec!["mojave", "moon", "mountains"],
                vec!["room", "sahara", "street"],
            ],
            selected_row: 0,
            selected_col: 0,
        }
    }
    pub fn next_row(&mut self) {
        let mut i = self.selected_row + 1;
        if i >= self.items.len() {
            i = 0;
        }
        self.selected_row = i;
    }

    pub fn prev_row(&mut self) {
        let mut i: isize = self.selected_row as isize - 1;
        if i < 0 {
            i = self.items.first().expect("No Value").len() as isize - 1;
        }
        self.selected_row = i as usize;
    }

    pub fn next_col(&mut self) {
        let mut i = self.selected_col + 1;
        if i >= self.items.first().expect("No value").len() {
            i = 0;
        }
        self.selected_col = i;
    }

    pub fn prev_col(&mut self) {
        let mut i: isize = self.selected_col as isize - 1;
        if i < 0 {
            i = self.items.first().expect("No Value").len() as isize - 1;
        }
        self.selected_col = i as usize;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down | KeyCode::Char('j') => app.next_row(),
                KeyCode::Up | KeyCode::Char('k') => app.prev_row(),
                KeyCode::Left | KeyCode::Char('h') => app.prev_col(),
                KeyCode::Right | KeyCode::Char('l') => app.next_col(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(5)
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Gray);
    let header_cells = ["Header1", "Header2", "Header3"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let mut col_idx: usize = 0;
    let mut row_idx: usize = 0;
    let rows = app.items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        col_idx = 0;
        let row = row_idx.clone();
        let cells = item.iter().map(|c| {
            let cell = match (col_idx, row) {
                (x, y) if x == app.selected_col && y == app.selected_row => {
                    Cell::from(">> ".to_owned() + *c).style(selected_style)
                }
                _ => Cell::from(*c),
            };
            col_idx = col_idx + 1;
            cell
        });
        row_idx = row_idx + 1;
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Table"))
        .highlight_style(selected_style)
        // .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ]);
    f.render_widget(t, rects[0]);
    // f.render_stateful_widget(t, rects[0], &mut app.state);
}
