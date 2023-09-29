use macroquad::prelude::*;

const CAR_SPRITES: [&str; 4] = [
    "assets/cars/audi.png",
    "assets/cars/taxi.png",
    "assets/cars/minitruck.png",
    "assets/cars/viper.png",
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
