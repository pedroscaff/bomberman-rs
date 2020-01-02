use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use crate::state::{Tile, TileStatus, MapTiles};

pub fn read_map(filepath: &str) -> io::Result<MapTiles> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut map: MapTiles = [[Tile {
        status: TileStatus::Free,
        coordinates: [0, 0],
    }; 11]; 13];
    for (j, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (i, c) in line.chars().enumerate() {
            match c {
                '-' => {
                    map[i][j] = Tile {
                        status: TileStatus::Free,
                        coordinates: [i, j],
                    }
                }
                '#' => {
                    map[i][j] = Tile {
                        status: TileStatus::PermanentWall,
                        coordinates: [i, j],
                    }
                }
                '0' => {
                    map[i][j] = Tile {
                        status: TileStatus::Wall,
                        coordinates: [i, j],
                    }
                }
                _ => panic!("unknown symbol {}", c),
            }
        }
    }

    Ok(map)
}
