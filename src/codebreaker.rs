use std::io;
use rand::Rng;
use std::process::exit;
use std::borrow::BorrowMut;

static mut combo_gen: Vec<Vec<usize>> = Vec::new(); //TODO: make this local to Codebreaker

pub struct CodeBreaker {
    pub guessed: Vec<usize>,
    pub combinations: Vec<Vec<usize>>,
}

impl CodeBreaker {
    fn create_combos() -> Vec<Vec<usize>>{
        let mut current: Vec<usize> = vec![0; 6 as usize];
        let digits: Vec<usize> = (0..10).collect();
        combo_recur(6, 0, current.as_mut(), &digits);
        return unsafe{combo_gen.clone()};
    }

    pub fn constructor() -> CodeBreaker {
        return CodeBreaker {
            guessed: vec![],
            combinations: CodeBreaker::create_combos(),
        }

    pub fn play(&mut self) -> usize {
        if self.guessed.is_empty() {
            let guess = Self::init_guess();
            self.guessed.push(guess);
            return guess
        }else {
            return 0;  //TODO: implement
        }
    }

    pub fn get_combos(self) -> Vec<Vec<usize>>{
        self.combinations
    }

    fn init_guess() -> usize {
        return 001122;
    }
}

fn combo_recur(combinationLength: usize, element: usize, current: &Vec<usize>, digits: &Vec<usize>){
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

fn score_calc(guess: &Vec<usize>, chosen: &Vec<usize>) -> Vec<usize> {
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