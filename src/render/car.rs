use crate::traffic::Model;
use crate::{
    config::SECTOR_WIDTH,
    traffic::{car::Car, Moving},
};
use macroquad::prelude::*;
pub fn render_car(car: &Car, textures: &[Texture2D]) {
    let texture = match car.model {
        Model::Standard => &textures[0],
        Model::Audi => &textures[1],
        Model::Viper => &textures[2],
    };
    // Determine which sprite to use based on the car's direction
    let rotation: f32 = match car.moving {
        Moving::Up => 0.0,
        Moving::Left => -90.0,
        Moving::Down => -180.0,
        Moving::Right => -270.0,
    };
    let src_rect = Rect::new(0.0, 0.0, SECTOR_WIDTH, SECTOR_WIDTH);
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
            rotation: rotation.to_radians(),
            dest_size: Some(Vec2::new(scaled_size, scaled_size)), // Set to 80% of the sector size
            ..Default::default()
        },
    );
}
