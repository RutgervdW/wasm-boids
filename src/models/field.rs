use super::boid::Boid;

#[derive(Clone)]
pub struct Field {
    pub width: f64,
    pub height: f64,
    pub boids: Vec<Boid>
}

impl Field {
    pub fn new(width: f64, height: f64, boid_count: usize) -> Self {
        let boids = (0..boid_count).map(|_| Boid::new(width, height)).collect();
        Field { width, height, boids }
    }

    pub fn resize(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }

    pub fn advance(&mut self, bounce_off_walls: bool, wrap_around_edges: bool) {
        let mut temp_boids = self.boids.clone();

        for boid in &mut temp_boids {
            let (flock_x_vel, flock_y_vel) = self.flock(boid, 50.0, 0.0003);
            let (align_x_vel, align_y_vel) = self.align(boid, 50.0, 0.01);
            let (avoid_x_vel, avoid_y_vel) = self.avoid(boid, 20.0, 0.001);
            boid.x_vel += flock_x_vel + avoid_x_vel + align_x_vel;
            boid.y_vel += flock_y_vel + avoid_y_vel + align_y_vel;
            
            boid.move_forward();

            if bounce_off_walls {
                self.bounce_off_walls(boid);
            }
            if wrap_around_edges {
                self.wrap_around(boid);
            }
        }
        self.boids = temp_boids;
    }

    fn flock(&self, boid: &Boid, distance: f64, power: f64) -> (f64, f64) {
        let neighbors: Vec<&Boid> = self.boids.iter().filter(|other| boid.get_distance(other) < distance).collect();
        if neighbors.is_empty() {
            return (0.0, 0.0);
        }
        let mean_x: f64 = neighbors.iter().map(|b| b.x).sum::<f64>() / neighbors.len() as f64;
        let mean_y: f64 = neighbors.iter().map(|b| b.y).sum::<f64>() / neighbors.len() as f64;
        let delta_center_x = mean_x - boid.x;
        let delta_center_y = mean_y - boid.y;
        (delta_center_x * power, delta_center_y * power)
    }

    fn avoid(&self, boid: &Boid, distance: f64, power: f64) -> (f64, f64) {
        let neighbors: Vec<&Boid> = self.boids.iter().filter(|other| boid.get_distance(other) < distance).collect();
        if neighbors.is_empty() {
            return (0.0, 0.0);
        }
        let sum_closeness_x: f64 = neighbors.iter().map(|n| (boid.x - n.x) * (distance - boid.get_distance(n))).sum();
        let sum_closeness_y: f64 = neighbors.iter().map(|n| (boid.y - n.y) * (distance - boid.get_distance(n))).sum();
        (sum_closeness_x * power, sum_closeness_y * power)
    }

    fn align(&self, boid: &Boid, distance: f64, power: f64) -> (f64, f64) {
        let neighbors: Vec<&Boid> = self.boids.iter().filter(|other| boid.get_distance(other) < distance).collect();
        if neighbors.is_empty() {
            return (0.0, 0.0);
        }
        let mean_x_vel: f64 = neighbors.iter().map(|b| b.x_vel).sum::<f64>() / neighbors.len() as f64;
        let mean_y_vel: f64 = neighbors.iter().map(|b| b.y_vel).sum::<f64>() / neighbors.len() as f64;
        let d_x_vel = mean_x_vel - boid.x_vel;
        let d_y_vel = mean_y_vel - boid.y_vel;
        (d_x_vel * power, d_y_vel * power)
    }

    fn bounce_off_walls(&mut self, boid: &mut Boid) {
        let pad = 50.0;
        let turn = 0.5;
        if boid.x < pad {
            boid.x_vel += turn;
        }
        if boid.x > self.width - pad {
            boid.x_vel -= turn;
        }
        if boid.y < pad {
            boid.y_vel += turn;
        }
        if boid.y > self.height - pad {
            boid.y_vel -= turn;
        }
    }

    fn wrap_around(&mut self, boid: &mut Boid) {
        if boid.x < 0.0 {
            boid.x += self.width;
        }
        if boid.x > self.width {
            boid.x -= self.width;
        }
        if boid.y < 0.0 {
            boid.y += self.height;
        }
        if boid.y > self.height {
            boid.y -= self.height;
        }
    }
}