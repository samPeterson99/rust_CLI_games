mod wordle;
use std::env;
use wordle::Wordle;

fn main() {
    let args: Vec<String> = env::args().collect();

    let term = args[1].clone();

    println!("{}", term);

    if term == "wordle" || term == "Wordle" {
        Wordle::default().run();
    } else {
        println!("nothing goin")
    }
}
