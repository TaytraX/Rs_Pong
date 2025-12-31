use glam::Vec2;
use crate::box_collider::Collider;

pub struct RigidBody {
    pub mass: f32,
    pub restitution: f32
}

pub struct Componant {
    pub position: Vec2,
    pub velocity: Vec2,
    pub collider: Collider,
}

impl Componant {
    pub fn set_velocity(&mut self, velocity: Vec2) {
        if self.velocity.x < velocity.x || self.velocity.x < -velocity.x {
            self.velocity.x += velocity.x * 0.00001
        }
        if self.velocity.y < velocity.y || self.velocity.y < -velocity.y {
            self.velocity.y += velocity.y * 0.00001
        }
    }
}