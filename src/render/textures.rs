use macroquad::prelude::*;

#[derive(PartialEq, Clone)]
pub struct Textures {
    pub road: Texture2D,
    pub bg: Texture2D,
    pub cars: Vec<Texture2D>,
}

impl Textures {
    pub async fn load() -> Self {
        let mut cars = Vec::new();
        // Load multiple car textures, you can add more paths as needed
        let car_paths = vec![
            "assets/cars/Car_Sprite_Sheet_Resized.png",
            "assets/cars/Audi_Sprite_Sheet.png",
            "assets/cars/Mini_Truck_Sprite_Sheet.png",
            "assets/cars/Mini_Van_Sprite_Sheet.png",
            "assets/cars/Taxi_Sprite_Sheet.png",
            "assets/cars/Viper_Sprite_Sheet.png",
        ];

        for path in car_paths {
            cars.push(macroquad::texture::load_texture(path).await.unwrap());
        }
        Self {
            road: macroquad::texture::load_texture("assets/asphalt/ground_asphalt_synth_08.png")
                .await
                .unwrap(),
            bg: macroquad::texture::load_texture("assets/RockBG.png")
                .await
                .unwrap(),
            cars,
        }
    }
}
