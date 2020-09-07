use super::graphic_engine::GraphicEngine;
use sdl2::{render::Canvas, video::Window, EventPump, pixels::Color, event::Event, keyboard::Keycode};

pub struct SdlInterface {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    is_running: bool,
}

impl SdlInterface {
    pub fn new() -> SdlInterface {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("chip8", super::SCREEN_WIDTH*4, super::SCREEN_HEIGHT*4)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        SdlInterface { canvas, event_pump, is_running: true }
    }
}

impl GraphicEngine for SdlInterface {
    fn clear_screen(&mut self) {
        todo!()
    }

    fn draw_sprite(&mut self, x: u8, y: u8, sprite_bytes: &[u8]) -> bool {
        todo!()
    }

    fn draw(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    self.is_running = false;
                },
                _ => {},
            }
        }

        self.canvas.present();
    }

    fn is_running(&self) -> bool {
        self.is_running
    }
}
