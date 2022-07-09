use crossterm::style::{Color, StyledContent, Stylize};

#[derive(Clone, Copy, Default)]
pub enum Type {
    #[default]
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Blank
}

#[derive(Clone, Copy)]
pub struct Piece<'a> {
    pub char: StyledContent<&'a str>,
    pub has_moved: bool,
    pub piece_type: Type
}

impl<'a> Piece<'a> {
    pub fn new(char: &'a str, color: Color, piece_type: Type) -> Piece<'a> {
        Piece {
            char: char.with(color),
            has_moved: false,
            piece_type
        }
    }

    pub fn set_char(&mut self, char: &'a str) {
        self.char = char.with(self.char.style().foreground_color.unwrap());
    }

    pub fn set_type(&mut self, piece_type: Type) {
        self.piece_type = piece_type;
    }
}