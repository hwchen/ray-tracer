use vec3::{Point};

#[derive(Debug, Clone)]
pub struct Ray{
    pub origin: Point,
    pub direction: Point,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn point_at_parameter(&self, time: f64) -> Point {
        // TODO figure out uncloning
        self.origin.clone() + self.direction.clone() * time
    }
}
