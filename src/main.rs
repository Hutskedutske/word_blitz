use std::{
    collections::HashSet, fs::File, io::{self, BufRead, BufReader}, str::Chars, vec
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
            "String is invalid" => {println!("String is invalid");
                                     continue},
            _ => {},

        }

        let grid: [[char; 4]; 4] = grid_builder(input);

        //finds every word and prints it out if it is valid
        for path in &paths {
            let woord: String = word_builder(grid, path);
            if wordlist.contains(&woord){
                println!("{}", woord);
            }
        }

    }
}

/**Gets input and checks if it is valid */
fn input_handler() -> String {
    let mut input: String = String::new();

    //input
    println!("Enter the grid: ");

    io::stdin().read_line(&mut input)
    .expect("Failed to read line");



    //removes \n chars from input
    input = String::from(input.trim());

    //checks for validity
    if input != String::from("quit") && input.len() != 16 {
        String::from("String is invalid")
    } else {
        for char in input.chars() {
            if !char.is_alphabetic(){
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
    if length < 5 {
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
    
    let filepath: &str = "resources/woordenlijst-5.txt";

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