mod common;

mod test_main {
    use crate::*;
}
mod test_state {
    use smart_road::traffic::state::*;
    #[test]
    fn test_constructor() {
        let state = State::new();

        // Statistics tests.
        assert_eq!(state.stats.min_time(), 0.0);
        assert_eq!(state.stats.max_time(), 0.0);
        assert_eq!(state.stats.max_velocity(), 0.0);
        assert_eq!(state.stats.close_calls(), 0);
        assert_eq!(state.stats.min_velocity(), 0.0);
        assert_eq!(state.stats.max_vehicles(), 0);

        // Other tests here
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
