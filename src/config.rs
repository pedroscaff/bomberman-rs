use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use crate::state::{Tile, TileStatus};

pub fn read_map(filepath: &str) -> io::Result<[[Tile; 10]; 10]> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut map: [[Tile; 10]; 10] = [[Tile { status: TileStatus::FREE }; 10]; 10];
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (j, c) in line.chars().enumerate() {
            match c {
                '-' => map[i][j] = Tile { status: TileStatus::FREE },
                '#' => map[i][j] = Tile { status: TileStatus::WALL },
                _ => panic!("unknown symbol {}", c)
            }
        }
    }

    Ok(map)
}
