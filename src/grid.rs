use std::{
    collections::{HashMap, HashSet},
    str::Chars,
};

/**makes a string that represents the grid with the word filled in */
pub fn word_in_grid(path: &Vec<(i32, i32)>, word: &String) -> String {
    let chars: Vec<char> = word.chars().collect();

    //representation of the grid
    let mut grid_repr: String = String::new();

    //building the string
    for x in 0..4 {
        for y in 0..4 {
            //keeping the order of chars in the grid
            if path.contains(&(x, y)) {
                grid_repr.push(
                    *chars
                        .get(
                            path.iter()
                                .position(|&coords| coords == (x, y))
                                .expect("Index not found"),
                        )
                        .expect("Char not found"),
                );
            } else {
                grid_repr.push('#');
            }
        }
        //new line in the grid
        grid_repr.push('\n');
    }
    grid_repr
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

/**finds all valid words and returns a list with words and their representation in the grid sorted by length*/
pub fn word_checker(
    paths: &Vec<Vec<(i32, i32)>>,
    grid: [[char; 4]; 4],
    wordlist: &HashSet<String>,
) -> Vec<(String, String)> {

    //hashmap used to remove duplicates
    let mut wordmap: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
    let mut sorted_words: Vec<(String, String)> = Vec::new();

    for path in paths {
        let woord: String = word_builder(grid, path);
        if wordlist.contains(&woord) && !wordmap.contains_key(&woord) {
            wordmap.insert(woord, path.to_vec());
        }
    }

    for word in wordmap.keys() {
        sorted_words.push((word.clone(), word_in_grid( wordmap.get(word).expect("Did not find word in hashmap"), word)));
    }

    sorted_words.sort_by(|a, b| a.0.len().cmp(&b.0.len()));

    sorted_words
}
