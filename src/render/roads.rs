use crate::config::{ROAD_COLOR, ROAD_LINE_COLOR, ROAD_LINE_LENGTH, ROAD_LINE_WIDTH, ROAD_WIDTH, WINDOW_SIZE};

use macroquad::prelude::*;
pub fn render_roads() {
    // Vertical road
    draw_rectangle(
        (WINDOW_SIZE as f32 - ROAD_WIDTH) / 2.0,
        0.0,
        ROAD_WIDTH,
        WINDOW_SIZE as f32,
        ROAD_COLOR,
    );

    // Horizontal road
    draw_rectangle(
        0.0,
        (WINDOW_SIZE as f32 - ROAD_WIDTH) / 2.0,
        WINDOW_SIZE as f32,
        ROAD_WIDTH,
        ROAD_COLOR,
    );

    for i in 0..(WINDOW_SIZE / 100) {
        // Vertical road lines
        for j in 1..(6) {
            draw_rectangle(
                (ROAD_WIDTH / 2.0)  + (j as f32 * 80.0) + ROAD_LINE_WIDTH * (j - 1) as f32,
                25.0 + i as f32 * 100.0,
                ROAD_LINE_WIDTH,
                ROAD_LINE_LENGTH,
                ROAD_LINE_COLOR,
            );
        }

        // Horizontal road lines
        for j in 1..(6) {
            draw_rectangle(
                25.0 + i as f32 * 100.0,
                (ROAD_WIDTH / 2.0) + (j as f32 * 80.0) + ROAD_LINE_WIDTH * (j - 1) as f32,
                ROAD_LINE_LENGTH,
                ROAD_LINE_WIDTH,
                ROAD_LINE_COLOR,
            );
        }
    }

    // Remove lines in the center
    draw_rectangle(
        (WINDOW_SIZE as f32 - ROAD_WIDTH) / 2.0,
        (WINDOW_SIZE as f32 - ROAD_WIDTH) / 2.0,
        ROAD_WIDTH,
        ROAD_WIDTH,
        ROAD_COLOR,
    );
}