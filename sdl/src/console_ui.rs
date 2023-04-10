pub struct ConsoleUI();
impl ConsoleUI {
    fn update(display: &chip8::DisplayData) {
        print!("{esc}c", esc = 27 as char);
        for row in display.chunks(chip8::SCREEN_WIDTH) {
            println!(
                "{}",
                row.iter()
                    .map(|&p| if p { "█" } else { " " })
                    .collect::<Vec<&str>>()
                    .join("")
            )
        }
    }

    fn beep(&self) {
        println!("BEEPED");
    }
}

impl crate::Ui for ConsoleUI {
    fn run(mut chip: chip8::Chip8) {
        loop {
            chip.run_frame().unwrap();
            if chip.updated_display() {
                chip.reset_updated();
                ConsoleUI::update(chip.display());
            }
            // std::thread::sleep(Duration::from_millis(17));
        }
    }
}
