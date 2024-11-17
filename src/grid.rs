use std::{
    collections::{HashMap, HashSet},
    str::Chars,
};

/**makes a string that represents the grid with the word filled in */
pub fn word_in_grid(path: &Vec<(i32, i32)>, word: &String) -> String {
    let chars: Vec<char> = word.chars().collect();

    let mut last: String = String::new();

    //building the string
    for x in 0..4 {
        for y in 0..4 {
            //keeping the order of chars in the grid
            if path.contains(&(x, y)) {
                last.push(
                    *chars
                        .get(
                            path.iter()
                                .position(|&coords| coords == (x, y))
                                .expect("Index not found"),
                        )
                        .expect("Char not found"),
                );
            } else {
                last.push('#');
            }
        }
        //new line in the grid
        last.push('\n');
    }
    last
}

/**Builds a grid from a string */
pub fn grid_builder(input: String) -> [[char; 4]; 4] {
    //initialize grid
    let mut grid: [[char; 4]; 4] = [
        [' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' '],
    ];

    //creates iterator over characters from the string to put in the grid
    let mut chars: Chars<'_> = input.chars();

    for x in 0..4 {
        for y in 0..4 {
            grid[x][y] = chars.next().expect("No char found");
        }
    }

    grid
}

/**Constructs a string from a path in the grid */
pub fn word_builder(grid: [[char; 4]; 4], path: &Vec<(i32, i32)>) -> String {
    let mut word: String = String::new();
    for coord in path {
        word.push(grid[coord.0 as usize][coord.1 as usize]);
    }
    word
}

/**finds all valid words */
pub fn word_checker(
    paths: &Vec<Vec<(i32, i32)>>,
    grid: [[char; 4]; 4],
    wordlist: &HashSet<String>,
) -> HashMap<String, Vec<(i32, i32)>> {
    //hashmap used to remove duplicates
    let mut wordmap: HashMap<String, Vec<(i32, i32)>> = HashMap::new();

    for path in paths {
        let woord: String = word_builder(grid, path);
        if wordlist.contains(&woord) {
            wordmap.insert(woord, path.to_vec());
        }
    }

    wordmap
}
