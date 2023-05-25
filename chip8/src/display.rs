use std::ops::Deref;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub type DisplayData = [bool; SCREEN_WIDTH * SCREEN_HEIGHT];

pub struct Display {
    data: DisplayData,
    should_redraw: bool,
}

impl Display {
    pub fn data(&self) -> [bool; SCREEN_WIDTH * SCREEN_HEIGHT] {
        self.data
    }

    pub fn should_redraw(&self) -> bool {
        self.should_redraw
    }

    pub fn reset_redraw(&mut self) {
        self.should_redraw = false;
    }

    pub(crate) fn mut_data_to_update(&mut self) -> &mut DisplayData {
        self.should_redraw = true;
        &mut self.data
    }
}

impl Default for Display {
    fn default() -> Display {
        Display {
            data: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            should_redraw: false,
        }
    }
}

impl Deref for Display {
    type Target = DisplayData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
