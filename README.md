# smart-road
[Project description](https://github.com/01-edu/public/blob/master/subjects/smart-road/README.md), completed during grit:lab in September 2023.

#### Authors:
- [Viktor Boman](https://github.com/bomanviktor)
- [Johannes Eckerman](https://github.com/jo-eman)
- [André Teetor](https://github.com/FinnTune)

### Run
Clone the repository, go into root and write:
`cargo run`
into the terminal.

### Controls
Generate a car in an available path by pressing:
- `↑` from the `South`
- `↓` from the `North`
- `←` from the `East`
- `→` from the `West`
- `R` continuously generate cars from random directions

Press `Esc` display statistics. Press `Esc` again to exit.


### Dependencies
```toml
macroquad = "0.4.2"
rand = "0.8.5"
```

[macroquad](https://crates.io/crates/macroquad) for rendering, [rand](https://crates.io/crates/rand) for everything that requires randomization.

## Description


### The grid
The screen is divided up into a 12x12 `grid` of `sectors` with equal size. The size of each sector is calculated by taking `WINDOW_SIZE` / 12. 
Each car will then be assigned a path on the grid, here's an example:
```rust
// Car generated from north, going left. Numbers being (x, y)
Sector::new(5, 0, Moving::Down),   // Entry
Sector::new(5, 6, Moving::Right),  // Turning-point
Sector::new(11, 6, Moving::Right), // Exit
```

### Collision detection
To avoid collision for our self-driving cars, we utilize sector scanning and ray casting.
Sector scanning is simply checking the sector ahead in the path, if there is currently a car in that sector, the car should stop. 
Ray casting is scanning for all cars in front of the car, within the `SCAN_DISTANCE`. Then take the closest car with a collision course and brake accordingly. 
The closer the scanned car is, the more the car will brake. 

Formula for braking:
```rust
let new_vel = self.vel - distance / SCAN_DISTANCE;
if new_vel > 0.0 {
    self.vel -= new_vel
}
```

### Acceleration
To accelerate the cars, we simply scan the sectors right in front of the cars, if there is no car within the `ACCELERATION_RANGE`, 
accelerate the car.

Formula for acceleration:
```rust
        let new_vel = (SPEED_LIMIT - self.vel) / FPS;
        if self.vel < SPEED_LIMIT {
            self.vel += new_vel;
        }
```


