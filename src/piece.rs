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
    pub fn new(char: &'a str, fg_color: Color, bg_color: Color, piece_type: Type) -> Piece<'a> {
        Piece {
            char: char.with(fg_color).on(bg_color),
            has_moved: false,
            piece_type
        }
    }

    pub fn change_background_color(&mut self, color: Color) {
        self.char.style_mut().background_color = Some(color);
    }

    pub fn change_char(&mut self, char: &'a str) {
        let style = self.char.style();
        self.char = char.with(style.foreground_color.unwrap()).on(style.background_color.unwrap());
    }
}