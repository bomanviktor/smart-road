use crate::traffic::Statistics;
use macroquad::prelude::*;

use crate::config::{SECTOR_WIDTH, WINDOW_SIZE};
use crate::render::{FONT_SIZE, TITLE_SIZE};

const TEXT_X_POS: f32 = CENTER_Y - 100.0;
const CENTER_Y: f32 = WINDOW_SIZE as f32 / 2.0;

pub fn render_statistics(stats: &Statistics) {
    // Render a translucent rectangle as a backdrop
    draw_rectangle(0.0, 0.0, WINDOW_SIZE as f32, WINDOW_SIZE as f32, BLACK);

    // Display title
    draw_text(
        "Final Statistics:",
        TEXT_X_POS,
        CENTER_Y - 80.0,
        TITLE_SIZE,
        WHITE,
    );

    // Display statistics
    draw_text(
        &format!("Max Vehicles: {} cars", stats.max_vehicles()),
        TEXT_X_POS,
        CENTER_Y - 60.0,
        FONT_SIZE,
        WHITE,
    );
    draw_text(
        &format!(
            "Max Velocity: {} px/s",
            round_to_tenth(stats.max_velocity() * SECTOR_WIDTH)
        ),
        TEXT_X_POS,
        CENTER_Y - 40.0,
        FONT_SIZE,
        WHITE,
    );
    draw_text(
        &format!(
            "Min Velocity: {} px/s",
            round_to_tenth(stats.min_velocity() * SECTOR_WIDTH)
        ),
        TEXT_X_POS,
        CENTER_Y - 20.0,
        FONT_SIZE,
        WHITE,
    );
    draw_text(
        &format!("Max Time: {} s", round_to_tenth(stats.max_time())),
        TEXT_X_POS,
        CENTER_Y,
        FONT_SIZE,
        WHITE,
    );
    draw_text(
        &format!("Min Time: {} s", round_to_tenth(stats.min_time())),
        TEXT_X_POS,
        CENTER_Y + 20.0,
        FONT_SIZE,
        WHITE,
    );
    draw_text(
        &format!("Close Calls: {}", stats.close_calls()),
        TEXT_X_POS,
        CENTER_Y + 40.0,
        FONT_SIZE,
        WHITE,
    );
    draw_text(
        &format!("Collisions: {}", stats.collisions()),
        TEXT_X_POS,
        CENTER_Y + 60.0,
        FONT_SIZE,
        WHITE,
    );
}

pub fn round_to_tenth(num: f32) -> f32 {
    (num * 10.0).round() / 10.0
}
