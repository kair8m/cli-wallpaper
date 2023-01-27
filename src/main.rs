use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout, Rect, Direction},
    style::{Modifier, Style},
    text::Span,
    widgets::{List, ListItem},
    Frame, Terminal,
};
mod image;

struct App<'a> {
    items: Vec<&'a str>,
    selected_image_index: usize,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: vec![
                "aurora",
                "beach",
                "tokyo",
                "chihuahuan",
                "cliffs",
                "colony",
                "desert",
                "earth",
                "exodus",
                "factory",
                "firewatch",
                "forest",
                "gradient",
                "home",
                "island",
                "lake",
                "lakeside",
                "market",
                "mojave",
                "moon",
                "mountains",
                "room",
                "sahara",
                "street",
            ],
            selected_image_index: 0,
        }
    }
    pub fn next_image(&mut self) {
        let mut i = self.selected_image_index + 1;
        if i >= self.items.len() {
            i = 0;
        }
        self.selected_image_index = i;
    }

    pub fn prev_image(&mut self) {
        let mut i: isize = self.selected_image_index as isize - 1;
        if i < 0 {
            i = self.items.first().expect("No Value").len() as isize - 1;
        }
        self.selected_image_index = i as usize;
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
    let app = App::new();
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
                KeyCode::Down | KeyCode::Char('j') => app.next_image(),
                KeyCode::Up | KeyCode::Char('k') => app.prev_image(),
                KeyCode::Left | KeyCode::Char('h') => {}
                KeyCode::Right | KeyCode::Char('l') => {}
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects: Vec<Rect> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(66), Constraint::Percentage(33)].as_ref())
        .margin(5)
        .split(f.size());
    let mut idx: usize = 0;
    let items: Vec<ListItem> = app
        .items
        .iter()
        .map(|image_name| {
            let style = match idx {
                idx if idx == app.selected_image_index => {
                    Style::default().add_modifier(Modifier::REVERSED)
                }
                _ => Style::default(),
            };
            let text = Span::styled(*image_name, style);
            idx = idx + 1;
            ListItem::new(text)
        })
        .collect();
    let items = List::new(items);
    f.render_widget(items, rects[1]);
    let image_path = crate::image::get_image_path(app.items[app.selected_image_index]);

    let image_widget = crate::image::get_image_widget(
        image_path.as_str(),
        rects[0].width as u32,
        rects[0].height as u32,
    )
    .unwrap();
    f.render_widget(image_widget, rects[0]);
}
