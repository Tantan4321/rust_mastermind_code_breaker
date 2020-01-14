use std::io;
use rand::Rng;
use std::process::exit;
use std::borrow::BorrowMut;
use crate::{parse_response, vectorize_number};
use std::ops::Index;

static mut combo_gen: Vec<Vec<usize>> = Vec::new(); //TODO: make this local to Codebreaker

pub struct CodeBreaker {
    pub secret_length: usize,
    pub guessed: Vec<String>,
    pub combinations: Vec<Vec<usize>>,
}

impl CodeBreaker {
    fn create_combos(length: usize) -> Vec<Vec<usize>> {
        let mut current: Vec<usize> = vec![0; length];
        let digits: Vec<usize> = (0..10).collect();
        combo_recur(length, 0, current.as_mut(), &digits);
        return unsafe { combo_gen.clone() };
    }

    pub fn constructor(length: usize) -> CodeBreaker {
        return CodeBreaker {
            secret_length: length,
            guessed: vec![],
            combinations: CodeBreaker::create_combos(length),
        };
    }

    pub fn play(self: &mut Self) -> String {
        if self.guessed.is_empty() {
            let guess = Self::init_guess(self.secret_length);
            self.guessed.push(guess.to_string());
            return guess.to_string();
        } else {
            let guess = self.next_guess();
            return "0".to_string();  //TODO: implement
        }
    }


    pub fn next_guess(&self) -> String{
        let sorted = |set: Vec<usize>| {
            let mut temp = set;
            temp.sort();
            temp
        };

        let next_guesses = sorted(self.minimax());
    }

    fn minimax(self: &Self) -> Vec<usize> {
        return vec![]
    }


    pub fn prune(self: &mut Self, last_guess: String, response: Vec<usize>) {
        self.guessed.push(last_guess);
        let vec_guess = vectorize_number(&last_guess);
        self.guessed.retain(&|element: &Vec<usize>| score_calc(&vec_guess, element) == response)
    }

    pub fn remove_guess(self: &mut Self, last_guess: Vec<usize>){
        let index = self.combinations.iter().position(|x| *x == some_x).unwrap();
        self.combinations.remove(index);
        unsafe {
            combo_gen.remove(index);
        }
    }

    pub fn get_combos(self) -> Vec<Vec<usize>> {
        self.combinations
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
}

fn combo_recur(combinationLength: usize, element: usize, current: &Vec<usize>, digits: &Vec<usize>) {
    let mut current_copy = current.to_vec();
    if element >= combinationLength {
        unsafe {
            combo_gen.push(current_copy);
        }
        return;
    }
    for i in 0..digits.len() {
        current_copy[element] = digits[i];
        combo_recur(combinationLength, element + 1, &mut current_copy, digits);
    }
    return;
}

fn score_calc(guess: &Vec<usize>, chosen: &Vec<usize>) -> Vec<usize> { //TODO: check behavior
    let mut c: usize = 0;
    let mut w: usize = 0;
    for i in 0..guess.len() {
        if guess[i] == chosen[i] {
            c += 1;
        } else if chosen.contains(&guess[i]) {
            w += 1;
        }
    }

    let mut ret: Vec<usize> = Vec::new();
    ret.push(c);
    ret.push(w);
    return ret;
}