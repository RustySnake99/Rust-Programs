use std::io::{self, Write};
use crossterm::{style::{Color, ResetColor, SetForegroundColor}, ExecutableCommand};
#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    X,
    O,
}
#[derive(Clone, Copy, PartialEq)]
enum Player {
    Human,
    AI,
}

type Board = [[Cell; 3]; 3];

fn display_board(board: &Board) {
    let mut stdout = io::stdout();
    println!("\nBoard:");
    for i in 0..3 {
        for j in 0..3 {
            let symbol = match board[i][j] {
                Cell::Empty => ". ".to_string(),
                Cell::X => {
                    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
                    "X ".to_string()
                }
                Cell::O => {
                    stdout.execute(SetForegroundColor(Color::Red)).unwrap();
                    "O ".to_string()
                }
            };
            print!("{}", symbol);
            stdout.execute(ResetColor).unwrap();
        }
        println!();
    }
    println!();
}

fn evaluate(board: &Board) -> Option<Cell> {
    for i in 0..3 {
        if board[i][0] != Cell::Empty && board[i][0] == board[i][1] && board[i][1] == board[i][2] {
            return Some(board[i][0]);
        }
        if board[0][i] != Cell::Empty && board[0][i] == board[1][i] && board[1][i] == board[2][i] {
            return Some(board[0][i]);
        }
    }
    if board[0][0] != Cell::Empty && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return Some(board[0][0]);
    }
    if board[0][2] != Cell::Empty && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return Some(board[0][2]);
    }
    if board.iter().all(|r| r.iter().all(|&c| c != Cell::Empty)) {
        return Some(Cell::Empty);
    }
    None
}

fn score_for(cell: Cell) -> i32 {
    match cell {
        Cell::O => 1,
        Cell::X => -1,
        _ => 0,
    }
}

fn make_move(board: &mut Board, is_human: bool, r: usize, c: usize) {
    if is_human {
        board[r][c] = Cell::X;
    } else {
        board[r][c] = Cell::O;
    }
}

fn minmax(board: &mut Board, depth: i32, is_maximizing: bool) -> i32 {
    if let Some(result) = evaluate(board) {
        return score_for(result);
    }
    if is_maximizing {
        let mut best_score = i32::MIN;
        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == Cell::Empty {
                    make_move(board, false, i, j);
                    let score = minmax(board, depth + 1, false);
                    board[i][j] = Cell::Empty;
                    best_score = best_score.max(score);
                }
            }
        }
        best_score
    } else {
        let mut best_score = i32::MAX;
        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == Cell::Empty {
                    make_move(board, true, i, j);
                    let score = minmax(board, depth + 1, true);
                    board[i][j] = Cell::Empty;
                    best_score = best_score.min(score);
                }
            }
        }
        best_score
    }
}

fn best_move(board: &mut Board) -> (usize, usize) {
    let mut best_score = i32::MIN;
    let mut best_move = (0, 0);

    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == Cell::Empty {
                make_move(board, false, i, j);
                let score = minmax(board, 0, false);
                board[i][j] = Cell::Empty;
                if score > best_score {
                    best_score = score;
                    best_move = (i, j);
                }
            }
        }
    }
    best_move
}

fn get_human_move(board: &Board) -> (usize, usize) {
    let mut input = String::new();
    loop {
        print!("Enter Row of your move (1, 2 or 3): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let row: usize = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {println!("Invalid move...."); continue;}
        };

        input.clear();
        print!("Enter the column (1, 2 or 3): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let col: usize = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {println!("Invalid move....."); continue;}
        };

        if row < 1 || col < 1 || row > 3 || col > 3 || board[row-1][col-1] != Cell::Empty {
            println!("Invalid move, please try again!");
        } else {
            return (row - 1, col - 1);
        }
    }
}

fn main() {
    let mut board: Board = [[Cell::Empty; 3]; 3];
    let mut current_player = Player::Human;

    loop {
        display_board(&board);

        match current_player {
            Player::Human => {
                let (i, j) = get_human_move(&board);
                board[i][j] = Cell::X;
                current_player = Player::AI;
            }
            Player::AI => {
                let (i, j) = best_move(&mut board);
                board[i][j] = Cell::O;
                current_player = Player::Human;
            }
        }

        if let Some(winner) = evaluate(&board) {
            display_board(&board);

            match winner {
                Cell::X => println!("Congratulations! You have defeated the AI!"),
                Cell::O => println!("Oops... AI wins this time...."),
                Cell::Empty => println!("This game ends in a draw!"),
            }
            break;
        }
    }
}