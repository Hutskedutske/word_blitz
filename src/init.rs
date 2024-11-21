use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
};

/**Calls recursive_pathfinder on every coordinate to get every path and returns them in a list */
pub fn init_paths() -> Vec<Vec<(i32, i32)>> {
    let mut paths: Vec<Vec<(i32, i32)>> = Vec::new();

    //loop over every starting point
    for x in 0..4 {
        for y in 0..4 {
            let mut coords = vec![(x, y)];
            //find all paths from a starting point and add to paths
            recursive_pathfinder(&mut coords, &mut paths);
        }
    }
    paths
}

/**Constructs all paths of length 1-5 from a given starting point */
fn recursive_pathfinder(path: &mut Vec<(i32, i32)>, paths: &mut Vec<Vec<(i32, i32)>>) {
    //adds current path to all paths
    paths.push(path.to_vec());
    if path.len() < 10 {
        for x in -1..2 {
            for y in -1..2 {
                let lastcoord: &(i32, i32) = path.last().expect("No coordinate was found in path");

                //checks if coordinate is in the grid or if it has veen visited before
                if lastcoord.0 + x < 4
                    && lastcoord.1 + y < 4
                    && lastcoord.0 + x >= 0
                    && lastcoord.1 + y >= 0
                    && !path.contains(&(lastcoord.0 + x, lastcoord.1 + y))
                {
                    //recursive backtracking
                    path.push((lastcoord.0 + x, lastcoord.1 + y));
                    recursive_pathfinder(path, paths);
                    path.remove(path.len() - 1);
                }
            }
        }
    }
}

//*lets the user choose a language and initializes the wordlist */
pub fn init_language() -> HashSet<String> {
    let mut input: String = String::new();

    let mut filepath: &str = "none";

    //map of the available languages
    let language_map: HashMap<&str, &str> =
        [("NL", "resources/nl.txt"),
         ("EN", "resources/en.txt"),
         ("FR", "resources/fr.txt"),
         ("ES", "resources/es.txt")
        ].iter().cloned().collect();

    //loop to get input of a valid language
    while filepath == "none" {
        println!("Choose a language: EN, ES, FR, NL");

        //read input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        //formats input to select in the map
        input = input.to_ascii_uppercase();

        let trimmed_input: &str = input.trim();

        filepath = match language_map.get(trimmed_input){
            Some(path) => path,
            None => {println!("language not included");
                     continue}
        };
    }

    init_wordlist(filepath)
}

/**Fills a HashSet<String> with words from a file*/
pub fn init_wordlist(filepath: &str) -> HashSet<String> {
    let mut woordenlijst: HashSet<String> = HashSet::new();

    let file: File = match File::open(filepath) {
        Ok(file) => {file},
        Err(_) => panic!("wordlist not found at {}", filepath),
    };

    let reader: BufReader<File> = BufReader::new(file);

    for line in reader.lines() {
        let woord = match line {
            Ok(woord) => woord,
            Err(_) => panic!("Failed to read from {}", filepath),
        };
        woordenlijst.insert(woord);
    }
    woordenlijst
}
