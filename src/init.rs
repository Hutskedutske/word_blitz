use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
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
fn recursive_pathfinder(
    path: &mut Vec<(i32, i32)>,
    paths: &mut Vec<Vec<(i32, i32)>>
) {
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

/**Fills a HashSet<String> with words from a file*/
pub fn init_wordlist() -> HashSet<String> {
    let mut woordenlijst: HashSet<String> = HashSet::new();
    let filepath: &str = "resources/woordenlijst.txt";

    let file: File = match File::open(filepath) {
        Ok(file) => file,
        Err(_) => panic!("wordlist not found at {}", filepath)
    };

    let reader: BufReader<File> = BufReader::new(file);

    for line in reader.lines() {
        let woord = match line {
            Ok(woord) => woord,
            Err(_) => panic!("Failed to read from {}", filepath)
        };
        woordenlijst.insert(woord);
    }
    woordenlijst
}
