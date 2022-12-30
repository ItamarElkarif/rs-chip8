mod resources;
use chip8::Chip8;
use std::path::Path;

struct SDL;
impl chip8::UI for SDL {
    fn update(&mut self, display: &chip8::DisplayData) {
        print!("\x1B[2J\x1B[1;1H");
        for row in display {
            println!("{}", row.map(|p| if p { "X" } else { " " }).join(""))
        }
    }
}
fn main() {
    let mut sdl = SDL;
    let mut chip = Chip8::new(&mut sdl);
    chip8::run_file(&mut chip, Path::new("../../IBM Logo.ch8")).unwrap();
}
