use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    let mut position = Point { x: 0, y: 0 };
    let mut direction = Direction::North;
    let mut max_distance_sq = 0;

    let obstacle_set: HashSet<Point> = obstacles
        .into_iter()
        .map(|obs| Point {
            x: obs[0],
            y: obs[1],
        })
        .collect();

    for cmd in commands {
        match cmd {
            -2 => direction = turn_left(direction),
            -1 => direction = turn_right(direction),
            1..=9 => {
                for _ in 0..cmd {
                    let next_position = move_forward(position, direction);
                    if !obstacle_set.contains(&next_position) {
                        position = next_position;
                        max_distance_sq = max_distance_sq.max(distance_squared(position));
                    } else {
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    max_distance_sq
}

fn turn_left(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::West,
        Direction::West => Direction::South,
        Direction::South => Direction::East,
        Direction::East => Direction::North,
    }
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn move_forward(position: Point, direction: Direction) -> Point {
    match direction {
        Direction::North => Point {
            x: position.x,
            y: position.y + 1,
        },
        Direction::East => Point {
            x: position.x + 1,
            y: position.y,
        },
        Direction::South => Point {
            x: position.x,
            y: position.y - 1,
        },
        Direction::West => Point {
            x: position.x - 1,
            y: position.y,
        },
    }
}

fn distance_squared(point: Point) -> i32 {
    point.x * point.x + point.y * point.y
}
