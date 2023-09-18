pub mod config {
    use macroquad::color::Color;
    use macroquad::window::Conf;

    pub const WINDOW_SIZE: i32 = 1000;
    pub const ROAD_LINE_WIDTH: f32 = 5.0;

    pub const ROAD_LINE_LENGTH: f32 = 50.0;

    pub const ROAD_LINE_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.0);
    pub const ROAD_WIDTH: f32 = WINDOW_SIZE as f32 / 2.0;
    pub const SECTOR_WIDTH: f32 = WINDOW_SIZE as f32 / 12.0;
    pub fn window_conf() -> Conf {
        Conf {
            window_title: "Smart-Road | Grit:lab".to_owned(),
            window_width: WINDOW_SIZE,
            window_height: WINDOW_SIZE,
            window_resizable: false,
            ..Default::default()
        }
    }
}

pub mod controls {
    use crate::traffic::{Direction, State};
    use macroquad::prelude::*;

    pub fn handle_input(state: &mut State) {
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }

        if is_key_pressed(KeyCode::Up) {
            state.add_car(Direction::South);
        }

        if is_key_pressed(KeyCode::Down) {
            state.add_car(Direction::North);
        }

        if is_key_pressed(KeyCode::Right) {
            state.add_car(Direction::West);
        }

        if is_key_pressed(KeyCode::Left) {
            state.add_car(Direction::East);
        }

        if is_key_pressed(KeyCode::R) {
            state.add_car_random();
        }
    }
}

pub mod traffic {
    pub mod car;
    pub mod grid;
    pub mod lane;
    pub mod path;
    pub mod state;
    pub mod statistics;

    pub use car::*;
    pub use grid::*;
    pub use path::*;
    pub use path::*;
    pub use state::{Direction, State};
    pub use statistics::*;
}

pub mod render {
    pub mod roads;

    pub use roads::render_textured_roads;

    pub mod textures;

    pub use textures::Textures;
}
