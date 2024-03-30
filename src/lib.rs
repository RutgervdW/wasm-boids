mod utils;
use models::field::Field;
use wasm_bindgen::prelude::*;

mod models {
    pub mod boid;
    pub mod field;
}

#[wasm_bindgen]
pub struct BoidField {
    field: Field,
    buffer: Vec<f64>
}

#[wasm_bindgen]
impl BoidField {
    pub fn new(width: f64, height: f64, boid_count: usize) -> Self {
        let field = Field::new(width, height, boid_count);
        let buffer = Vec::new();
        BoidField { field, buffer }
    }

    pub fn buffer_pointer(&self) -> *const f64 {
        self.buffer.as_ptr()
    }

    pub fn tick(&mut self, width: f64, height: f64) {
        Field::resize(&mut self.field, width, height);
        Field::advance(&mut self.field, false, true);
        self.buffer.clear();

        for boid in &self.field.boids {
            self.buffer.push(boid.x);
            self.buffer.push(boid.y);
            self.buffer.push(boid.get_angle() / 360.0);
        }
    }
}