pub(crate) const STACK_SIZE: usize = 0x10;

#[derive(Default)]
pub struct Stack {
    storage: [u16; STACK_SIZE],
    sp: usize,
}

impl Stack {
    pub fn push(&mut self, var: u16) -> Result<(), &'static str> {
        if self.sp == self.storage.len() {
            return Err("The stack is full");
        }

        self.storage[self.sp] = var;
        self.sp += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<u16> {
        match self.sp {
            0 => None,
            _ => {
                self.sp -= 1;
                Some(self.storage[self.sp])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    // The `test` attribute macro identifies functions that will run as tests
    #[test]
    fn test_fifo() {
        let mut s = Stack::default();
        assert_eq!(Ok(()), s.push(1));
        assert_eq!(Ok(()), s.push(2));
        assert_eq!(Some(2), s.pop());
        assert_eq!(Some(1), s.pop());
    }

    #[test]
    fn empty_pop() {
        let mut s = Stack::default();
        s.push(1);
        assert_eq!(Some(1), s.pop());
        assert_eq!(None, s.pop());
        assert_eq!(None, s.pop());
    }

    #[test]
    // #[should_panic]
    fn full_stack_err() {
        let mut s = Stack::default();
        for i in 0..s.storage.len() as u16 {
            assert_eq!(Ok(()), s.push(i));
        }
    }
}
