use std::vec;

fn main() {
    println!("Hello, world!");
}

fn word_seeker(grid: [[char; 4]; 4]) -> Vec<String> {
    for x in 0..5 {
        for y in 0..5 {
            let mut coords = vec![(x, y)];
            recursive_search(&mut coords, 1);
        }
    }

    Vec::new()
}

fn recursive_search(visited_coords: &mut Vec<(i32, i32)>, length: usize) {
    if length < 3 {
        for x in -1..2 {
            for y in -1..2 {
                
                let lastcoord: &(i32, i32) = match visited_coords.get(length - 1) {
                    Some(el) => el,
                    None => &(0, 0),
                };

                if     lastcoord.0 + x < 5
                    && lastcoord.1 + y < 5
                    && lastcoord.0 + x >= 0
                    && lastcoord.1 + y >= 0
                    && visited_coords.contains(&(lastcoord.0 + x, lastcoord.1 + y))
                {
                    visited_coords.push((lastcoord.0 + x, lastcoord.1 + y));
                    recursive_search(visited_coords, length + 1);
                    visited_coords.remove(length);
                }
            }
        }
    }
}
