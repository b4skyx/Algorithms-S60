use std::collections::HashSet;
use std::io;

struct Board {
    tile: Vec<Tile>,
}

struct Tile {
    tile_num: u8,
    moves: u8,
    snake: Option<u8>,
    ladder: Option<u8>,
}
impl Tile {
    fn new(tile_num: u8) -> Tile {
        Tile {
            tile_num: tile_num,
            moves: 0,
            snake: None,
            ladder: None,
        }
    }
    fn add_snake(&mut self, end: u8) {
        self.snake = Some(end);
    }
    fn add_ladder(&mut self, end: u8) {
        self.ladder = Some(end);
    }
}

impl Board {
    fn new() -> Board {
        Board {
            tile: (0..101).map(|x| Tile::new(x)).collect(),
        }
    }
    fn add_snake(&mut self, from: u8, to: u8) {
        self.tile[from as usize].add_snake(to);
    }
    fn add_ladder(&mut self, from: u8, to: u8) {
        self.tile[from as usize].add_ladder(to);
    }
}

fn read_input() -> Vec<u8> {
    let mut ipt = String::new();
    io::stdin().read_line(&mut ipt).expect("Err");
    ipt.split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<u8>>()
}

fn snakes_and_ladders() {
    let mut board = Board::new();

    let num_ladders = read_input()[0];
    for _ in 0..num_ladders {
        let ladder = read_input();
        board.add_ladder(ladder[0], ladder[1]);
    }
    // println!("Ladders Added");

    let num_snakes = read_input()[0];
    for _ in 0..num_snakes {
        let snake = read_input();
        board.add_snake(snake[0], snake[1]);
    }

    // println!("Snakes Added");
    let mut bfs = vec![1];
    let mut visited_tiles = HashSet::new();

    while !bfs.is_empty() {
        let tile_num = bfs.remove(0);
        let current = &board.tile[tile_num];
        let moves = current.moves;
        // println!("Tile: {}, Moves:{}", tile_num, moves);
        if !visited_tiles.contains(&tile_num) {
            visited_tiles.insert(tile_num);
            if current.tile_num == 100 {
                println!("{}\n", current.moves);
                return;
            } else {
                for i in (tile_num + 1)..(tile_num + 7) {
                    if !visited_tiles.contains(&i) {
                        if i < 101 {
                            let mut c = i;
                            if let Some(x) = board.tile[i].ladder {
                                c = x as usize;
                            }
                            if let Some(x) = board.tile[i].snake {
                                c = x as usize;
                            }
                            bfs.push(c);
                            if board.tile[c].moves == 0 || board.tile[c].moves > moves + 1 {
                                board.tile[c].moves = moves + 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("-1");
}

fn main() {
    for _ in 0..(read_input()[0]) {
        snakes_and_ladders();
    }
}
