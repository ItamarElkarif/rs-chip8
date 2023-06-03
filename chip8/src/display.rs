use std::ops::Deref;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const fn screen_size() -> usize {
    SCREEN_HEIGHT * SCREEN_WIDTH
}
pub type DisplayData = [bool; screen_size()];

pub struct Display {
    data: DisplayData,
    should_redraw: bool,
}

impl Display {
    pub fn data(&self) -> [bool; screen_size()] {
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
            data: [false; screen_size()],
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
