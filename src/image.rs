use std::{fs::read_dir, path::PathBuf};

use anyhow::{Context, Result};
use image::Rgba;
use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn get_image_widget(image_path: &str, terminal_w: u32, terminal_h: u32) -> Result<Paragraph> {
    let mut img = image::open(image_path).context("Failed to open image")?;
    img = img.resize_exact(terminal_w, terminal_h, image::imageops::Triangle);
    let imgbuf = img.unsharpen(8.0, 10).to_rgba8();
    let (width, height) = imgbuf.dimensions();
    let mut span_vec = vec![];
    for y in 0..height {
        let mut line = vec![];
        for x in 0..width {
            let Rgba(data) = &imgbuf.get_pixel(x, y);
            // check alpha
            if data[3] == 0 {
                line.push(Span::from(" "));
            } else {
                line.push(Span::styled(
                    " ",
                    Style::default().bg(Color::Rgb(data[0], data[1], data[2])),
                ));
            }
        }
        span_vec.push(Spans::from(line));
    }
    let res = Paragraph::new(span_vec)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
        .alignment(Alignment::Center);
    Ok(res)
}

pub fn list_images() -> Result<Vec<String>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("images");
    let image_files = read_dir(path).context("couldn't find images in filesystem")?;
    let result = image_files
        .into_iter()
        .filter(|file| match file {
            Err(_) => false,
            Ok(dir_entry) => match dir_entry.path().to_str() {
                Some(path) => !path.contains("preview"),
                None => false,
            },
        })
        .map(|file| -> Result<String> {
            let res = file
                .context("Image access failed")?
                .path()
                .with_extension("")
                .file_name()
                .context("invalid path ends with '..'")?
                .to_str()
                .context("invalid os string")?
                .to_owned();
            Ok(res)
        })
        .filter_map(|f| f.ok())
        .collect();
    Ok(result)
}

pub fn get_image_path(image_name: &str) -> Result<String> {
    let mut image_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    image_dir.push("images");
    image_dir.push(image_name.to_string() + "_preview.jpg");
    Ok(image_dir.to_str().context("Invalid path")?.to_string())
}
