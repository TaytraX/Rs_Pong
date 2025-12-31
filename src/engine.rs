use glam::{vec2};
use crate::rigid_body::{Component, RigidBody};
use crate::physics::Physics;

pub struct Scene {
    pub size: (u32, u32),
    pub ball: Component,
    pub player1: Component,
    pub player2: Component,
    pub ball_body: RigidBody,
}

pub struct Engine {
    pub physics: Physics,
    dt: f32,
}

impl Engine {
    pub fn new() -> Self {
        let scene = Scene {
            size: (800, 600),

            ball: Component::new(
                vec2(0.0, 0.0),
                vec2(0.02, 0.02),
                [1.0, 0.0, 0.0]
            ),

            player1: Component::new(
                vec2(-0.85, 0.0),
                vec2(0.03, 0.2),
                [1.0, 1.0, 1.0]
            ),

            player2: Component::new(
                vec2(0.85, 0.0),
                vec2(0.03, 0.2),
                [1.0, 1.0, 1.0]
            ),

            ball_body: RigidBody {
                mass: 1.0,
                restitution: 1.0,
            },
        };

        Self {
            dt: 0.016,
            physics: Physics {
                gravity: 0.0,
                scene,
            },
        }
    }

    pub fn update(&mut self) {
        self.physics.update(self.dt);
    }
}