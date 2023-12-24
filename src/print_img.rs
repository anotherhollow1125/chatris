use super::game::{Game, MinoColors};
use anyhow::{bail, Context, Result};
use image::{ImageBuffer, Rgba};
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut};
use imageproc::rect::Rect;
use std::path::Path;

const BORDER_WIDTH: u32 = 2;
const SQUARE_MINIMUM_SIZE: u32 = BORDER_WIDTH + 1;
const WIDTH_MINIMUM: u32 = SQUARE_MINIMUM_SIZE * 12;
const BORDER_COLOR: Rgba<u8> = Rgba([0, 0, 0, 64]);
const BACKGROUND_COLOR: Rgba<u8> = Rgba([255, 255, 255, 255]);

pub enum ColorMode {
    Colorful,
    Monochrome,
}

impl ColorMode {
    fn encode_color(&self, color: MinoColors, contmino_color: MinoColors) -> Rgba<u8> {
        let color = match self {
            &ColorMode::Colorful => {
                let color = match color {
                    MinoColors::TRANS => MinoColors::WHITE,
                    MinoColors::GRAY => contmino_color,
                    c => c,
                };
                color.to_rgb()
            }
            &ColorMode::Monochrome => {
                let color = match color {
                    MinoColors::TRANS => MinoColors::WHITE,
                    MinoColors::GRAY => contmino_color,
                    _ => MinoColors::GRAY,
                };
                color.to_rgb()
            }
        };
        let v = color
            .into_iter()
            .map(|v| (v * 255.0).round() as u8)
            .collect::<Vec<_>>();
        Rgba([v[0], v[1], v[2], v[3]])
    }
}

pub fn draw_img(g: &Game, width: u32, path: impl AsRef<Path>, mode: ColorMode) -> Result<()> {
    if width < WIDTH_MINIMUM {
        bail!("width must be greater than {}", WIDTH_MINIMUM);
    }

    let square_size = width / 12;
    let height = square_size * 23;

    let mut img = ImageBuffer::from_pixel(width, height, BACKGROUND_COLOR);

    let field = g.rend_field();

    let contmino_color = g.get_contmino().get_color();
    for (y, row) in field.iter().enumerate() {
        for (x, block) in row.iter().enumerate() {
            let color = block.get_color();
            let color = mode.encode_color(color, contmino_color);
            let x = (((x as u32) + 1) * square_size) as i32;
            let y = (((y as u32) + 1) * square_size) as i32;
            let rect = Rect::at(x, y).of_size(square_size, square_size);
            draw_filled_rect_mut(&mut img, rect, color);
        }
    }

    let start_x = square_size as f32;
    let end_x = (square_size * 11) as f32;
    for y in 1..=22 {
        let y = (y * square_size) as f32;
        draw_line_segment_mut(&mut img, (start_x, y), (end_x, y), BORDER_COLOR);
    }

    let start_y = square_size as f32;
    let end_y = (square_size * 22) as f32;
    for x in 1..=11 {
        let x = (x * square_size) as f32;
        draw_line_segment_mut(&mut img, (x, start_y), (x, end_y), BORDER_COLOR);
    }

    img.save(&path)
        .with_context(|| format!("@{}:{}", file!(), line!()))?;

    Ok(())
}
