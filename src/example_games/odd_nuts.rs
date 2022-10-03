use crate::game::*;
use crate::{mex, nim_sum};

///The problem https://timus.online/problem.aspx?space=1&num=2068
pub struct OddNuts {
    max_odd_size_: usize, //This is the state you are interested
    states_: Vec<usize>, //grundy values of the positions we will only care about odd sizes
}

impl OddNuts {
    pub fn new(maxi: usize) -> OddNuts {
        OddNuts{
            max_odd_size_: maxi,
            states_: vec![0; maxi+1],
        }
    }
    

    ///We determine the grundy numbers for the states
    pub fn calculate_states() {
        for 0.. 
    }
}

impl Grundy for OddNuts {
    
    type State = usize;

    fn grundy_number(&self, state: usize) -> usize {
        if(state > max_odd_size) eprintln!("The requested state is too big!");
        self.states_[state]
    }
}
