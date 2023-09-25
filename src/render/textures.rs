use macroquad::prelude::*;

#[derive(PartialEq, Clone)]
pub struct Textures {
    pub bg: Texture2D,
    pub car: Texture2D,
}

impl Textures {
    pub async fn load() -> Self {
        Self {
            bg: load_texture("assets/intersection.png").await.unwrap(),
            car: load_texture("assets/cars/Car_Sprite_Sheet_Resized.png")
                .await
                .unwrap(),
        }
    }
}
