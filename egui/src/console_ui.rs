use chip8::{Display, FRAME_DURATION};

pub struct ConsoleUI();
impl ConsoleUI {
    fn update(display: &Display) {
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
            let (display, _) = chip.run_frame().unwrap();
            ConsoleUI::update(display);

            std::thread::sleep(FRAME_DURATION);
        }
    }
}
