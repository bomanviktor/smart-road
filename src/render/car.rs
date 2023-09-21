use crate::config::SECTOR_WIDTH;
use crate::traffic::{car::Car, Moving};
use macroquad::prelude::*;

pub fn render_car(car: &Car, texture: &Texture2D) {
    // If you want one-fourth horizontally
    let sprite_width = texture.width() / 4.0;

    let x_offset = match car.moving {
        Moving::Up => 0.0,
        Moving::Left => sprite_width,
        Moving::Down => sprite_width * 2.0,
        Moving::Right => sprite_width * 3.0,
    };

    let src_rect = Rect::new(x_offset, 0.0, SECTOR_WIDTH, SECTOR_WIDTH);

    draw_texture_ex(
        texture,
        car.x,
        car.y,
        WHITE,
        DrawTextureParams {
            source: Some(src_rect),
            ..Default::default()
        },
    );
}
