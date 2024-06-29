use anyhow::Result;

const LETTERS: [char; 10] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];

/// Status of cell on board
enum Status {
    Empty,
    Missed,
    Intact,
    Hit,
    Sunken,
}

struct Ship {
    length: u8,
    start: (u8, u8)
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
    let stdin = stdin::io::stdin();
    for _ in (0..10) {
        stdin.read_line(&mut board_input)?;
    }

}