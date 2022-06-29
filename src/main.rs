use colored::Colorize;

fn main() {
    print_board()
}

fn print_board() {
    println!();
    //rows
    for i in 0..=8 {
        //cols
        for j in 0..=8 {
            pick_char(j, i);
        }
        println!();
    }
}

fn pick_char(a: u8, b: u8) {
    if b % 2 == 0 {
        if a % 2 == 0 {
            print!("{}", "♟️".on_color("white"));
        } else {
            print!("{}", "♟️".on_color("black"));
        }
    } else if a % 2 == 0 {
        print!("{}", "♟️".on_color("black"));
    } else {
        print!("{}", "♟️".on_color("white"));
    }
}
