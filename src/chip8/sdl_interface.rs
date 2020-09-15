use super::graphic_engine::GraphicEngine;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
    EventPump,
};

pub struct SdlInterface {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    is_running: bool,
}

const SCALE: u32 = 4;

impl SdlInterface {
    pub fn new() -> SdlInterface {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(
                "chip8",
                super::SCREEN_WIDTH * SCALE,
                super::SCREEN_HEIGHT * SCALE,
            )
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        SdlInterface {
            canvas,
            event_pump,
            is_running: true,
        }
    }
}

impl GraphicEngine for SdlInterface {
    fn clear_screen(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    fn draw_sprite(&mut self, x: u8, y: u8, sprite_bytes: &[u8]) -> bool {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        println!("({} ; {}) {:?}", x, y, sprite_bytes);

        for height in 0..sprite_bytes.len() - 1 {
            let bit = sprite_bytes[height];
            for index_bit in 0..8 {
                if bit & (1 << index_bit) > 0 {
                    self.canvas
                        .fill_rect(Rect::new(
                            (x as i32 + 7 - index_bit) * SCALE as i32,
                            (y as i32 + height as i32) * SCALE as i32,
                            SCALE,
                            SCALE,
                        ))
                        .unwrap();
                }
            }
        }
        false
    }

    fn flush(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.is_running = false;
                }
                _ => {}
            }
        }

        self.canvas.present();
    }

    fn is_running(&self) -> bool {
        self.is_running
    }

    fn init(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }
}
