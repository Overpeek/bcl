use std::ops::{Deref, DerefMut};

//

#[derive(Debug, Clone)]
pub struct Stack(Vec<u8>);

//

impl Stack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self(vec![0; cap])
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::with_capacity(u16::MAX as usize)
    }
}

impl Deref for Stack {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0[..]
    }
}

impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0[..]
    }
}
