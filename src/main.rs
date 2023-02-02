use anyhow::{anyhow, Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{fs::read_dir, io, path::PathBuf};
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
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("images");
        let image_files = read_dir(path).context("couldn't find images in filesystem")?;
        Ok(App {
            items: image_files
                .into_iter()
                .filter(|file| match file {
                    Err(_) => false,
                    Ok(file) => !file.path().to_str().unwrap().contains("preview"),
                })
                .map(|file| {
                    file.context("Image access failed")
                        .unwrap()
                        .path()
                        .with_extension("")
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
                .collect(),
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
}

fn main() -> Result<()> {
    let app = App::new()?;
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    loop {
        let mut ui_failed = false;
        terminal.draw(|f| {
            let res = ui(f, &mut app);
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
                KeyCode::Down | KeyCode::Char('j') => app.next_image(),
                KeyCode::Up | KeyCode::Char('k') => app.prev_image(),
                KeyCode::Left | KeyCode::Char('h') => {}
                KeyCode::Right | KeyCode::Char('l') => {}
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) -> Result<()> {
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
            let text = Span::styled(image_name, style);
            idx = idx + 1;
            ListItem::new(text)
        })
        .collect();
    let items = List::new(items);
    f.render_widget(items, rects[1]);
    let image_path = crate::image::get_image_path(app.items[app.selected_image_index].as_str());

    let binding = image_path?;
    let image_widget = crate::image::get_image_widget(
        binding.as_str(),
        rects[0].width as u32,
        rects[0].height as u32,
    )
    .context("Failed to get image widget")?;
    f.render_widget(image_widget, rects[0]);
    Ok(())
}
