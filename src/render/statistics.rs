use crate::traffic::Statistics;
use macroquad::prelude::*;

use crate::config::{SECTOR_WIDTH, WINDOW_SIZE};

pub fn render_statistics(stats: &Statistics, max_cars: u32) {
    // Render a translucent rectangle as a backdrop
    draw_rectangle(
        0.0,
        0.0,
        WINDOW_SIZE as f32,
        WINDOW_SIZE as f32,
        Color::new(0.0, 0.0, 0.0, 0.5),
    );

    // Display title
    draw_text(
        "Final Statistics:",
        WINDOW_SIZE as f32 / 2.0 - 100.0,
        WINDOW_SIZE as f32 / 2.0 - 100.0,
        30.0,
        Color::new(1.0, 1.0, 1.0, 1.0),
    );

    // Display statistics
    draw_text(
        &format!("Max Vehicles: {} cars", max_cars),
        WINDOW_SIZE as f32 / 2.0 - 100.0,
        WINDOW_SIZE as f32 / 2.0 - 60.0,
        20.0,
        Color::new(1.0, 1.0, 1.0, 1.0),
    );
    draw_text(
        &format!(
            "Max Velocity: {} px/s",
            round_to_tenth(stats.max_velocity() * SECTOR_WIDTH)
        ),
        WINDOW_SIZE as f32 / 2.0 - 100.0,
        WINDOW_SIZE as f32 / 2.0 - 40.0,
        20.0,
        Color::new(1.0, 1.0, 1.0, 1.0),
    );
    draw_text(
        &format!(
            "Min Velocity: {} px/s",
            round_to_tenth(stats.min_velocity() * SECTOR_WIDTH)
        ),
        WINDOW_SIZE as f32 / 2.0 - 100.0,
        WINDOW_SIZE as f32 / 2.0 - 20.0,
        20.0,
        Color::new(1.0, 1.0, 1.0, 1.0),
    );
    draw_text(
        &format!("Max Time: {} s", round_to_tenth(stats.max_time())),
        WINDOW_SIZE as f32 / 2.0 - 100.0,
        WINDOW_SIZE as f32 / 2.0,
        20.0,
        Color::new(1.0, 1.0, 1.0, 1.0),
    );
    draw_text(
        &format!("Min Time: {} s", round_to_tenth(stats.min_time())),
        WINDOW_SIZE as f32 / 2.0 - 100.0,
        WINDOW_SIZE as f32 / 2.0 + 20.0,
        20.0,
        Color::new(1.0, 1.0, 1.0, 1.0),
    );
    draw_text(
        &format!("Close Calls: {}", stats.close_calls()),
        WINDOW_SIZE as f32 / 2.0 - 100.0,
        WINDOW_SIZE as f32 / 2.0 + 40.0,
        20.0,
        Color::new(1.0, 1.0, 1.0, 1.0),
    );
    draw_text(
        &format!("Collisions: {}", "0"),
        WINDOW_SIZE as f32 / 2.0 - 100.0,
        WINDOW_SIZE as f32 / 2.0 + 60.0,
        20.0,
        Color::new(1.0, 1.0, 1.0, 1.0),
    );
}

pub fn round_to_tenth(num: f32) -> f32 {
    (num * 10.0).round() / 10.0
}
