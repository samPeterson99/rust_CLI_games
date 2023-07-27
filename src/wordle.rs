use crate::terminal;
use crate::Row;
use std::io;
use terminal::Terminal;

pub struct Wordle {
    pub term: String,
    pub rounds: usize,
    pub term_collection: Vec<char>,
    rows: Vec<Row>,
    revealer_array: Vec<i32>,
}

impl Wordle {
    pub fn default() -> Self {
        Terminal::default();
        // println!("enter a word for other's to solve: ");
        // io::stdin().read_line(&mut input).expect("what?");
        let trimmed_input = String::from("honey");
        let length = trimmed_input.len();
        let collection: Vec<char> = trimmed_input.chars().collect();
        let int_array = Self::make_revealer_array(length);
        Self {
            term: trimmed_input,
            rounds: length,
            term_collection: collection,
            rows: Vec::with_capacity(6),
            revealer_array: int_array,
        }
    }
    pub fn run(&mut self) {
        let mut tries = 0;

        loop {
            tries += 1;
            let mut attempt = String::new();
            println!("The word is {} letters long and you have {} chances to solve it. Begin typing guesses.", self.rounds, self.rounds);
            io::stdin()
                .read_line(&mut attempt)
                .expect("word not recognized");
            let attempt_array: Vec<char> = attempt.trim().chars().collect();

            for index in 0..self.rounds {
                if attempt_array[index] == self.term_collection[index] {
                    self.revealer_array[index] = 2;
                } else if self.term_collection.contains(&attempt_array[index]) {
                    self.revealer_array[index] = 1;
                }
            }

            dbg!(&self.term_collection);
            dbg!(attempt_array);
            dbg!(&self.revealer_array);

            if tries == self.rounds {
                println!("out of chances");
                break;
            }
            if self.revealer_array.iter().all(|&x| x == 2) {
                println!("you won in {} tries", tries);
                break;
            }
            println!("fin");
        }
    }
    fn make_revealer_array(x: usize) -> Vec<i32> {
        let mut result = Vec::with_capacity(x);

        for _ in 0..x {
            result.push(0);
        }
        result
    }
}
