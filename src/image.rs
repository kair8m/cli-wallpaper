use std::path::PathBuf;

use anyhow::*;
use image::{DynamicImage, GenericImageView, Rgba};
use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
};

fn image_fit_size(img: &DynamicImage, term_w: u32, term_h: u32) -> (u32, u32) {
    let (img_width, img_height) = img.dimensions();
    let (w, h) = get_dimensions(img_width, img_height, term_w, term_h);
    let h = if h == term_h { h - 1 } else { h };
    (w, h)
}

fn get_dimensions(width: u32, height: u32, bound_width: u32, bound_height: u32) -> (u32, u32) {
    let bound_height = 2 * bound_height;

    if width <= bound_width && height <= bound_height {
        return (width, std::cmp::max(1, height / 2 + height % 2));
    }

    let ratio = width * bound_height;
    let nratio = bound_width * height;

    let use_width = nratio <= ratio;
    let intermediate = if use_width {
        height * bound_width / width
    } else {
        width * bound_height / height
    };

    if use_width {
        (bound_width, std::cmp::max(1, intermediate / 2))
    } else {
        (intermediate, std::cmp::max(1, bound_height / 2))
    }
}

pub fn get_image_widget(image_path: &str, terminal_w: u32, terminal_h: u32) -> Result<Paragraph> {
    let mut img = image::open(image_path).context("Failed to open image")?;
    let (w, h) = image_fit_size(&img, terminal_w, terminal_h);
    img = img.resize_exact(w, h, image::imageops::Triangle);
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
