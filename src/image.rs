use std::path::PathBuf;

use anyhow::*;
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
    let imgbuf = img.to_rgba8();
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

pub fn get_image_path(image_name: &str) -> String {
    let mut image_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    image_dir.push("images");
    image_dir.push(image_name.to_string() + "_preview.jpg");
    image_dir.to_str().unwrap().to_string()
}
