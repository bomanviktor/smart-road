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

    #[test]
    fn test_car_move() {
        use smart_road::traffic::Car;
        use smart_road::traffic::Direction;
        use smart_road::traffic::Turning;
        use smart_road::traffic::Velocity;

        for direction in &[
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            for turning in &[Turning::Left, Turning::Straight, Turning::Right] {
                let mut car = Car::new(direction.clone(), turning.clone());
                let initial_x = car.x;
                let initial_y = car.y;

                car.move_car();

                match direction {
                    Direction::North => {
                        if *turning == Turning::Straight {
                            assert!(car.y > initial_y);
                            assert_eq!(car.x, initial_x);
                            assert_eq!(car.vel, Velocity::Down(car.get_velocity()));
                        }
                    }
                    Direction::East => {
                        if *turning == Turning::Straight {
                            // X should increase for East, Y should remain the same
                            assert!(car.x < initial_x);
                            assert_eq!(car.y, initial_y);
                            assert_eq!(car.vel, Velocity::Left(car.get_velocity()));
                        }
                    }
                    Direction::South => {
                        if *turning == Turning::Straight {
                            // Y should increase for South, X should remain the same
                            assert!(car.y < initial_y);
                            assert_eq!(car.x, initial_x);
                            assert_eq!(car.vel, Velocity::Up(car.get_velocity()));
                        }
                    }
                    Direction::West => {
                        if *turning == Turning::Straight {
                            assert!(car.x > initial_x);
                            assert_eq!(car.y, initial_y);
                            assert_eq!(car.vel, Velocity::Right(car.get_velocity()));
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_add_random() {
        let mut state = State::default();
        // Add cars
        for _ in 0..20 {
            state.add_car_random();
        }
        let mut cars = Vec::new();

        for lane in state.roads {
            cars.push(lane.cars);
        }

        // Check if just the right amount of cars were added
        assert_eq!(cars.iter().flatten().count(), 12);

        // Check if len of path is longer than 0
        // TODO: improve this test
        for cars in cars.iter().flatten() {
            for car in cars {
                assert!(car.path.sectors.len() > 6);
                assert!(car.path.sectors.len() < 14);
            }
        }
    }

    #[test]
    fn test_update() {
        let mut state = State::default();
        state.update();
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
}
