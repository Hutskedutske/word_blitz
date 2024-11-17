use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
    str::Chars,
    vec,
};

fn main() {
    //initialize wordlist and paths
    let mut wordlist: HashSet<String> = HashSet::new();
    let _ = fill_set_file(&mut wordlist);
    let paths: Vec<Vec<(i32, i32)>> = pathfinder();

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

        //finds every word and adds to a list it out if it is valid
        let wordmap = word_checker(&paths, grid, &wordlist);

        for pair in wordmap{
            println!("{}\n{}", pair.0, make_grid_word(&pair.1, &pair.0));
        }
    }
}

/**finds all valid words */
fn word_checker(
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

/**Builds a grid from a string */
fn grid_builder(woord: String) -> [[char; 4]; 4] {
    //initialize grid
    let mut grid: [[char; 4]; 4] = [
        [' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' '],
    ];

    //creates iterator over characters from the string to put in the grid
    let mut chars: Chars<'_> = woord.chars();

    for x in 0..4 {
        for y in 0..4 {
            grid[x][y] = chars.next().expect("No char found");
        }
    }

    grid
}

/**Calls recursive_pathfinder on every coordinate to get every path and returns them in a list */
fn pathfinder() -> Vec<Vec<(i32, i32)>> {
    let mut all_paths: Vec<Vec<(i32, i32)>> = Vec::new();

    //loop over every starting point
    for x in 0..4 {
        for y in 0..4 {
            let mut coords = vec![(x, y)];
            let mut paths: Vec<Vec<(i32, i32)>> = Vec::new();

            //find all paths from a starting point and add them to all paths
            recursive_pathfinder(&mut coords, &mut paths, 1);
            all_paths.append(&mut paths);
        }
    }
    all_paths
}

/**Constructs all paths of length 1-5 from a given starting point */
fn recursive_pathfinder(
    visited_coords: &mut Vec<(i32, i32)>,
    paths: &mut Vec<Vec<(i32, i32)>>,
    length: usize,
) {
    //add current coordinates to visited coordinates
    paths.push(visited_coords.to_vec());
    if length < 10 {
        for x in -1..2 {
            for y in -1..2 {
                //None case never occurs
                let lastcoord: &(i32, i32) = match visited_coords.get(length - 1) {
                    Some(el) => el,
                    None => &(0, 0),
                };

                //checks if coordinate is in the grid or if it has veen visited before
                if lastcoord.0 + x < 4
                    && lastcoord.1 + y < 4
                    && lastcoord.0 + x >= 0
                    && lastcoord.1 + y >= 0
                    && !visited_coords.contains(&(lastcoord.0 + x, lastcoord.1 + y))
                {
                    //recursive backtracking
                    visited_coords.push((lastcoord.0 + x, lastcoord.1 + y));
                    recursive_pathfinder(visited_coords, paths, length + 1);
                    visited_coords.remove(length);
                }
            }
        }
    }
}

/**Constructs a string from a path in the grid */
fn word_builder(grid: [[char; 4]; 4], path: &Vec<(i32, i32)>) -> String {
    let mut word: String = String::new();
    for coord in path {
        word.push(grid[coord.0 as usize][coord.1 as usize]);
    }
    word
}

/**Fills a HashSet<String> with words from a file*/
fn fill_set_file(set: &mut HashSet<String>) {
    let filepath: &str = "resources/woordenlijst.txt";

    let file: File = match File::open(filepath) {
        Ok(file) => file,
        Err(_) => panic!("File could not be opened"),
    };

    let reader: BufReader<File> = BufReader::new(file);

    for line in reader.lines() {
        let woord = match line {
            Ok(woord) => woord,
            Err(_) => panic!("Line read failed"),
        };
        set.insert(woord);
    }
}

/**makes a string that represents the grid with the word filled in */
fn make_grid_word(path: &Vec<(i32, i32)>, word: &String) -> String {
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
