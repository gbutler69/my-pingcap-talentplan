use std::{error::Error, fs, io, path::Path};

use serde::{Deserialize, Serialize};

fn main() -> Result<(), Box<dyn Error>> {
    let moves = &mut Vec::<Move>::new();
    let db = &mut Database::open_new_or_truncate(&Path::new("./temp.database.kvs"))?;

    let mut current_direction = Direction::North;
    for i in 0..1000 {
        let the_move = Move {
            number: i,
            direction: current_direction,
            count: i % u8::MAX as i32,
        };
        db.write(&the_move)?;
        moves.push(the_move);
        current_direction = next_direction_clockwise(current_direction);
    }

    let mut i = 0;
    while let Some(the_move) = db.read_next()? {
        println!("Move A = {:?}", moves[i]);
        println!("Move B = {:?}", the_move);
        assert_eq!(moves[i], the_move);
        i += 1;
    }

    Ok(())
}

fn next_direction_clockwise(current_direction: Direction) -> Direction {
    match current_direction {
        Direction::North => Direction::NorthEast,
        Direction::NorthEast => Direction::East,
        Direction::East => Direction::SouthEast,
        Direction::SouthEast => Direction::South,
        Direction::South => Direction::SouthWest,
        Direction::SouthWest => Direction::West,
        Direction::West => Direction::NorthWest,
        Direction::NorthWest => Direction::North,
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
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
    pub number: i32,
    pub direction: Direction,
    pub count: i32,
}

struct Database<'a> {
    reader: Box<dyn io::Read + 'a>,
    writer: Box<dyn io::Write + 'a>,
}

impl<'a> Database<'a> {
    pub fn open_new_or_truncate(path: &'a Path) -> Result<Database, Box<dyn Error>> {
        let writer = Box::new(io::BufWriter::new(
            fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(path)?,
        ));
        let reader = Box::new(io::BufReader::new(
            fs::OpenOptions::new().read(true).open(path)?,
        ));
        Ok(Self { reader, writer })
    }
    pub fn write(&mut self, the_move: &Move) -> Result<(), Box<dyn Error>> {
        bson::to_document(&the_move)?.to_writer(&mut self.writer)?;
        self.writer.flush()?;
        Ok(())
    }
    pub fn read_next(&mut self) -> Result<Option<Move>, Box<dyn Error>> {
        let doc = match bson::Document::from_reader(&mut self.reader) {
            Ok(result) => result,
            Err(bson::de::Error::IoError(err @ io::Error { .. }))
                if err.kind() == io::ErrorKind::UnexpectedEof =>
            {
                return Ok(None)
            }
            Err(err) => return Err(Box::new(err)),
        };
        let result: Move = bson::de::from_document(doc)?;
        Ok(Some(result))
    }
}
