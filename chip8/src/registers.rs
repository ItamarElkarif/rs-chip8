use std::ops::{Index, IndexMut, Range, RangeFrom, RangeTo};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reg(pub u8);

pub struct Regs(pub [Reg; 0x10]);
impl Default for Regs {
    fn default() -> Self {
        Regs([Reg(0); 0x10])
    }
}

impl Index<Reg> for Regs {
    type Output = Reg;

    fn index(&self, index: Reg) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

impl IndexMut<Reg> for Regs {
    fn index_mut(&mut self, index: Reg) -> &mut Self::Output {
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

impl Index<RangeFrom<Reg>> for Regs {
    type Output = [Reg];

    fn index(&self, index: RangeFrom<Reg>) -> &Self::Output {
        &self.0[index.start.0 as usize..]
    }
}

impl Index<RangeTo<Reg>> for Regs {
    type Output = [Reg];

    fn index(&self, index: RangeTo<Reg>) -> &Self::Output {
        &self.0[..index.end.0 as usize]
    }
}

impl Index<Range<Reg>> for Regs {
    type Output = [Reg];

    fn index(&self, index: Range<Reg>) -> &Self::Output {
        &self.0[index.start.0 as usize..index.end.0 as usize]
    }
}

impl Index<RangeTo<u8>> for Regs {
    type Output = [Reg];

    fn index(&self, index: RangeTo<u8>) -> &Self::Output {
        &self.0[..index.end as usize]
    }
}

impl IndexMut<RangeTo<Reg>> for Regs {
    fn index_mut(&mut self, index: RangeTo<Reg>) -> &mut Self::Output {
        &mut self.0[..index.end.0 as usize]
    }
}

impl IndexMut<RangeFrom<Reg>> for Regs {
    fn index_mut(&mut self, index: RangeFrom<Reg>) -> &mut Self::Output {
        &mut self.0[index.start.0 as usize..]
    }
}

impl IndexMut<Range<Reg>> for Regs {
    fn index_mut(&mut self, index: Range<Reg>) -> &mut Self::Output {
        &mut self.0[index.start.0 as usize..index.end.0 as usize]
    }
}
