use std::{
    collections::HashSet, fs::File, io::{BufRead, BufReader}
};

/**Calls recursive_pathfinder on every coordinate to get every path and returns them in a list */
pub fn init_paths() -> Vec<Vec<(i32, i32)>> {
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

/**Fills a HashSet<String> with words from a file*/
pub fn init_wordlist() -> HashSet<String>{

    let mut woordenlijst: HashSet<String> = HashSet::new();
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
        woordenlijst.insert(woord);
    }
    woordenlijst
}
