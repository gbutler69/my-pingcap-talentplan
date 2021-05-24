use std::{fs::File, io, path::Path};

use serde::{Deserialize, Serialize};

fn main() -> Result<(), io::Error> {
    let move_a = Move {
        direction: Direction::North,
        count: 3,
    };

    let file = serialize(&move_a, &Path::new("./tmp.txt"))?;
    let move_b = deserialize(file)?;

    println!("Move A = {:?}", move_a);
    println!("Move B = {:?}", move_b);

    assert_eq!(move_a, move_b);

    Ok(())
}

fn serialize(the_move: &Move, path: &Path) -> Result<File, io::Error> {
    serde_json::to_writer(io::BufWriter::new(File::create(path)?), the_move)?;
    File::open(path)
}

fn deserialize(file: File) -> Result<Move, io::Error> {
    let u = serde_json::from_reader(io::BufReader::new(file))?;
    Ok(u)
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct Move {
    pub direction: Direction,
    pub count: u8,
}
