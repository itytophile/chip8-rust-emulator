use std::{thread, rc::Rc, sync::Mutex, sync::Arc};

use glutin_window::GlutinWindow;
use graphics::{clear, math::Matrix2d, rectangle};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use super::graphic_engine::GraphicEngine;

pub struct PistonInterface {
    is_running: Arc<Mutex<bool>>,
    is_draw_open: bool,
}

const SCALE: u32 = 4;

impl PistonInterface {
    pub fn new() -> PistonInterface {
        PistonInterface {
            is_running: Arc::new(Mutex::new(true)),
            is_draw_open: false,
        }
    }
}

const BLACK: [f32; 4] = [0., 0., 0., 1.];
const WHITE: [f32; 4] = [1., 1., 1., 1.];

impl GraphicEngine for PistonInterface {
    fn clear_screen(&mut self) { /*
                                 if let Some(e) = self.events.next(&mut self.window) {
                                     if let Some(args) = e.render_args() {
                                         self.gl.draw(args.viewport(), |_, gl| {
                                             clear(BLACK, gl);
                                         });
                                     }
                                 }*/
    }

    fn draw_sprite(&mut self, x: u8, y: u8, sprite_bytes: &[u8]) -> bool {
        /*
                println!("({} ; {}) {:?}", x, y, sprite_bytes);

                // self.safe_open_draw();

                if let Some(e) = self.events.next(&mut self.window) {
                    println!("coucou");
                    if let Some(args) = e.render_args() {
                        self.gl.draw(args.viewport(), |c, gl| {
                            println!("je dessine");
                            for height in 0..sprite_bytes.len() - 1 {
                                let bit = sprite_bytes[height];
                                for index_bit in 0..8 {
                                    if bit & (1 << index_bit) > 0 {
                                        rectangle(
                                            WHITE,
                                            rectangle::square(
                                                ((x + 7 - index_bit) as u32 * SCALE) as f64,
                                                ((y as u32 + height as u32) * SCALE) as f64,
                                                SCALE as f64,
                                            ),
                                            c.transform,
                                            gl,
                                        );
                                    }
                                }
                            }
                        });
                    }
                }
        */
        false
    }

    fn flush(&mut self) {
        // self.safe_close_draw();
    }

    fn is_running(&self) -> bool {
        *self.is_running.lock().unwrap()
    }

    fn init_draw(&mut self) {
        let is_running = Arc::clone(&self.is_running);
        thread::spawn(move || {
            let opengl = OpenGL::V3_2;

            let mut window: GlutinWindow = WindowSettings::new(
                "Piston Chip8",
                [super::SCREEN_WIDTH * SCALE, super::SCREEN_HEIGHT * SCALE],
            )
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

            let mut gl = GlGraphics::new(opengl);
            let mut events = Events::new(EventSettings::new());

            while let Some(e) = events.next(&mut window) {
                if let Some(args) = e.render_args() {
                    gl.draw(args.viewport(), |_, gl| {
                        clear(BLACK, gl);
                    })
                }
            }

            *is_running.lock().unwrap() = false;
        });
    }
}
