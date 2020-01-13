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

    /**
        Permutation generation
    */
    let gen = (0..10).permutations(_size);
    let mut choices = Vec::new();

    //convert Permutations into vectorized list
    for i in gen{
        choices.push(i);
    }
    //shuffle the list of possibilities
    choices.shuffle(&mut rng);

    /* debug
    for i in choices{
        println!("{:?}", i);
    }
    */
    /**
        UI init
    */
    let mut answers: Vec<Vec<usize>> = Vec::new();
    let mut scores: Vec<Vec<usize>> = Vec::new();

    loop {
        let mut check = choices[0].to_vec();

        print!("{}   ", choices.len());
        print!("My guess is: ");
        for num in &check{
            print!("{}", num);
        }
        println!(". Answer? (in format: '1,2')");

        //read and parse user response
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Input failed");
        let input = input.trim();
        let mut score = parse_score(input).to_vec();

        //check win condition
        if score[0] == 6 {
            println!("LET'S GOOOOOOO!!");
            exit(0);
        }

        //clone choices and reset the choices array
        let temp_choices = choices.clone();
        choices.clear();

        for choice in temp_choices{
            let calc = score_calc(&choice, &check);
            if score == calc{
                choices.push(choice);
            }
        }

        //push to answers and scores log
        scores.push(score);
        answers.push(check);
    }

}

fn parse_score(score: &str) -> Vec<usize> {
    let score_list: Vec<usize> = score.trim().split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    score_list
}

fn score_calc(guess: &Vec<usize>, chosen: &Vec<usize>) -> Vec<usize> {
    let mut c: usize = 0;
    let mut w: usize = 0;

    for i in 0..guess.len(){
        if guess[i] == chosen[i]{
            c += 1;
        }else if chosen.contains(&guess[i]) {
            w += 1;
        }
    }

    let mut ret: Vec<usize> = Vec::new();
    ret.push(c);
    ret.push(w);
    return ret;
}