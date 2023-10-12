#![allow(dead_code)]

use smart_road::traffic::State;
pub async fn setup() -> State {
    let mut state = State::new();
    for _ in 0..=100 {
        state.add_car_random();
        state.add_car_random();
        state.add_car_random();
        state.add_car_random();

        for _ in 0..100 {
            state.update();
        }

        state.add_car_random();
        state.add_car_random();
        state.add_car_random();
        state.add_car_random();

        for _ in 0..100 {
            state.update();
        }

        state.add_car_random();
        state.add_car_random();
        state.add_car_random();
        state.add_car_random();

        // Simulate traffic for a certain number of iterations
        let simulation_iterations = 2000;
        for _ in 0..simulation_iterations {
            state.update();
        }
    }
    return state;
}
