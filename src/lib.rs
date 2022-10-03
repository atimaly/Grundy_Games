pub mod game {
    ///This is for games which have grundy numbers    
    pub trait Grundy {
        ///The way you describe the state of the game in the current moment
        type State;

        fn grundy_number(&self, state: &Self::State) -> usize;
      /* 
        ///The move that is needed to move from a higher grundy state (state_start) to a lower
        ///state_end
        fn moving_from_state<T>(&self, state_start: &Self::State, state_end: &Self::State) -> Result<T, String>;*/

    	///Computes two games nim sum given the states for the games
        fn add<T: Grundy + Grundy<State = U>, U>(&self, state_self: &Self::State, game_2: &T, state_2: &U) -> usize {
            self.grundy_number(state_self) ^ game_2.grundy_number(state_2)
        }
    }

}

pub mod example_games;

///Minimum excluded number
///# Examples
/// ```
/// let mut v = vec![2,4,0,1];
/// assert_eq!(mex(&mut v), 3);
/// ```
pub fn mex(numb: &mut Vec<usize>) -> usize{
    let mut smallest: usize = 0;
    numb.sort();
    for v in numb.iter() {
        if smallest == *v {
            smallest += 1;
        }
        if smallest < *v {
            return smallest;
        }
    }
    return smallest;
}

///The bitwise XOR on the given values
pub fn nim_sum(numb: &Vec<usize>) -> usize {
    numb.iter().fold(0, |mut acc, x| {acc = acc^x; acc})
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mex() {
        let mut v = vec![2,4,0,1];
        assert_eq!(mex(&mut v), 3);

        let mut v = vec![0,1,0,2];
        assert_eq!(mex(&mut v), 3);
    }
}
