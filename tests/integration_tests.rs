mod common;

mod test_config {}
mod test_state {
    use crate::common;
    use smart_road::traffic::state::*;

    #[test]
    fn test_constructor() {
        let state = State::default();

        // Statistics tests.
        assert_eq!(state.stats.min_time(), 0.0);
        assert_eq!(state.stats.max_time(), 0.0);
        assert_eq!(state.stats.max_velocity(), 0.0);
        assert_eq!(state.stats.close_calls(), 0);
        assert_eq!(state.stats.min_velocity(), 0.0);
        assert_eq!(state.stats.max_vehicles(), 0);
    }

    #[test]
    fn test_add_car() {
        let mut state = State::default();
        // Add cars
        for _ in 0..10 {
            state.add_car(Direction::North);
            state.add_car(Direction::East);
            state.add_car(Direction::South);
            state.add_car(Direction::West);
        }

        // Check if len of path is longer than 0
        // TODO: improve this test

        for road in state.roads {
            for cars in road.cars {
                assert_eq!(cars.len(), 1);
                for car in cars {
                    assert!(car.path.sectors.len() > 6);
                    assert!(car.path.sectors.len() < 14);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_simulation() {
        let state = common::setup().await;
        let total_cars = state
            .roads
            .iter()
            .flat_map(|r| r.cars.clone())
            .flatten()
            .count();

        assert_eq!(
            total_cars, 0,
            "Number of cars after simulation does not match expected value."
        );

        assert_eq!(state.stats.collisions(), 0);
    }
}
mod test_statistics {
    use macroquad::rand::gen_range;
    use smart_road::traffic::Statistics;

    #[test]
    fn max_vehicles() {
        let mut stats = Statistics::new();
        let n = gen_range(1, 100);

        stats.set_max_vehicles(n);
        assert_eq!(stats.max_vehicles(), n);

        stats.set_max_vehicles(n - 1);
        assert_eq!(stats.max_vehicles(), n);
    }

    #[test]
    fn min_time() {
        let mut stats = Statistics::new();
        let n = gen_range(1, 100) as f32;

        stats.set_min_time(n);
        assert_eq!(stats.min_time(), n);

        stats.set_min_time(n + 1.0);
        assert_eq!(stats.min_time(), n);

        stats.set_min_time(n - 1.0);
        assert_eq!(stats.min_time(), n - 1.0);
    }

    #[test]
    fn max_time() {
        let mut stats = Statistics::new();
        let n = gen_range(1, 100) as f32;

        stats.set_max_time(n);
        assert_eq!(stats.max_time(), n);

        stats.set_max_time(n - 1.0);
        assert_eq!(stats.max_time(), n);
    }

    #[test]
    fn min_velocity() {
        let mut stats = Statistics::new();
        let n = gen_range(1, 100) as f32;

        stats.set_min_velocity(n);
        assert_eq!(stats.min_velocity(), n);

        stats.set_min_velocity(n + 1.0);
        assert_eq!(stats.min_velocity(), n);

        stats.set_min_velocity(n - 1.0);
        assert_eq!(stats.min_velocity(), n - 1.0);
    }

    #[test]
    fn max_velocity() {
        let mut stats = Statistics::new();
        let n = gen_range(1, 100) as f32;

        stats.set_max_velocity(n);
        assert_eq!(stats.max_velocity(), n);

        stats.set_max_velocity(n - 1.0);
        assert_eq!(stats.max_velocity(), n);

        stats.set_max_velocity(n + 1.0);
        assert_eq!(stats.max_velocity(), n + 1.0);
    }

    #[test]
    fn collisions() {
        let mut stats = Statistics::new();

        let n = gen_range(1, 100) as u32;
        for _ in 0..=(n * 120) {
            stats.set_collisions();
        }
        assert_eq!(stats.collisions(), n);
    }
}
