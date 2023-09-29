use crate::traffic::Model;
use crate::{
    config::SECTOR_WIDTH,
    traffic::{car::Car, Moving},
};
use macroquad::prelude::*;

pub fn render_car(car: &Car, textures: &[Texture2D]) {
    let texture = match car.model {
        Model::Viper => &textures[0],
        Model::Audi => &textures[1],
        Model::Truck => &textures[2],
        Model::Taxi => &textures[3],
    };

    let sprite_width = 218.0;

    // Determine which sprite to use based on the car's direction
    let x_offset = match car.moving {
        Moving::Up => 0.0,
        Moving::Left => sprite_width,
        Moving::Down => sprite_width * 2.0,
        Moving::Right => sprite_width * 3.0,
    };

    let src_rect = Rect::new(x_offset, 0.0, sprite_width, sprite_width); // Use actual sprite dimensions

    // Calculate scaling factor to fit the sprite into the sector square
    let scale_factor = SECTOR_WIDTH / sprite_width;

    // Calculate the position to center the car in the sector
    let center_x = car.x + (SECTOR_WIDTH - (sprite_width * scale_factor)) / 2.0;
    let center_y = car.y + (SECTOR_WIDTH - (sprite_width * scale_factor)) / 2.0;

    draw_texture_ex(
        texture,
        center_x,
        center_y,
        WHITE,
        DrawTextureParams {
            source: Some(src_rect),
            dest_size: Some(Vec2::new(
                sprite_width * scale_factor,
                sprite_width * scale_factor,
            )), // Scale to fit sector
            ..Default::default()
        },
    );
}
