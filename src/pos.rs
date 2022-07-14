#[derive(PartialEq, Clone, Copy)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(y: usize, x: usize) -> Pos {
        Pos { y, x }
    }
}
