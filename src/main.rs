use anyhow::Result;

const LETTERS: [char; 10] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];

/// Status of cell on board
#[derive(Clone, Copy, Debug)]
enum Status {
    Empty,
    Missed,
    Intact(u8),
    Hit(u8),
    Sunken(u8),
}

enum Orientation {
    Horizontal,
    Vertical,
}

struct Ship {
    length: u8,
    start: (u8, u8),
    orientation: Orientation,
}

// type Board = [[Status; 10]; 10];
struct Board {
    cells: [[Status; 10]; 10],
    ships: Vec<Ship>,
}

struct Game {
    player1_board: Board,
    player2_board: Board,
}

impl Game {
    fn new() -> Result<Game> {
        
    }
}

fn main() {
    println!("Welcome to Battleship!")

}

fn create_board(player_num: u8) -> Result<Board> {
    println!("Create a board!");
    let mut board_input = String::with_capacity(112);
    let stdin = std::io::stdin();
    for _ in (0..10) {
        stdin.read_line(&mut board_input)?;
    }

    let board: [[Status; 10]; 10] = board_input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '-' => Status::Empty,
                    n if n.is_numeric() => Status::Intact(n.to_digit(10).unwrap()),
                    _ => panic!("Invalid character entered!"),
                }).collect::<Vec<_>>()
                .try_into()
                .unwrap()
        }).collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let mut ships: Vec<Ship> = Vec::with_capacity(5);
    let mut seen_ships: Vec<u8> = Vec::with_capacity(5);
    for (i, row) in board.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let ship_num = match cell { 
                Status::Hit(n) => n,
                Status::Intact(n) => n,
                Status::Sunken(n) => n,
                _ = continue,
            };
            let to_right = board.get(i + 1).map(|r| r.get(j)).flatten()
        }
    }
    todo!()
}