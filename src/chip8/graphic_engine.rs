pub trait GraphicEngine {
    fn clear_screen(&mut self);
    /// Draw a sprite from bytes.
    /// One byte is a line of 8 pixels, one pixel for each bit.
    /// If the bit equals one the pixel is on, off otherwise.
    ///
    fn draw_sprite(&mut self, x: u8, y: u8, sprite_bytes: &[u8]) -> bool;
    fn flush(&mut self);
    fn is_running(&self) -> bool;
    fn init(&mut self);
}
