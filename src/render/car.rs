use macroquad::prelude::*;

use crate::traffic::Model;
use crate::{
    config::SECTOR_WIDTH,
    traffic::{car::Car, Velocity},
};

pub fn render_car(car: &Car, textures: &[Texture2D]) {
    let texture = match car.model {
        Model::Standard => &textures[0],
        Model::Audi => &textures[1],
        Model::Truck => &textures[2],
        Model::Van => &textures[3],
        Model::Taxi => &textures[4],
        Model::Viper => &textures[5],
    };

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
