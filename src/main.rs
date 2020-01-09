use std::io;
use rand::Rng;
use std::process::exit;
use rand::seq::SliceRandom;
use itertools::Itertools;

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

    //possible digits
    let digits = "1234567890";

    let gen = (0..10).permutations(_size);
    let mut choices = Vec::new();

    //Convert Permutations into vectorized list
    for i in gen{
        choices.push(i);
    }

    choices.shuffle(&mut rng);

    for i in choices{
        println!("{:?}", i);
    }
}


fn parse_score(score: &str) -> Vec<i32> {
    let score_list: Vec<i32> = score.trim().split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    score_list
}