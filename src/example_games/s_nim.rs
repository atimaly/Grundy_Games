use crate::game::*;
use crate::mex;


///Example problem: https://timus.online/problem.aspx?space=1&num=1180
pub struct SNim{
    s_ : Vec<usize>, //The amount you can take from the only pile
    states_: Vec<usize>, // grundy values
    pub number_states_: usize, //How many states one should calculate up to and the state you are interested in

}

impl SNim {
    ///Given the max number of stones, which you still want to check
    ///and the possible number of stones you can take away from the pile
    ///it generates an instance.
    pub fn new(n: usize, s: Vec<usize>) -> SNim {
        SNim{
            s_: s,
            states_: vec![0; n+1],
            number_states_: n,
        }
    }

    pub fn print_states(&self) -> (){
        for (pos, v) in self.states_.iter().enumerate() {
            if *v == 0 {
                println!["{}: P, grundy value: {}", pos, self.states_[pos]];
            }
            else {
                println!["{}: N, grundy value: {}", pos, self.states_[pos]];
            }
        }
    }

    pub fn calculate_states(&mut self) {
        for pos in 0..self.states_.len() {
            let mut values = Vec::new(); //possible positions grundy values
            for m in self.s_.iter() {
                if (pos as isize)-(*m as isize) >= 0 {
                    values.push(self.states_[pos-*m]);
                }
            }
            self.states_[pos] = mex(&mut values);
        }
    }
}


impl Grundy for SNim {
   
    type State = usize;

    ///Need to call self.calculate_states() before
    fn grundy_number(&self, state: &usize) -> usize {
        //Before using the function need to call self.calculate_states();
        //self.states_[self.number_states_]
        self.states_[*state]
    }
    /*
    fn moving_from_state(&self, state_start: &usize, state_end: &usize) -> Result<usize, String> {
        if self.grundy_number(state_start) <= self.grundy_number(&state_end) {
            return Err("The start has lower grundy number")
        }
        


    }*/
}
