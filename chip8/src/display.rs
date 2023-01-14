pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub type DisplayData = [bool; SCREEN_WIDTH * SCREEN_HEIGHT];
pub trait UI {
    fn update(&mut self, display: &DisplayData);
    fn beep(&self);
}

pub struct Display {
    data: DisplayData,
    updated: bool,
}

impl Display {
    pub fn mut_data_to_update(&mut self) -> &mut [bool; SCREEN_WIDTH * SCREEN_HEIGHT] {
        self.updated = true;
        &mut self.data
    }

    pub fn data(&self) -> &[bool; SCREEN_WIDTH * SCREEN_HEIGHT] {
        &self.data
    }

    pub fn updated(&self) -> bool {
        self.updated
    }

    pub fn reset_updated(&mut self) {
        self.updated = false;
    }
}

impl Default for Display {
    fn default() -> Display {
        Display {
            data: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            updated: false,
        }
    }
}
