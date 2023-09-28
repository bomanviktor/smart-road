use macroquad::prelude::*;

use crate::config::SECTOR_WIDTH;
use crate::traffic::{car::Car, Moving};

pub fn render_car(car: &Car, texture: &Texture2D) {
    // Assuming the new sprite sheet has four car images arranged horizontally
    let sprite_width = texture.width() / 4.0;
    let sprite_height = texture.height(); // Assuming all car sprites have the same height

    // Determine which sprite to use based on the car's direction
    let x_offset = match car.moving {
        Moving::Up => 0.0,
        Moving::Left => sprite_width,
        Moving::Down => sprite_width * 2.0,
        Moving::Right => sprite_width * 3.0,
    };

    let src_rect = Rect::new(x_offset, 0.0, sprite_width, sprite_height);

    // Scale down by 80%
    let scaled_size = SECTOR_WIDTH * 0.9;

    // Calculate the position to center the car in the sector
    let center_x = car.x + (SECTOR_WIDTH - scaled_size) / 2.0;
    let center_y = car.y + (SECTOR_WIDTH - scaled_size) / 2.0;

    draw_texture_ex(
        texture,
        center_x,
        center_y,
        WHITE,
        DrawTextureParams {
            source: Some(src_rect),
            dest_size: Some(Vec2::new(scaled_size, scaled_size)), // Set to 80% of the sector size
            ..Default::default()
        },
    );
}
