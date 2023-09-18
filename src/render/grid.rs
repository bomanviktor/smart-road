use crate::config::{ROAD_LINE_COLOR, ROAD_LINE_WIDTH, SECTOR_WIDTH, WINDOW_SIZE};

use macroquad::prelude::*;

pub fn render_grid() {
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
