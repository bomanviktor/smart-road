use macroquad::prelude::*;

const CAR_SPRITES: [&str; 3] = [
    "assets/cars/Car_Sprite_Sheet_crop.png",
    "assets/cars/Audi_Sprite_Sheet_crop.png",
    "assets/cars/Viper_Sprite_Sheet_crop.png",
];

#[derive(PartialEq, Clone)]
pub struct Textures {
    pub bg: Texture2D,
    pub cars: Vec<Texture2D>,
}

impl Textures {
    pub async fn load() -> Self {
        let mut cars = Vec::new();
        for sprite in CAR_SPRITES {
            cars.push(load_texture(sprite).await.unwrap())
        }
        Self {
            bg: load_texture("assets/intersection.png").await.unwrap(),
            cars,
        }
    }
}
