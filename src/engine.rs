use glam::{vec2};
use crate::box_collider::Collider;
use crate::physics::Physics;
use crate::rigid_body::{Componant, RigidBody};

pub struct Scene {
    pub size: (u32, u32),
    pub(crate) ball: Componant,
    pub(crate) player1: Componant,
    pub(crate) player2: Componant,
    pub ball_body: RigidBody
}

pub struct Engine {
    physics: Physics,
    dt: f32
}

impl Engine {
    pub fn new() -> Self {
        let player1 = Componant {
            position: vec2(-1.0, 0.0),
            velocity: vec2(0.0, 0.0),
            collider: Collider::new((0.2, 2.0))
        };

        let player2 = Componant {
            position: vec2(10.0, 0.0),
            velocity: vec2(0.0, 0.0),
            collider: Collider::new((0.2, 2.0))
        };

        let scene = Scene {
            size: (800, 600),
            ball: Componant {
                position: Default::default(),
                velocity: Default::default(),
                collider: Collider::new((0.1, 0.1))
            },
            player1,
            player2,
            ball_body: RigidBody { mass: 0.2, restitution: 1.0 },
        };

        Self {
            dt: 0.016,
            physics: Physics {
                gravity: 8.6,
                collide: false,
                scene
            },
        }
    }

    pub fn update(&mut self) {
        self.physics.scene.player1.collider.update(self.physics.scene.player1.position);
        self.physics.scene.player2.collider.update(self.physics.scene.player2.position);
        self.physics.scene.ball.collider.update(self.physics.scene.ball.position);
        self.physics.update(self.dt);
        println!("position: {:?},  velocite: {:?}", self.physics.scene.ball.position, self.physics.scene.ball.velocity);
    }
}