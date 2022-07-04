use colored::Colorize;
use std::{thread, time::{Duration}};


const BOARD_SIZE: usize = 8;
const LETTERS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
// 0,154,255 - darker field
// 0,102,204 - brighter field
const COLORS: [[u8;3];2] = [[0,154, 255], [0,102,204]];

fn fps(fps: u64) -> Duration {
    Duration::from_millis(1000/fps)
}

fn main() {
    let mut board: [[&str; 8]; 8] = [["   "; 8]; 8];
    let mut white_move = true;
    generate_pieces(&mut board, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    println!();

    loop {
        print!("{esc}[2J{esc}[5;1H", esc = 27 as char);
        print_board_letters();
        print_board(board);
        println!();
        match white_move {
            true => println!("       White's move"),
            false => println!("      Black's move")
        }
        thread::sleep(fps(5));
    }
}

fn print_board(board: [[&str; 8]; 8]) {
    for row in 0..BOARD_SIZE {
        print!("{} ", 8-row);
        for col in 0..BOARD_SIZE {
            pick_char(row, col, board);
        }
        println!();
    }
}

fn print_board_letters() {
    print!("  ");
    for letter in LETTERS {
        print!(" {} ", letter);
    }
    println!();
}

fn pick_char(row: usize, col: usize, board: [[&str; 8]; 8]) {
    let color = COLORS[(row+col) % 2];
    print!("{}", board[row][col].on_truecolor(color[0], color[1], color[2]));
}

fn generate_pieces(board: &mut [[&str; 8]; 8], pieces: &str) {
    let mut row: usize = 0;
    let mut col: usize = 0;
    for c in pieces.chars() {
        if c == '/' {
            row += 1;
            col = 0;
            continue;
        }
        match c {
            // black pieces
            'r' => board[row][col] = " ♖ ",
            'n' => board[row][col] = " ♘ ",
            'b' => board[row][col] = " ♗ ",
            'q' => board[row][col] = " ♕ ",
            'k' => board[row][col] = " ♔ ",
            'p' => board[row][col] = " ♙ ",
            // white pieces
            'R' => board[row][col] = " ♜ ",
            'N' => board[row][col] = " ♞ ",
            'B' => board[row][col] = " ♝ ",
            'Q' => board[row][col] = " ♛ ",
            'K' => board[row][col] = " ♚ ",
            'P' => board[row][col] = " ♟ ",
            _ => {}
        }
        col += 1;
    }
}
