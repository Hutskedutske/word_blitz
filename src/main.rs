use std::vec;

fn main() {
    let grid: [[char; 4]; 4] = [['a', 'b', 'c', 'd'], 
                                ['e', 'f', 'g', 'h'],
                                ['i', 'j', 'k', 'l'],
                                ['m', 'n', 'o', 'p']];
    word_seeker(grid);
}

fn word_seeker(grid: [[char; 4]; 4]) {
    for x in 0..4 {
        for y in 0..4 {
            let mut coords = vec![(x, y)];
            let mut paths: Vec<Vec<(i32, i32)>> = Vec::new();
            recursive_pathfinder(&mut coords, &mut paths, 1);
            for path in paths {
                // println!("{:?}", path);
                word_builder(grid, path);
            }
        }
    }
}

fn recursive_pathfinder(visited_coords: &mut Vec<(i32, i32)>, paths: &mut Vec<Vec<(i32, i32)>>, length: usize) {
    paths.push(visited_coords.to_vec());
    if length > 5 {
        
        // println!("path: {:?}", visited_coords);
    }
    else {
        for x in -1..2 {
            for y in -1..2 {
                
                let lastcoord: &(i32, i32) = match visited_coords.get(length - 1) {
                    Some(el) => el,
                    None => &(0, 0),
                };

                if     lastcoord.0 + x < 4
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

fn word_builder(grid: [[char; 4]; 4], path: Vec<(i32, i32)>){
    let mut word: String= String::new();
    for coord in path {
        word.push(grid[coord.0 as usize][coord.1 as usize])
    } println!("{}", word);
}

