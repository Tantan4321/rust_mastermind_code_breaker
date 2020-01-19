use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::collections::HashMap;
use std::borrow::Borrow;
use crate::vectorize_number;

pub struct CodeBreaker {
    pub guessed: Vec<String>,
    pub combinations: Vec<Vec<usize>>,
    pub original: Vec<Vec<usize>>,
}

impl CodeBreaker {
    pub fn create_combos(&mut self, length: usize) {
        let mut current: Vec<usize> = vec![0; length];
        let digits: Vec<usize> = (0..10).collect();
        self.combo_recur(length, 0, current.as_mut(), &digits);
    }

    fn combo_recur(&mut self, combination_length: usize, element: usize, current: &Vec<usize>, digits: &Vec<usize>) {
        let mut current_copy = current.to_vec();
        if element >= combination_length {
            self.original.push(current_copy.to_vec());
            self.combinations.push(current_copy.to_vec());
            return;
        }
        for i in 0..digits.len() {
            current_copy[element] = digits[i];
            self.combo_recur(combination_length, element + 1, &mut current_copy, digits);
        }
        return;
    }

    pub fn constructor() -> CodeBreaker {
        return CodeBreaker {
            guessed: vec![],
            combinations: vec![],
            original: vec![]
        };
    }

    pub fn play(self: &mut Self, last_guess: &String, last_score: Vec<usize>) -> String {
        //self.remove_guess(vectorize_number(&last_guess));
        println!("past remove");
        self.prune(&last_guess.to_string(), last_score);
        println!("past pruning. length of combinations: {}", self.combinations.len());

        let guess = self.next_guess().to_string();
        println!("second. Guessed: {:?}", self.guessed);
        return guess;
    }


    pub fn next_guess(&mut self) -> String {
        let next_guesses = self.minimax() ;

        let next_guess = next_guesses.first().unwrap();

        let result: String = next_guess.iter().fold(String::new(), |mut result, x| {
            result.push_str(&x.to_string());
            result
        });
        return result;
    }

    fn minimax(&mut self) -> Vec<Vec<usize>> { //TODO: rework
        // The score of a guess is the minimum number of possibilities it might eliminate.
        let minimum_eliminated = |guess: Vec<usize>| {
            let s_pass = self.original.iter().filter(|p| self.combinations.contains(p.borrow()));

            let peg_scores = s_pass.map(|possibility| score_calc(&guess, &possibility));
            let hit_count = {
                let mut map = HashMap::new();

                for bw in peg_scores {
                    *(map.entry(bw).or_insert(0)) += 1;
                }
                map
            };

            // Find the highest hit count for guess
            let highest_hit_count = hit_count.values()
                .max()
                .expect("no max hit count: empty S? already won?");
            self.combinations.len() - highest_hit_count
        };

        let append = |xs: Vec<Vec<usize>>, x| {
            let mut v = xs;
            v.push(x);
            v
        };


        let unused = |p: &Vec<usize>| !self.guessed.contains(&stringify_number(p));
        let (_, max_scoring_guesses) = self.original.clone().into_iter()
            .filter(unused)
            .fold((0, vec![]), |acc: (usize, Vec<Vec<usize>>), guess| {
                let (high_score, candidates) = acc;
                println!("{}", &candidates.len());
                let score = minimum_eliminated(Vec::from(guess.borrow()));
                match score.cmp(&high_score) {
                    Greater => (score, vec![guess]),
                    Equal => (score, append(candidates, Vec::from(guess.borrow()))),
                    _ => (high_score, candidates),
                }
            });

        max_scoring_guesses
    }


    pub fn prune(self: &mut Self, last_guess: &String, response: Vec<usize>) {
        self.guessed.push(last_guess.to_string());
        let vec_guess = vectorize_number(&last_guess);
        self.combinations.retain(&|element: &Vec<usize>| score_calc(&vec_guess, element) == response)
    }

    pub fn remove_guess(self: &mut Self, last_guess: Vec<usize>) {
        let index = self.combinations.iter().position(|x| *x == last_guess).unwrap();
        self.combinations.remove(index);
    }

    pub fn get_combos(self: &mut Self) -> Vec<Vec<usize>> {
        self.combinations.clone()
    }
}


fn stringify_number(num: &Vec<usize>) -> String {
    return num.into_iter().map(|i| i.to_string()).collect::<String>();
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