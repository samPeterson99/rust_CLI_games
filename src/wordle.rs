use std::io;

pub struct Wordle {
    pub term: String,
    pub tries: usize,
    pub term_collection: Vec<char>,
}

impl Wordle {
    pub fn default() -> Self {
        let mut input = String::new();

        println!("enter a word for other's to solve: ");
        io::stdin().read_line(&mut input).expect("what?");
        let trimmed_input = String::from(input.trim());
        let length = trimmed_input.len();
        let collection: Vec<char> = trimmed_input.chars().collect();
        Self {
            term: trimmed_input,
            tries: length,
            term_collection: collection,
        }
    }
    pub fn run(&mut self) {
        println!("{}", self.term);
        println!("{}", self.tries);
    }
}
