use crate::vectorize_number;
use std::collections::HashMap;

static mut COMBO_GEN: Vec<Vec<usize>> = Vec::new(); //TODO: make this local to Codebreaker

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
        return unsafe { COMBO_GEN.clone() };
    }

    pub fn constructor(length: usize) -> CodeBreaker {
        return CodeBreaker {
            secret_length: length,
            guessed: vec![],
            combinations: CodeBreaker::create_combos(length),
        };
    }

    pub fn play(self: &mut Self, last_guess: &String, last_score: Vec<usize>) -> String {
        self.remove_guess(vectorize_number(&last_guess));
        println!("past remove");
        self.prune(&last_guess.to_string(), last_score);
        println!("past pruning. length of combinations: {}", self.combinations.len());

        let guess = self.next_guess().to_string();
        println!("second. Guessed: {:?}", self.guessed);
        return guess;
    }


    pub fn next_guess(&self) -> String {
        let next_guesses = self.minimax() ;

        let next_guess = next_guesses.first().unwrap();

        let result: String = next_guess.iter().fold(String::new(), |mut result, x| {
            result.push_str(&x.to_string());
            result
        });
        return result;
    }

    fn minimax(self: &Self) -> Vec<Vec<usize>> { //TODO: rework
        let mut score_count: HashMap<Vec<usize>, usize> = HashMap::new();
        let mut score: HashMap<Vec<usize>, usize> = HashMap::new();
        let mut next_guesses: Vec<Vec<usize>> = Vec::new();

        let combos = unsafe { COMBO_GEN.clone() };
        let mut max: usize = 0;
        let mut min: usize = usize::max_value();

        println!("length of combos: {}", combos.len());

        for combo in combos.iter() {
            for pruned in self.combinations.iter() {
                let this_score = score_calc(&combo, &pruned); //comp scores
                *(score_count.entry(this_score).or_insert(0)) += 1; //add to scoring list
            }
            //find the max count for this scoring result
            for elem in score_count.iter() {
                if elem.1 > &max {
                    max = *elem.1;
                }
            }
            score.insert(combo.to_vec(), max); //insert into max count for this combo
            score_count.clear()
        }
        println!("past score max counter");

        //Find the minimum count
        for elem in score.iter() {
            if elem.1 < &min {
                min = *elem.1;
            }
        }

        //Find guesses with min count
        for elem in score.iter() {
            if elem.1 == &min {
                next_guesses.push(elem.0.to_vec());
            }
        }
        next_guesses
    }


    pub fn prune(self: &mut Self, last_guess: &String, response: Vec<usize>) {
        self.guessed.push(last_guess.to_string());
        let vec_guess = vectorize_number(&last_guess);
        self.combinations.retain(&|element: &Vec<usize>| score_calc(&vec_guess, element) == response)
    }

    pub fn remove_guess(self: &mut Self, last_guess: Vec<usize>) {
        let index = self.combinations.iter().position(|x| *x == last_guess).unwrap();
        self.combinations.remove(index);
        unsafe { //TODO: remove after rework
            let index = COMBO_GEN.iter().position(|x| *x == last_guess).unwrap();
            COMBO_GEN.remove(index);
        }
    }

    pub fn get_combos(self: &mut Self) -> Vec<Vec<usize>> {
        self.combinations.clone()
    }
}

fn combo_recur(combination_length: usize, element: usize, current: &Vec<usize>, digits: &Vec<usize>) {
    let mut current_copy = current.to_vec();
    if element >= combination_length {
        unsafe {
            COMBO_GEN.push(current_copy);
        }
        return;
    }
    for i in 0..digits.len() {
        current_copy[element] = digits[i];
        combo_recur(combination_length, element + 1, &mut current_copy, digits);
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