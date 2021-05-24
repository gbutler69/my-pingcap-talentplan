use std::error::Error;

use serde::{Deserialize, Serialize};

fn main() -> Result<(), Box<dyn Error>> {
    let move_a = Move {
        direction: Direction::North,
        count: 3,
    };

    let serialized_value = serialize(&move_a)?;
    let move_b = deserialize(serialized_value)?;

    println!("Move A = {:?}", move_a);
    println!("Move B = {:?}", move_b);

    assert_eq!(move_a, move_b);

    Ok(())
}

fn serialize(the_move: &Move) -> Result<String, Box<dyn Error>> {
    let serialized = ron::ser::to_string(the_move)?;
    Ok(serialized)
}

fn deserialize(serialized_value: String) -> Result<Move, Box<dyn Error>> {
    let the_move = ron::de::from_str(&serialized_value)?;
    Ok(the_move)
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
