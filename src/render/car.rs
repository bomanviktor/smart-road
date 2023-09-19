use crate::traffic::{car::Car, Velocity};
use macroquad::prelude::*;

pub fn render_car(car: &Car, texture: &Texture2D) {
    let full_width = texture.width();
    let full_height = texture.height();

    // If you want one-fourth horizontally
    let sprite_width = full_width / 4.0;
    let sprite_height = full_height;

    let x_offset = match car.vel {
        Velocity::Up(_) => 0.0,
        Velocity::Left(_) => sprite_width,
        Velocity::Down(_) => sprite_width * 2.0,
        Velocity::Right(_) => sprite_width * 3.0,
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
