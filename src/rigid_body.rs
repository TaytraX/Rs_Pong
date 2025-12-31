use glam::{vec2, Vec2};
use crate::box_collider::Collider;

pub struct RigidBody {
    pub mass: f32,
    pub restitution: f32,
}

pub struct Component {
    pub position: Vec2,
    pub velocity: Vec2,
    pub collider: Collider,
}

impl Component {
    pub fn new(position: Vec2, half_size: Vec2, color: [f32; 3]) -> Self {
        Self {
            position,
            velocity: vec2(2.0, 0.3),
            collider: Collider::new(half_size, color),
        }
    }
}