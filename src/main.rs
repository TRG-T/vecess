use colored::Colorize;

fn main() {
    let mut board: [[&str; 8]; 8] = [
        ["   "; 8], ["   "; 8], ["   "; 8], ["   "; 8], ["   "; 8], ["   "; 8], [" "  ; 8], ["   "; 8],
    ];
    generate_pieces(&mut board, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    print_board(board)
}

fn print_board(board: [[&str; 8]; 8]) {
    println!();
    //rows
    for i in 0..8 {
        //cols
        for j in 0..8 {
            pick_char(i, j, board);
        }
        println!();
    }
}

fn pick_char(a: usize, b: usize, board: [[&str; 8]; 8]) {
    if b % 2 == 0 {
        if a % 2 == 0 {
            print!("{}", board[a][b].on_truecolor(0, 153, 255));
        } else {
            print!("{}", board[a][b].on_truecolor(0, 102, 204));
        }
    } else if a % 2 == 0 {
        print!("{}", board[a][b].on_truecolor(0 , 102, 204));
    } else {
        print!("{}", board[a][b].on_truecolor(0, 153, 255));
    }
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
