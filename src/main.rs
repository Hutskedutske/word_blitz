use std::{collections::HashSet, vec};

use rusqlite::Connection;

fn main() {
    let grid: [[char; 4]; 4] = [
        ['A', 'B', 'C', 'D'],
        ['E', 'F', 'G', 'H'],
        ['I', 'J', 'K', 'L'],
        ['M', 'N', 'O', 'P'],
    ];

    let mut woorden: HashSet<String> = HashSet::new();
    let _ = fill_set(&mut woorden);
    word_seeker(grid, woorden);
}

fn word_seeker(grid: [[char; 4]; 4], wordlist: HashSet<String>) {
    for x in 0..4 {
        for y in 0..4 {
            let mut coords = vec![(x, y)];
            let mut paths: Vec<Vec<(i32, i32)>> = Vec::new();
            recursive_pathfinder(&mut coords, &mut paths, 1);
            for path in paths {
                let woord: String = word_builder(grid, path);
                if wordlist.contains(&woord) {
                    println!("{}", woord);
                }
            }
        }
    }
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

fn word_builder(grid: [[char; 4]; 4], path: Vec<(i32, i32)>) -> String {
    let mut word: String = String::new();
    for coord in path {
        word.push(grid[coord.0 as usize][coord.1 as usize]);
    }
    word
}

fn fill_set(set: &mut HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
    let path: &str = "D:\\SQLite\\word_blitz.db";

    let conn: Connection = Connection::open(path)?;

    let mut stmt = conn.prepare("SELECT * FROM words")?;

    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        set.insert(row.get(0)?);
    }

    Ok(())
}
