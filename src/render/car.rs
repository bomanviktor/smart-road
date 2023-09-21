use crate::{
    config::SECTOR_WIDTH,
    traffic::{car::Car, Velocity},
};
use macroquad::prelude::*;

pub fn render_car(car: &Car, textures: &[Texture2D]) {
    let texture = &textures[car.sprite_index];
    let full_width = texture.width();

    let sprite_width = full_width / 4.0;

    let x_offset = match car.vel {
        Velocity::Up(_) => 0.0,
        Velocity::Left(_) => sprite_width,
        Velocity::Down(_) => sprite_width * 2.0,
        Velocity::Right(_) => sprite_width * 3.0,
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
