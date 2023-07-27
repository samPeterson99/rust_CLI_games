mod row;
mod terminal;
mod wordle;
pub use row::Row;
use std::env;
use terminal::Terminal;
use wordle::Wordle;

fn main() {
    let args: Vec<String> = env::args().collect();

    let term = args[1].clone();

    println!("{}", term);

    if term == "wordle" || term == "Wordle" {
        Wordle::default().run();
    } else if term == "terminal" {
        Terminal::default().run();
    } else {
        println!("nothing goin")
    }
}
