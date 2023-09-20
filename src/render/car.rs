use crate::traffic::{car::Car, Moving};
use macroquad::prelude::*;

pub fn render_car(car: &Car, texture: &Texture2D) {
    let full_width = texture.width();
    let full_height = texture.height();

    // If you want one-fourth horizontally
    let sprite_width = full_width / 4.0;
    let sprite_height = full_height;

    let x_offset = match car.moving {
        Moving::Up => 0.0,
        Moving::Left => sprite_width,
        Moving::Down => sprite_width * 2.0,
        Moving::Right => sprite_width * 3.0,
    };

    let src_rect = Rect::new(x_offset, 0.0, sprite_width, sprite_height);

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
