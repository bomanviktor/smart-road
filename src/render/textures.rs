use macroquad::prelude::*;

pub struct Textures {
    pub road: Texture2D,
    pub bg: Texture2D,
}

impl Textures {
    pub async fn load() -> Self {
        Self {
            road: macroquad::texture::load_texture("assets/asphalt/ground_asphalt_synth_08.png")
                .await
                .unwrap(),
            bg: macroquad::texture::load_texture("assets/RockBG.png")
                .await
                .unwrap(),
        }
    }
}
