fn main() {
    print_board()
}

fn print_board() {
    //rows
    for i in 0..=32 {
        //cols
        for j in 0..=32 {
            pick_char(j, i);
        }
        print!("\n");
    }
}

fn pick_char(a: u8, b: u8) {
    if a % 8 == 0 && b % 4 == 0 {
        print!("+");
    } else if a % 4 == 0 {
        print!("|");
    } else if b % 4 == 0 {
        print!("-")
    } else {
        print!(" ");
    }
}
