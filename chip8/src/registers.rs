use std::ops::{Index, IndexMut, Range, RangeFrom, RangeTo};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reg(pub u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RegIndex(pub u8);

pub struct Regs(pub [Reg; 0x10]);
impl Default for Regs {
    fn default() -> Self {
        Regs([Reg(0); 0x10])
    }
}

impl Index<RegIndex> for Regs {
    type Output = Reg;

    fn index(&self, index: RegIndex) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

impl IndexMut<RegIndex> for Regs {
    fn index_mut(&mut self, index: RegIndex) -> &mut Self::Output {
        &mut self.0[index.0 as usize]
    }
}

impl Index<u8> for Regs {
    type Output = Reg;

    fn index(&self, index: u8) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u8> for Regs {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl Index<RangeFrom<RegIndex>> for Regs {
    type Output = [Reg];

    fn index(&self, index: RangeFrom<RegIndex>) -> &Self::Output {
        &self.0[index.start.0 as usize..]
    }
}

impl Index<RangeTo<RegIndex>> for Regs {
    type Output = [Reg];

    fn index(&self, index: RangeTo<RegIndex>) -> &Self::Output {
        &self.0[..index.end.0 as usize]
    }
}

impl Index<Range<RegIndex>> for Regs {
    type Output = [Reg];

    fn index(&self, index: Range<RegIndex>) -> &Self::Output {
        &self.0[index.start.0 as usize..index.end.0 as usize]
    }
}

impl Index<RangeTo<u8>> for Regs {
    type Output = [Reg];

    fn index(&self, index: RangeTo<u8>) -> &Self::Output {
        &self.0[..index.end as usize]
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
