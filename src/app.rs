use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};
use render_backend::state::State;
use crate::render_backend;

pub struct App {
    state: Option<State>,
    last_time: instant::Instant,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: None,
            last_time: instant::Instant::now(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut window_attributes = Window::default_attributes();
        window_attributes.inner_size = Some(PhysicalSize {
            width: 1200,
            height: 700,
        }.into());
        window_attributes.title = "Pong - Rust/WGPU".to_string();

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.state = Some(pollster::block_on(State::new(window)).unwrap());
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(s) => s,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::Resized(size) => state.resize(size.width, size.height),

            WindowEvent::RedrawRequested => {
                let dt = self.last_time.elapsed();
                self.last_time = instant::Instant::now();

                state.update(dt);

                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = state.window.inner_size();
                        state.resize(size.width, size.height);
                    }
                    Err(e) => log::error!("Render error: {}", e),
                }
            }

            // ✅ CONTRÔLES CLAVIER
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state: key_state,
                    ..
                },
                ..
            } => {
                let speed = 0.03;

                if key_state == ElementState::Pressed {
                    match code {
                        KeyCode::Escape => event_loop.exit(),

                        // Raquette gauch
                        KeyCode::KeyW => {
                            state.engine.physics.scene.player1.position.y += speed;
                        }
                        KeyCode::KeyS => {
                            state.engine.physics.scene.player1.position.y -= speed;
                        }

                        // Raquette droite
                        KeyCode::ArrowUp => {
                            state.engine.physics.scene.player2.position.y += speed;
                        }
                        KeyCode::ArrowDown => {
                            state.engine.physics.scene.player2.position.y -= speed;
                        }

                        // Reset balle
                        KeyCode::Space => {
                            use glam::vec2;
                            state.engine.physics.scene.ball.position = vec2(0.0, 0.0);
                            state.engine.physics.scene.ball.velocity = vec2(0.02, 0.015);
                        }

                        _ => {}
                    }
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = &self.state {
            Window::request_redraw(&state.window);
        }
    }
}