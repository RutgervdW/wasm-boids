use rand::{thread_rng, Rng};

#[derive(Clone, Copy)]
pub struct Boid {
    pub x: f64,
    pub y: f64,
    pub x_vel: f64,
    pub y_vel: f64
}

impl Boid {
    pub fn new(width: f64, height: f64) -> Self {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..width);
        let y = rng.gen_range(0.0..height);
        let x_vel = rng.gen_range(-1.0..1.0);
        let y_vel = rng.gen_range(-1.0..1.0);
        Boid {x, y, x_vel, y_vel}
    }
    
    pub fn move_forward(&mut self) {
        self.x += self.x_vel;
        self.y += self.y_vel;

        let min_speed = 2.0;
        let max_speed = 5.0;

        let speed = self.get_speed();
        if speed > max_speed {
            self.x_vel = self.x_vel / speed * max_speed;
            self.y_vel = self.y_vel / speed * max_speed;
        }
        else if speed < min_speed {
            self.x_vel = self.x_vel / speed * min_speed;
            self.y_vel = self.y_vel / speed * min_speed;
        }

        if self.x_vel.is_nan() {
            self.x_vel = 0.0;
        }
        if self.y_vel.is_nan() {
            self.y_vel = 0.0;
        }
    }

    pub fn get_angle(&self) -> f64 {
        if self.x_vel.is_nan() || self.y.is_nan() {
            return 0.0;
        }
        if self.x_vel == 0.0 && self.y_vel == 0.0 {
            return 0.0;
        }
        let mut angle = (self.y_vel / self.x_vel).atan2(1.0) * 180.0 / std::f64::consts::PI - 90.0;

        if self.x_vel < 0.0 {
            angle += 180.0;
        }
        angle
    }

    pub fn get_speed(&self) -> f64 {
        (self.x_vel.powi(2) + self.y_vel.powi(2)).sqrt()
    }

    pub fn get_distance(&self, other: &Boid) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}