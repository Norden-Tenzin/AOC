use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Clone, Debug)]
struct Chara {
    pos: Pos,
    dir: Direction,
}

impl Chara {
    fn turn(&mut self) {
        match &self.dir {
            Direction::North => self.dir = Direction::East,
            Direction::South => self.dir = Direction::West,
            Direction::East => self.dir = Direction::South,
            Direction::West => self.dir = Direction::North,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Pos(usize, usize);

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn new() -> Self {
        return Direction::North;
    }

    fn char_to_dir(ch: char) -> Self {
        match ch {
            '^' => Direction::North,
            'v' => Direction::South,
            '<' => Direction::West,
            '>' => Direction::East,
            _ => Direction::North,
        }
    }

    fn dir_to_char(&self) -> char {
        match self {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::East => '>',
            Direction::West => '<',
        }
    }
}

pub fn problem1(path: &str) -> usize {
    let file = fs::read_to_string(path).expect("unable to read file.");
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut res: usize = 1;

    // character
    let mut chara: Chara = Chara {
        pos: Pos(0, 0),
        dir: Direction::new(),
    };

    for line in file.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    chara = find_start(&grid);
    println!("{:?}", chara);

    visited.insert(chara.pos.clone());

    loop {
        if let Ok(pos) = check_valid_move(&chara, &grid) {
            //println!("{:?}", grid[chara.pos.0][chara.pos.1]);
            if grid[pos.0][pos.1] == '.' {
                // move one step
                grid[chara.pos.0][chara.pos.1] = '.';
                println!("{:?}", chara.pos);
                chara.pos = pos;
                println!("{:?}", chara.pos);
                grid[chara.pos.0][chara.pos.1] = chara.dir.dir_to_char();
                visited.insert(chara.pos.clone());
                res += 1;
            } else {
                // turn
                println!("TURN {:?}", chara.dir);
                chara.turn();
                grid[chara.pos.0][chara.pos.1] = chara.dir.dir_to_char();
                println!("TURN {:?}", chara.dir);
            }
        } else {
            break;
        }
    }

    fn find_start(grid: &Vec<Vec<char>>) -> Chara {
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if ['^', 'v', '<', '>'].contains(&grid[row][col]) {
                    return Chara {
                        pos: Pos(row, col),
                        dir: Direction::char_to_dir(grid[row][col]),
                    };
                }
            }
        }
        return Chara {
            pos: Pos(0, 0),
            dir: Direction::new(),
        };
    }

    fn check_valid_move(chara: &Chara, grid: &Vec<Vec<char>>) -> Result<Pos, ()> {
        let mut curr_pos = chara.pos.clone();
        match chara.dir {
            Direction::North => curr_pos.0 -= 1,
            Direction::South => curr_pos.0 += 1,
            Direction::East => curr_pos.1 += 1,
            Direction::West => curr_pos.1 -= 1,
        }
        if (0..grid.len()).contains(&curr_pos.0) && (0..grid[0].len()).contains(&curr_pos.1) {
            return Ok(curr_pos);
        } else {
            return Err(());
        }
    }

    println!("set: {}", visited.len());
    return res;
}

pub fn problem2(path: &str) -> usize {
    let file = fs::read_to_string(path).expect("unable to read file.");
    let res: usize = 0;

    for line in file.lines() {}

    return res;
}
