use std::{
    collections::HashSet,
    io,
};
mod init;
use init::*;
mod grid;
use grid::*;

fn main() {
    //initialize wordlist and paths
    let wordlist: HashSet<String> = init_wordlist();
    let paths: Vec<Vec<(i32, i32)>> = init_paths();

    //execution loop
    loop {
        let input: String = input_handler();

        match input.as_str() {
            "QUIT" => break,
            "String is invalid" => {
                println!("String is invalid");
                continue;
            }
            _ => {}
        }

        let grid: [[char; 4]; 4] = grid_builder(input);

        //finds every valid word and it's representation in the grid
        let woorden = word_checker(&paths, grid, &wordlist);

        for pair in woorden {
            println!("{}\n{}", pair.0, pair.1);
        }
    }
}

/**Gets input and checks if it is valid */
fn input_handler() -> String {
    let mut input: String = String::new();

    //input
    println!("Enter the grid: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    //removes \n chars from input
    input = String::from(input.trim());

    //checks for validity
    if input != String::from("quit") && input.len() != 16 {
        String::from("String is invalid")
    } else {
        for char in input.chars() {
            if !char.is_alphabetic() {
                return String::from("String is invalid");
            }
        }
        input.to_ascii_uppercase()
    }
}
