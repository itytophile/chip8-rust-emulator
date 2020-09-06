mod chip8;

fn main() {
    let mut chip = chip8::Chip8::new();
    chip.read(std::path::Path::new(
        "/home/ityt/Téléchargements/Animal Race [Brian Astle].ch8",
    ));

    chip.run();
}
