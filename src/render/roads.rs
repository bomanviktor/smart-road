use crate::config::{ROAD_LINE_COLOR, ROAD_LINE_WIDTH, ROAD_WIDTH, SECTOR_WIDTH, WINDOW_SIZE};

use crate::render::textures::Textures;

use macroquad::prelude::*;

pub fn render_textured_roads(textures: &Textures) {
    // Tile size for background and road
    let bg_tile_size = WINDOW_SIZE as f32 / 10.0;
    let road_tile_size = ROAD_WIDTH / 10.0;

    // Draw the background as 10x10 tiles
    for x in 0..10 {
        for y in 0..10 {
            draw_texture_ex(
                &textures.bg,
                x as f32 * bg_tile_size,
                y as f32 * bg_tile_size,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(bg_tile_size, bg_tile_size)),
                    ..Default::default()
                },
            );
        }
    }

    // Calculate where the vertical and horizontal roads start
    let vertical_road_start_x = (WINDOW_SIZE as f32 - ROAD_WIDTH) / 2.0;
    let horizontal_road_start_y = (WINDOW_SIZE as f32 - ROAD_WIDTH) / 2.0;

    // Draw the vertical road as 10x10 tiles
    for x in 0..10 {
        for y in 0..WINDOW_SIZE / 10 {
            draw_texture_ex(
                &textures.road,
                vertical_road_start_x + x as f32 * road_tile_size,
                y as f32 * (WINDOW_SIZE as f32 / 10.0),
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(road_tile_size, WINDOW_SIZE as f32 / 10.0)),
                    ..Default::default()
                },
            );
        }
    }

    // Draw the horizontal road as 10x10 tiles
    for x in 0..WINDOW_SIZE / 10 {
        for y in 0..10 {
            draw_texture_ex(
                &textures.road,
                x as f32 * (WINDOW_SIZE as f32 / 10.0),
                horizontal_road_start_y + y as f32 * road_tile_size,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(WINDOW_SIZE as f32 / 10.0, road_tile_size)),
                    ..Default::default()
                },
            );
        }
    }

    // THIS IS FOR DEV PURPOSES ONLY
    // Vertical lines
    for i in 1..12 {
        if !(3..=9).contains(&i) {
            // Skip corners
            continue;
        }
        draw_rectangle(
            i as f32 * SECTOR_WIDTH,
            0.0, //25.0 + i as f32 * 100.0,
            ROAD_LINE_WIDTH,
            WINDOW_SIZE as f32,
            match i {
                3 | 9 => RED,
                6 => BLUE,
                _ => ROAD_LINE_COLOR,
            },
        );
    }

    // Horizontal lines
    for i in 1..12 {
        if !(3..=9).contains(&i) {
            // Skip corners
            continue;
        }
        draw_rectangle(
            0.0, // 25.0 + i as f32 * 100.0,
            i as f32 * SECTOR_WIDTH,
            WINDOW_SIZE as f32,
            ROAD_LINE_WIDTH,
            match i {
                3 | 9 => RED,
                6 => BLUE,
                _ => ROAD_LINE_COLOR,
            },
        );
    }
}
