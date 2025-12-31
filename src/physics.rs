use crate::engine::Scene;

pub struct Physics {
    pub gravity: f32,
    pub collide: bool,
    pub scene: Scene,
}

impl Physics {
    fn apply_gravity(&mut self) {
        let current = self.scene.ball.velocity;
        self.scene.ball.set_velocity(glam::vec2(current.x, current.y - self.gravity));
    }

    fn collider_result(&mut self) {
        if self.collide || self.scene.ball.collider.collide(&self.scene.player1.collider) || self.scene.ball.collider.collide(&self.scene.player2.collider) {
            self.scene.ball.velocity = -self.scene.ball.velocity * self.scene.ball_body.restitution;
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.collider_result();
        self.scene.ball.position += self.scene.ball.velocity * self.scene.ball_body.mass * dt;

        self.collide = if self.scene.ball.position.y <= -((self.scene.size.1 / 2) as f32) || self.scene.ball.position.y >= (self.scene.size.1 / 2) as f32 {
            true
        } else { false };

        self.apply_gravity();
    }
}