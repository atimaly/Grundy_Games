use crate::game::*;

///The standard Nim game only one pile is given
pub struct Nim {
    pub state_: usize
}

impl Nim {

    pub fn new(size: usize) -> Nim {
        Nim{state_: size}
    }
}

impl Grundy for Nim {
    
    type State = usize;

    fn grundy_number(&self, state: &usize) -> usize {
        *state
    }
}

