pub mod config {
    use macroquad::color::Color;
    use macroquad::window::Conf;

    pub const WINDOW_SIZE: i32 = 1000;
    pub const ROAD_LINE_WIDTH: f32 = 5.0;

    pub const ROAD_LINE_LENGTH: f32 = 50.0;

    pub const TILE_SIZE: f32 = 10.0;

    // Tile size for background and road
    pub const BG_TILE_SIZE: f32 = WINDOW_SIZE as f32 / TILE_SIZE;
    pub const ROAD_TILE_SIZE: f32 = ROAD_WIDTH / TILE_SIZE;

    pub const ROAD_LINE_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.0);
    pub const ROAD_WIDTH: f32 = WINDOW_SIZE as f32 / 2.0;
    pub const SECTOR_WIDTH: f32 = WINDOW_SIZE as f32 / 12.0;

    pub const SPRITE_WIDTH: f32 = SECTOR_WIDTH / 4.0;

    pub const SCAN_AREA: usize = (SECTOR_WIDTH / 1.5) as usize;

    pub const FPS: u64 = 60;

    pub const MAX_VELOCITY: f32 = (SECTOR_WIDTH * 2.0) / FPS as f32;
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
    use macroquad::prelude::*;

    use crate::traffic::{Direction, State};

    pub fn handle_input(state: &mut State) {
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }

        if is_key_pressed(KeyCode::P) {
            state.paused = !state.paused;
        }

        if is_key_pressed(KeyCode::Up) {
            state.add_car(Direction::South);
            state.random = false;
        }

        if is_key_pressed(KeyCode::Down) {
            state.add_car(Direction::North);
            state.random = false;
        }

        if is_key_pressed(KeyCode::Right) {
            state.add_car(Direction::West);
            state.random = false;
        }

        if is_key_pressed(KeyCode::Left) {
            state.add_car(Direction::East);
            state.random = false;
        }

        if is_key_pressed(KeyCode::R) {
            state.random = !state.random;
        }
    }
}

pub mod traffic {
    pub use car::*;
    pub use grid::*;
    pub use path::*;
    pub use path::*;
    pub use state::{Direction, State};
    pub use statistics::*;

    pub mod car;
    pub mod grid;
    pub mod path;
    pub mod road;
    pub mod state;
    pub mod statistics;
}

pub mod render {
    pub use car::render_car;
    pub use grid::render_grid;
    pub use roads::render_textured_roads;
    pub use textures::Textures;

    pub mod roads;

    pub mod textures;

    pub mod grid;

    pub mod car;
}
