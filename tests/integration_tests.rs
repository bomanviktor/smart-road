mod common;

mod test_config {}
mod test_state {
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

        for i in 0..=3 {
            assert_eq!(state.lanes[i].cars.len(), 10);
        }

        // Check if len of path is longer than 0
        // TODO: improve this test
        for i in 0..=3 {
            for j in 0..10 {
                assert!(state.lanes[i].cars[j].path.sectors.len() > 1);
                assert!(state.lanes[i].cars[j].path.sectors.len() < 14);
            }
        }
    }

    #[test]
    fn test_add_random() {
        let mut state = State::default();
        // Add cars
        for _ in 0..10 {
            state.add_car_random();
        }
        let mut cars = Vec::new();

        for lane in state.lanes {
            cars.push(lane.cars);
        }

        // Check if len of path is longer than 0
        // TODO: improve this test
        for car in cars.iter().flatten() {
            assert!(car.path.sectors.len() > 1);
            assert!(car.path.sectors.len() < 14);
        }
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
        let n = gen_range(1, 100) as f64;

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
        let n = gen_range(1, 100) as f64;

        stats.set_max_time(n);
        assert_eq!(stats.max_time(), n);

        stats.set_max_time(n - 1.0);
        assert_eq!(stats.max_time(), n);
    }

    #[test]
    fn min_velocity() {
        let mut stats = Statistics::new();
        let n = gen_range(1, 100) as f64;

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
        let n = gen_range(1, 100) as f64;

        stats.set_max_velocity(n);
        assert_eq!(stats.max_velocity(), n);

        stats.set_max_velocity(n - 1.0);
        assert_eq!(stats.max_velocity(), n);

        stats.set_max_velocity(n + 1.0);
        assert_eq!(stats.max_velocity(), n + 1.0);
    }
}
