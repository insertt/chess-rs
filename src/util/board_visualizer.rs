use std::collections::HashMap;

use image::{ImageBuffer, RgbaImage, Rgba, ImageFormat, Pixel};
use imageproc::rect::Rect;
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use rusttype::{Font, Scale};

use crate::chess::pieces::{Type};
use crate::chess::board::{Board, Square, Color as PieceColor};

pub type Color = Rgba<u8>;

pub struct Config {
    pub tile_size: usize,
    pub bottom_fill_size: usize,
    pub bottom_fill_color: Color,
    pub light_tile_color: Color,
    pub dark_tile_color: Color,
    pub light_tile_color_highlighted: Color,
    pub dark_tile_color_highlighted: Color,
    pub text_on_light_color: Color,
    pub text_on_dark_color: Color,
    pub text_font: Vec<u8>,
    pub text_font_size: usize,
    pub pieces_image: Vec<u8>,
    pub pieces_image_format: ImageFormat,
    pub pieces_mappings: HashMap<PieceColor, HashMap<Type, (u32, u32)>>,
    pub piece_size: usize,
}

pub struct BoardVisualizer {
    config: Config,
    font: Font<'static>,
    scale: Scale,
    piece_image: RgbaImage,
}

const BOARD_SIZE: usize = 8;

impl BoardVisualizer {
    pub fn new(config: Config) -> Self {
        Self {
            font: Font::try_from_vec(config.text_font.clone()).unwrap(),
            scale: Scale::uniform(config.text_font_size as f32),
            piece_image: image::load_from_memory_with_format(&config.pieces_image as &[u8], config.pieces_image_format).unwrap().to_rgba(),
            config,
        }
    }

    pub fn visualize(&self, board: &Board) {
        let mut image: RgbaImage = ImageBuffer::from_fn(
            (self.config.tile_size * 8) as u32,
            (self.config.tile_size * 8 + self.config.bottom_fill_size) as u32,
            |_, _| self.config.bottom_fill_color,
        );

        for file in 1..9 {
            for rank in 1..9 {
                let square = Square::new(file, rank);

                // Tile position in pixels
                let tile_start_x = (file - 1) as usize * self.config.tile_size;
                let tile_start_y = (self.config.bottom_fill_size / 2) + (BOARD_SIZE - rank as usize) * self.config.tile_size;

                // Draw tile colors
                let color = if board.highlighted_squares.contains(&square) {
                    if square.is_light() { self.config.light_tile_color_highlighted } else { self.config.dark_tile_color_highlighted }
                } else {
                    if square.is_light() { self.config.light_tile_color } else { self.config.dark_tile_color }
                };

                let rect = Rect::at(tile_start_x as i32, tile_start_y as i32)
                    .of_size(self.config.tile_size as u32, self.config.tile_size as u32);

                draw_filled_rect_mut(&mut image, rect, color);

                let text_color = if square.is_light() { self.config.text_on_light_color } else { self.config.text_on_dark_color };

                if file == 1 {
                    // Draw rank number
                    draw_text_mut(
                        &mut image,
                        text_color,
                        (tile_start_x + 1) as u32,
                        (tile_start_y + self.config.tile_size / 2 - self.config.text_font_size / 2) as u32,
                        self.scale,
                        &self.font,
                        square.rank_number.to_string().as_str(),
                    )
                }

                if rank == 1 {
                    let glyph = self.font.glyph(square.get_file_as_letter());
                    let glyph_width = glyph.scaled(self.scale).h_metrics().advance_width as usize;

                    // Draw file letter
                    draw_text_mut(
                        &mut image,
                        text_color,
                        (tile_start_x + self.config.tile_size / 2 - glyph_width / 2) as u32,
                        (tile_start_y + self.config.tile_size + 2) as u32,
                        self.scale,
                        &self.font,
                        square.get_file_as_letter().to_string().as_str(),
                    );
                }


                // Draw a piece
                if let Some(piece) = board.get_piece(square) {
                    let (piece_x, piece_y) = self.config.pieces_mappings[&piece.color][&piece.piece_type];
                    let padding = (self.config.tile_size - self.config.piece_size) / 2;

                    BoardVisualizer::draw_image(
                        &mut image,
                        (tile_start_x + padding) as u32,
                        (tile_start_y + padding) as u32,
                        &self.piece_image,
                        (piece_x * self.config.piece_size as u32) as u32,
                        (piece_y * self.config.piece_size as u32) as u32,
                        self.config.piece_size as u32,
                        self.config.piece_size as u32,
                    );
                }
            }
        }

        image.save("test.png").unwrap();
    }

    fn draw_image(image: &mut RgbaImage, x: u32, y: u32, image_from: &RgbaImage, from_x: u32, from_y: u32, width: u32, height: u32) {
        for shift_x in 0..width {
            for shift_y in 0..height {
                image.get_pixel_mut(x + shift_x, y + shift_y).blend(image_from.get_pixel(from_x + shift_x, from_y + shift_y));
            }
        }
    }
}