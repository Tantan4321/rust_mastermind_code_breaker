use std::io;
use rand::Rng;
use std::process::exit;
use rand::seq::SliceRandom;
use itertools::Itertools;
use crate::codebreaker::CodeBreaker;

mod codebreaker;

fn main() {
    println!("Let's play mastermind! My goal is to guess your number.");

    let stdin = io::stdin();
    let mut input = String::new();
    let mut rng = rand::thread_rng();

    println!("How many digits is your secret number?");
    stdin.read_line(&mut input).expect("Input failed");
    let input = input.trim();

    //check if input is actually a number, then set to size
    let _size: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("That's not a number!");
            exit(0);
        }
    };

    /**
        Combination generation
    */
    let mut breaker = CodeBreaker::constructor(_size);

    /*
    // ###############################
    // #   Printout Combination set  #
    // ###############################
    for val in breaker.get_combos().iter() {
        println!("{:?}", val);
    }*/

    /**
        UI init
    */

    let mut won = false;
    let mut this_guess = init_guess(_size);
    let mut score: Vec<usize> = [0,0].to_vec();

    while !won {
        print!("My guess is: {}", this_guess);

        println!(". Answer? (in format: '1,2')");

        //read and parse user response
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Input failed");
        let input = input.trim();
        //parse raw input into vectorized response
        let score = parse_response(input).to_vec();

        /*check win condition */
        if score[0] == _size {
            println!("######################\nCodeBreaker wins!!\n######################");
            won = true;
            break;
        }

        let mut this_guess = breaker.play(&this_guess, score).to_string();

    }
}

fn init_guess(length: usize) -> String {
    let format: Vec<usize> =
        [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9].to_vec();

    let mut ret: String = String::new();
    for x in 0..length {
        ret.push_str(&(format[x % 20]).to_string());
    }
    return ret;
}

fn vectorize_number(num: &str) -> Vec<usize> {
    num.chars()
        .map(|s| s.to_string().parse().unwrap())
        .collect()
}

fn parse_response(score: &str) -> Vec<usize> {
    let score_list: Vec<usize> = score.trim().split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    score_list
}