use std::io;

use grundy_games::example_games::*; //pub mod nim;//use crate::nim::*;
use grundy_games::game::*;

///A simple line by line input reader for code shortening
fn read_number() -> usize {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line)
      .expect("Failed to read line");
    input_line.trim().parse()
      .expect("Type in a number or a valid number")
}

fn example_nim() {
    println!("Nim example: START");

    println!("Give me the number of piles!");
    let n = read_number();

    let mut piles: Vec<nim::Nim> = Vec::new();
    println!("Give me line by line the stones in the piles!");
    for _i in 0..n {
        piles.push(nim::Nim::new(read_number()));
    }

    let nim_sum = piles.iter().fold(0, |mut acc, x| {acc = acc^x.grundy_number(&x.state_); acc});

    println!("Nim sum of the current game: {}", nim_sum);

    println!("Nim example: END");
}

fn example_s_nim() {
    println!("S-Nim example: START");

    println!("Give me the number of stones in the pile!");
    let n = read_number();
    
    println!("Enter the size of S:");
    let s_size = read_number();

    let mut s = Vec::new();
    println!("Enter the elements of S line by line:");
    for _i in 0..s_size {
        s.push(read_number());
    }
    
    let mut snim = s_nim::SNim::new(n, s);
    snim.calculate_states();

    println!("Grundy value of the state: {}", snim.grundy_number(&snim.number_states_));
    println!("S-Nim states:");
    snim.print_states();
    println!("S-Nim example: END");
}

fn main() {
    example_nim();

    example_s_nim();
}

