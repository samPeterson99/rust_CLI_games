extern crate termion;
use crate::Row;
use std::io::{stdin, stdout, Write};
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Terminal {
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Self {
        Self {
            _stdout: stdout().into_raw_mode().unwrap(),
        }
    }

    pub fn run(&mut self) {
        print!("start guessing six letter words");
        self._stdout.flush().unwrap();
        let mut line = 0 as usize;

        loop {
            line += 1;
            let stdin = stdin();
            let mut row_array: Vec<char> = Vec::with_capacity(5);
            print!("{}{}", termion::clear::All, Goto(1, line as u16));
            self.draw_previous_rows();
            self._stdout.flush().unwrap();
            for c in stdin.keys() {
                print!("{}", Goto(1, line as u16));
                match c.unwrap() {
                    Key::Ctrl('a') => panic!("halp"),
                    Key::Char(c) => {
                        row_array.push(c);
                    }
                    _ => print!("something else"),
                }
                for char in &row_array {
                    print!("| {} |", char)
                }
                if row_array.len() == 5 {
                    self.rows.push(Row::from(row_array));
                    break;
                }
                self._stdout.flush().unwrap();
            }

            if line == 6 {
                break;
            }
        }
    }
    pub fn draw_previous_rows(&mut self) {
        for (index, row) in self.rows.iter().enumerate() {
            print!("{}", Goto(1, index as u16 + 1));
            println!(
                "| {} || {} || {} || {} || {} |",
                row.one, row.two, row.three, row.four, row.five
            )
        }
    }
    // pub fn read_key() -> Result<Key, std::io::Error> {
    //     loop {
    //         if let Some(key) = stdin().lock().keys().next() {
    //             return key;
    //         }
    //     }
    // }
}
