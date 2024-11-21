use std::collections::HashSet;

mod grid;
mod init;

use grid::*;
use init::*;

fn main() {
    //initialize wordlist and paths
    let wordlist: HashSet<String> = init_language();
    let paths: Vec<Vec<(i32, i32)>> = init_paths();

    //execution loop
    loop {
        let input: String = grid_input_handler();

        match input.as_str() {
            "QUIT" => break,
            "invalid" => {
                println!("String is invalid");
                continue;
            }
            _ => {}
        }

        let grid: [[char; 4]; 4] = grid_builder(input);

        //finds every valid word and it's representation in the grid
        let valid_words: Vec<(String, String)> = word_checker(&paths, grid, &wordlist);

        for pair in valid_words {
            println!("{}\n{}", pair.0, pair.1);
        }
    }
}

