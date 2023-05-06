use std::ops::Deref;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub type DisplayData = [bool; SCREEN_WIDTH * SCREEN_HEIGHT];

pub struct Display {
    pub data: DisplayData,
    pub should_redrew: bool,
}

impl Display {
    pub(crate) fn mut_data_to_update(&mut self) -> &mut DisplayData {
        self.should_redrew = true;
        &mut self.data
    }
}

impl Default for Display {
    fn default() -> Display {
        Display {
            data: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            should_redrew: false,
        }
    }
}

impl Deref for Display {
    type Target = DisplayData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
