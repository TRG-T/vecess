use crossterm::style::{Color, StyledContent, Stylize};

#[derive(Clone, Copy)]
pub struct Piece<'a> {
    pub char: StyledContent<&'a str>,
    pub has_moved: bool,
}

impl<'a> Piece<'a> {
    pub fn new(char: &'a str, fg_color: Color, bg_color: Color) -> Piece<'a> {
        Piece {
            char: char.with(fg_color).on(bg_color),
            has_moved: false,
        }
    }

    pub fn change_background_color(&mut self, color: Color) {
        self.char.style_mut().background_color = Some(color);
    }
}