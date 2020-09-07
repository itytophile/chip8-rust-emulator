mod chip8;

use chip8::Chip8;

fn main() {
    let mut chip = Chip8::new();
    chip.read(std::path::Path::new(
        "/home/ityt/Téléchargements/Maze [David Winter, 199x].ch8",
    ));

    chip.run();
}
