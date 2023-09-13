pub struct Statistics {
    max_vehicles: u32,
    max_velocity: f64,
    min_velocity: f64,
    max_time: f64,
    min_time: f64,
    close_calls: u32
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            max_vehicles: 0,
            max_velocity: 0.0,
            min_velocity: 0.0,
            max_time: 0.0,
            min_time: 0.0,
            close_calls: 0
        }
    }

    // Setters
    pub fn set_max_vehicles(&mut self, max_vehicles: u32) {
        self.max_vehicles = max_vehicles;
    }
    pub fn set_max_velocity(&mut self, max_velocity: f64) {
        self.max_velocity = max_velocity;
    }
    pub fn set_min_velocity(&mut self, min_velocity: f64) {
        self.min_velocity = min_velocity;
    }
    pub fn set_max_time(&mut self, max_time: f64) {
        self.max_time = max_time;
    }
    pub fn set_min_time(&mut self, min_time: f64) {
        self.min_time = min_time;
    }
    pub fn set_close_calls(&mut self, close_calls: u32) {
        self.close_calls = close_calls;
    }

    // Getters
    pub fn max_vehicles(&self) -> u32 {
        self.max_vehicles
    }
    pub fn max_velocity(&self) -> f64 {
        self.max_velocity
    }
    pub fn min_velocity(&self) -> f64 {
        self.min_velocity
    }
    pub fn max_time(&self) -> f64 {
        self.max_time
    }
    pub fn min_time(&self) -> f64 {
        self.min_time
    }
    pub fn close_calls(&self) -> u32 {
        self.close_calls
    }
}