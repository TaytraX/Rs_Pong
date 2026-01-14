use glam::Vec2;
use crate::render_backend::buffer::Vertex;

#[derive(Clone)]
pub struct Collider {
    pub half_size: Vec2,
    pub color: [f32; 3],
}

impl Collider {
    pub fn new(half_size: Vec2, colors: [f32; 3]) -> Self {
        Self { half_size, color: colors }
    }

    pub fn to_vertices(&self) -> [Vertex; 4] {
        let w = self.half_size.x;
        let h = self.half_size.y;

        [
            Vertex { position: [-w, -h, 0.0], color: self.color },
            Vertex { position: [ w, -h, 0.0], color: self.color },
            Vertex { position: [ w,  h, 0.0], color: self.color },
            Vertex { position: [-w,  h, 0.0], color: self.color }
        ]
    }

    pub fn collides_with(&self, self_pos: Vec2, other: &Collider, other_pos: Vec2) -> bool {
        let self_min = self_pos - self.half_size;
        let self_max = self_pos + self.half_size;
        let other_min = other_pos - other.half_size;
        let other_max = other_pos + other.half_size;

        self_max.x >= other_min.x
            && self_min.x <= other_max.x
            && self_max.y >= other_min.y
            && self_min.y <= other_max.y
    }
}