mod chip8;

use chip8::Chip8;

fn main() {
    let mut chip = Chip8::new();
    chip.read(std::path::Path::new(
        "/home/ityt/Téléchargements/Animal Race [Brian Astle].ch8",
    ));

    chip.run();
}
