use std::{collections::VecDeque, sync::Arc, sync::Mutex, thread};

use glutin_window::GlutinWindow;
use graphics::{clear, rectangle, types::Rectangle};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

use super::graphic_engine::GraphicEngine;

pub struct PistonInterface {
    is_running: Arc<Mutex<bool>>,
    task_data_queue: Arc<Mutex<VecDeque<TaskData>>>,
}

const SCALE: u32 = 4;

struct TaskData {
    task: fn(&Data, &mut Vec<Rectangle>),
    data: Option<Data>,
}

struct Data {
    x: u32,
    y: u32,
    sprite_bytes: Vec<u8>,
}

impl PistonInterface {
    pub fn new() -> PistonInterface {
        PistonInterface {
            is_running: Arc::new(Mutex::new(true)),
            task_data_queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
}

const BLACK: [f32; 4] = [0., 0., 0., 1.];
const WHITE: [f32; 4] = [1., 1., 1., 1.];

impl GraphicEngine for PistonInterface {
    fn clear_screen(&mut self) {
        self.task_data_queue.lock().unwrap().push_back(TaskData {
            task: |_, rects| {
                rects.clear();
            },
            data: None,
        });
    }

    fn draw_sprite(&mut self, x: u8, y: u8, sprite_bytes: &[u8]) -> bool {
        println!("({} ; {}) {:?}", x, y, sprite_bytes);

        let mut vec_byte: Vec<u8> = Vec::new();

        for byte in sprite_bytes {
            vec_byte.push(*byte);
        }

        self.task_data_queue.lock().unwrap().push_back(TaskData {
            task: |data, rects| {
                for height in 0..data.sprite_bytes.len() - 1 {
                    let bit = data.sprite_bytes[height];
                    for index_bit in 0..8 {
                        if bit & (1 << index_bit) > 0 {
                            rects.push(rectangle::square(
                                ((data.x + 7 - index_bit) * SCALE) as f64,
                                ((data.y + height as u32) * SCALE) as f64,
                                SCALE as f64,
                            ));
                        }
                    }
                }
            },
            data: Some(Data {
                x: x as u32,
                y: y as u32,
                sprite_bytes: vec_byte,
            }),
        });

        false
    }

    fn flush(&mut self) {}

    fn is_running(&self) -> bool {
        *self.is_running.lock().unwrap()
    }

    fn init(&mut self) {
        let is_running = Arc::clone(&self.is_running);
        let task_data_queue = Arc::clone(&self.task_data_queue);

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

            let mut rects: Vec<Rectangle> = Vec::new();

            while let Some(e) = events.next(&mut window) {
                if let Some(args) = e.render_args() {
                    gl.draw(args.viewport(), |c, gl| {
                        clear(BLACK, gl);

                        let mut queue = task_data_queue.lock().unwrap();
                        for task_data in queue.iter() {
                            if let Some(ref data) = task_data.data {
                                (task_data.task)(data, &mut rects);
                            }
                        }
                        queue.clear();

                        for rect in &rects {
                            rectangle(WHITE, *rect, c.transform, gl);
                        }
                    })
                }
            }

            *is_running.lock().unwrap() = false;
        });
    }
}
