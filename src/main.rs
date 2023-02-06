use anyhow::{anyhow, Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{List, ListItem},
    Frame, Terminal,
};
mod image;

struct App {
    items: Vec<String>,
    selected_image_index: usize,
}

impl App {
    fn new() -> Result<App> {
        Ok(App {
            items: image::list_images()?,
            selected_image_index: 0,
        })
    }

    pub fn next_image(&mut self) {
        let mut i = self.selected_image_index + 1;
        if i >= self.items.len() {
            i = 0;
        }
        self.selected_image_index = i;
    }

    pub fn prev_image(&mut self) {
        if self.selected_image_index == 0 {
            self.selected_image_index = self.items.len() - 1;
        } else {
            self.selected_image_index = self.selected_image_index - 1;
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            let mut ui_failed = false;
            terminal.draw(|f| {
                let res = self.render(f);
                match res {
                    Ok(_) => {}
                    Err(_) => {
                        ui_failed = true;
                    }
                };
            })?;

            if ui_failed {
                return Err(anyhow!("Ui failed"));
            }
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => self.next_image(),
                    KeyCode::Up | KeyCode::Char('k') => self.prev_image(),
                    KeyCode::Left | KeyCode::Char('h') => {}
                    KeyCode::Right | KeyCode::Char('l') => {}
                    _ => {}
                }
            }
        }
    }

    fn render<B: Backend>(&mut self, f: &mut Frame<B>) -> Result<()> {
        let rects: Vec<Rect> = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(66), Constraint::Percentage(33)].as_ref())
            .margin(5)
            .split(f.size());
        let mut idx: usize = 0;
        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|image_name| {
                let style = match idx {
                    idx if idx == self.selected_image_index => {
                        Style::default().add_modifier(Modifier::REVERSED)
                    }
                    _ => Style::default(),
                };
                let text = Span::styled(image_name, style);
                idx = idx + 1;
                ListItem::new(text)
            })
            .collect();
        let items = List::new(items);
        f.render_widget(items, rects[1]);
        let image_path =
            crate::image::get_image_path(self.items[self.selected_image_index].as_str())?;

        let image_widget = crate::image::get_image_widget(
            image_path.as_str(),
            rects[0].width as u32,
            rects[0].height as u32,
        )
        .context("Failed to get image widget")?;
        f.render_widget(image_widget, rects[0]);
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut app = App::new()?;
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = app.run(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}
