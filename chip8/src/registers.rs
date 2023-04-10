// TODO: Gather with instuction/stack for memory stuff?
use std::ops::{Index, IndexMut, Range, RangeFrom, RangeTo};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RegIndex(pub u8);

pub struct Regs(pub [u8; 0x10]);
impl Default for Regs {
    fn default() -> Self {
        Regs([0u8; 0x10])
    }
}

impl Index<RegIndex> for Regs {
    type Output = u8;

    fn index(&self, index: RegIndex) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

impl IndexMut<RegIndex> for Regs {
    fn index_mut(&mut self, index: RegIndex) -> &mut Self::Output {
        &mut self.0[index.0 as usize]
    }
}

impl Index<RangeFrom<RegIndex>> for Regs {
    type Output = [u8];

    fn index(&self, index: RangeFrom<RegIndex>) -> &Self::Output {
        &self.0[index.start.0 as usize..]
    }
}

impl Index<RangeTo<RegIndex>> for Regs {
    type Output = [u8];

    fn index(&self, index: RangeTo<RegIndex>) -> &Self::Output {
        &self.0[..index.end.0 as usize]
    }
}

impl Index<Range<RegIndex>> for Regs {
    type Output = [u8];

    fn index(&self, index: Range<RegIndex>) -> &Self::Output {
        &self.0[index.start.0 as usize..index.end.0 as usize]
    }
}

impl IndexMut<RangeTo<RegIndex>> for Regs {
    fn index_mut(&mut self, index: RangeTo<RegIndex>) -> &mut Self::Output {
        &mut self.0[..index.end.0 as usize]
    }
}

impl IndexMut<RangeFrom<RegIndex>> for Regs {
    fn index_mut(&mut self, index: RangeFrom<RegIndex>) -> &mut Self::Output {
        &mut self.0[index.start.0 as usize..]
    }
}

impl IndexMut<Range<RegIndex>> for Regs {
    fn index_mut(&mut self, index: Range<RegIndex>) -> &mut Self::Output {
        &mut self.0[index.start.0 as usize..index.end.0 as usize]
    }
}
