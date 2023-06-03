use std::ops::{Index, IndexMut, Range, RangeFrom, RangeTo};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RegI(pub u8);

pub struct Regs(pub [u8; 0x10]);
impl Default for Regs {
    fn default() -> Self {
        Regs([0u8; 0x10])
    }
}

impl Index<RegI> for Regs {
    type Output = u8;

    fn index(&self, index: RegI) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

impl IndexMut<RegI> for Regs {
    fn index_mut(&mut self, index: RegI) -> &mut Self::Output {
        &mut self.0[index.0 as usize]
    }
}

impl Index<RangeFrom<RegI>> for Regs {
    type Output = [u8];

    fn index(&self, index: RangeFrom<RegI>) -> &Self::Output {
        &self.0[index.start.0 as usize..]
    }
}

impl Index<RangeTo<RegI>> for Regs {
    type Output = [u8];

    fn index(&self, index: RangeTo<RegI>) -> &Self::Output {
        &self.0[..index.end.0 as usize]
    }
}

impl Index<Range<RegI>> for Regs {
    type Output = [u8];

    fn index(&self, index: Range<RegI>) -> &Self::Output {
        &self.0[index.start.0 as usize..index.end.0 as usize]
    }
}

impl IndexMut<RangeTo<RegI>> for Regs {
    fn index_mut(&mut self, index: RangeTo<RegI>) -> &mut Self::Output {
        &mut self.0[..index.end.0 as usize]
    }
}

impl IndexMut<RangeFrom<RegI>> for Regs {
    fn index_mut(&mut self, index: RangeFrom<RegI>) -> &mut Self::Output {
        &mut self.0[index.start.0 as usize..]
    }
}

impl IndexMut<Range<RegI>> for Regs {
    fn index_mut(&mut self, index: Range<RegI>) -> &mut Self::Output {
        &mut self.0[index.start.0 as usize..index.end.0 as usize]
    }
}
