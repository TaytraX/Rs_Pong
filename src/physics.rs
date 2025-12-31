use crate::engine::Scene;
use glam::vec2;

pub struct Physics {
    pub gravity: f32,
    pub scene: Scene,
}

impl Physics {
    pub fn update(&mut self, dt: f32) {
        self.scene.ball.position += self.scene.ball.velocity * dt * 30.0;

        let screen_top = 1.0;
        let screen_bottom = -1.0;

        if self.scene.ball.position.y + self.scene.ball.collider.half_size.y >= screen_top {
            self.scene.ball.position.y = screen_top - self.scene.ball.collider.half_size.y;
            self.scene.ball.velocity.y = -self.scene.ball.velocity.y;
        }

        if self.scene.ball.position.y - self.scene.ball.collider.half_size.y <= screen_bottom {
            self.scene.ball.position.y = screen_bottom + self.scene.ball.collider.half_size.y;
            self.scene.ball.velocity.y = -self.scene.ball.velocity.y;
        }

        if self.scene.ball.collider.collides_with(
            self.scene.ball.position,
            &self.scene.player1.collider,
            self.scene.player1.position,
        ) {
            self.scene.ball.velocity.x = self.scene.ball.velocity.x.abs();
            self.scene.ball.position.x = self.scene.player1.position.x
                + self.scene.player1.collider.half_size.x
                + self.scene.ball.collider.half_size.x;
        }

        if self.scene.ball.collider.collides_with(
            self.scene.ball.position,
            &self.scene.player2.collider,
            self.scene.player2.position,
        ) {
            self.scene.ball.velocity.x = -self.scene.ball.velocity.x.abs();
            self.scene.ball.position.x = self.scene.player2.position.x
                - self.scene.player2.collider.half_size.x
                - self.scene.ball.collider.half_size.x;
        }

        if self.scene.ball.position.x < -1.2 || self.scene.ball.position.x > 1.2 {
            self.scene.ball.position = vec2(0.0, 0.0);
            self.scene.ball.velocity = vec2(0.02, 0.015);
        }

        self.scene.ball.velocity.y -= self.gravity * dt;
    }
}