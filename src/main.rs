use std::{
    collections::HashSet, fs::File, io::{self, BufRead, BufReader}, str::Chars, vec
};

use rusqlite::Connection;

fn main() {
    let mut wordlist: HashSet<String> = HashSet::new();
    let _ = fill_set_file(&mut wordlist);
    let paths: Vec<Vec<(i32, i32)>> = pathfinder();

    loop {
        let mut woord: String = String::new();

        println!("Enter the grid: ");

    io::stdin().read_line(&mut woord)
        .expect("Failed to read line");

        woord = String::from(woord.trim());

        let grid: [[char; 4]; 4] = grid_builder(woord);

        for path in &paths {
            let woord: String = word_builder(grid, path);
            if wordlist.contains(&woord){
                println!("{}", woord);
            }
        }

    }
}

fn grid_builder(woord: String) -> [[char; 4]; 4] {
    println!("{}", woord.len());

    if woord.len() != 16 {
        panic!("grid is niet 16 characters lang");
    }

    let capwoord = woord.to_ascii_uppercase();

    let chars = capwoord.chars();

    for char in chars {
        if !char.is_alphabetic() {
            panic!("niet alle chars zijn letters")
        }
    }

    let mut chars: Chars<'_> = woord.chars();

    let mut grid: [[char; 4]; 4] = [
        [' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' '],
    ];
    

    for x in 0..4 {
        for y in 0..4 {
            grid[x][y] = chars.next().expect("no char found");
        }
    }

    grid
}

fn pathfinder() -> Vec<Vec<(i32, i32)>> {
    let mut all_paths: Vec<Vec<(i32, i32)>> = Vec::new();

    for x in 0..4 {
        for y in 0..4 {
            let mut coords = vec![(x, y)];
            let mut paths: Vec<Vec<(i32, i32)>> = Vec::new();
            recursive_pathfinder(&mut coords, &mut paths, 1);
            all_paths.append(&mut paths);
        }
    }
    all_paths
}

fn recursive_pathfinder(
    visited_coords: &mut Vec<(i32, i32)>,
    paths: &mut Vec<Vec<(i32, i32)>>,
    length: usize,
) {
    paths.push(visited_coords.to_vec());
    if length < 5 {
        for x in -1..2 {
            for y in -1..2 {
                let lastcoord: &(i32, i32) = match visited_coords.get(length - 1) {
                    Some(el) => el,
                    None => &(0, 0),
                };

                if lastcoord.0 + x < 4
                    && lastcoord.1 + y < 4
                    && lastcoord.0 + x >= 0
                    && lastcoord.1 + y >= 0
                    && !visited_coords.contains(&(lastcoord.0 + x, lastcoord.1 + y))
                {
                    visited_coords.push((lastcoord.0 + x, lastcoord.1 + y));
                    recursive_pathfinder(visited_coords, paths, length + 1);
                    visited_coords.remove(length);
                }
            }
        }
    }
}

fn word_builder(grid: [[char; 4]; 4], path: &Vec<(i32, i32)>) -> String {
    let mut word: String = String::new();
    for coord in path {
        word.push(grid[coord.0 as usize][coord.1 as usize]);
    }
    word
}

fn fill_set_file(set: &mut HashSet<String>) {
    let filepath: &str = "resources/woordenlijst-5.txt";

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(_) => panic!("file could not be opened"),
    };

    let reader: BufReader<File> = BufReader::new(file);

    for line in reader.lines() {
        let woord = match line {
            Ok(woord) => woord,
            Err(_) => panic!("line read failed"),
        };
        set.insert(woord);
    }
}