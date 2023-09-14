use macroquad::color::Color;
use macroquad::window::Conf;
pub const WINDOW_SIZE: i32 = 1000;
pub const ROAD_LINE_WIDTH: f32 = 5.0;

pub const ROAD_LINE_LENGTH: f32 = 50.0;

pub const ROAD_COLOR: Color = Color::new(0.267, 0.294, 0.325, 1.0);
pub const ROAD_LINE_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.0);
pub const ROAD_WIDTH: f32 = WINDOW_SIZE as f32 / 2.0;
pub const SECTOR_WIDTH: f32 = ROAD_WIDTH / 6.0;
pub fn window_conf() -> Conf {
    Conf {
        window_title: "Smart-Road | Grit:lab".to_owned(),
        window_width: WINDOW_SIZE,
        window_height: WINDOW_SIZE,
        window_resizable: false,
        ..Default::default()
    }
}
