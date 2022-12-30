mod resources;
use chip8::Chip8;
use std::path::Path;

struct SDL;
impl chip8::UI for SDL {
    fn update(&mut self, _display: &chip8::Display) {
        todo!()
    }
}
fn main() {
    let mut sdl = SDL;
    let mut chip = Chip8::new(&mut sdl);
    chip8::run_file(&mut chip, &Path::new("test")).unwrap();
}
