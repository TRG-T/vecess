use colored::Colorize;


const BOARD_SIZE: usize = 8;
const LETTERS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
// 0,154,255 - darker field
// 0,102,204 - brighter field
const COLORS: [[u8;3];2] = [[0,154, 255], [0,102,204]];

fn main() {
    let mut board: [[&str; 8]; 8] = [["   "; 8]; 8];
    generate_pieces(&mut board, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    println!();
    print_board_letters();
    print_board(board);
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
    for a in 0..BOARD_SIZE {
        print!(" {} ", LETTERS[a]);
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
            'r' => board[row][col] = " ♜ ",
            'n' => board[row][col] = " ♞ ",
            'b' => board[row][col] = " ♝ ",
            'q' => board[row][col] = " ♛ ",
            'k' => board[row][col] = " ♚ ",
            'p' => board[row][col] = " ♟ ",
            // white pieces
            'R' => board[row][col] = " ♖ ",
            'N' => board[row][col] = " ♘ ",
            'B' => board[row][col] = " ♗ ",
            'Q' => board[row][col] = " ♕ ",
            'K' => board[row][col] = " ♔ ",
            'P' => board[row][col] = " ♙ ",
            _ => {}
        }
        col += 1;
    }
}
