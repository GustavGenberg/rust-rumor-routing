use std::cmp::{min, max};

#[derive(Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn get_distance_to(&self, position: &Position) -> f64 {
        let dx = max(&self.x, &position.x) - min(&self.x, &position.x);
        let dy = max(&self.y, &position.y) - min(&self.y, &position.y);

        return (dx.pow(2) as f64 + dy.pow(2) as f64).sqrt();
    }
    
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y
        }
    }
}