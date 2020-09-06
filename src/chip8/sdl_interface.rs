use super::graphic_engine::GraphicEngine;

pub struct SdlInterface {}

impl SdlInterface {
    pub fn new() -> SdlInterface {
        SdlInterface {}
    }
}

impl GraphicEngine for SdlInterface {
    fn clear_screen(&mut self) {
        todo!()
    }

    fn draw_sprite(&mut self, x: u8, y: u8, sprite_bytes: &[u8]) -> bool {
        todo!()
    }
}