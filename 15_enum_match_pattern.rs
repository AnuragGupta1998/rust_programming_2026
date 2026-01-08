//enum
enum Direction {
    North,
    East,
    South,
    West,
}

fn move_direction(direction: Direction) {
    match direction {
        Direction::North => println!("Moving North"),
        Direction::East => println!("Moving East"),
        Direction::South => println!("Moving South"),
        Direction::West => println!("Moving West"),
    }
}

fn main() {
    //assigning enum value to a variable
    let dir = Direction::West;

    let new_direction = dir;

    move_direction(new_direction);
    move_direction(Direction::East);
    move_direction(Direction::North);
    move_direction(Direction::South);
}
