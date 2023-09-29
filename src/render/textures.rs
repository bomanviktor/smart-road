use macroquad::prelude::*;

const CAR_SPRITES: [&str; 6] = [
    "assets/cars/Car_Sprite_Sheet_Resized.png",
    "assets/cars/Audi_Sprite_Sheet.png",
    "assets/cars/Mini_Truck_Sprite_Sheet.png",
    "assets/cars/Mini_Van_Sprite_Sheet.png",
    "assets/cars/Taxi_Sprite_Sheet.png",
    "assets/cars/Viper_Sprite_Sheet.png",
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
                .await
                .unwrap(),
            cars,
        }
    }
}
