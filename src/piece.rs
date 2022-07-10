use crossterm::style::{Color, StyledContent, Stylize};

use crate::{Pos, board::Board};

#[derive(Clone, Copy, PartialEq)]
pub enum Type {
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

    pub fn get_piece_moves(&self, pos: &Pos, board: &mut Board) -> [[bool; 8]; 8] {
        let mut possible_moves = [[false;8];8];
        match self.piece_type {
            Type::Pawn => {
                if board.fields[pos.y-1][pos.x].piece_type == Type::Blank {
                    possible_moves[pos.y-1][pos.x] = true;
                    if !self.has_moved && board.fields[pos.y-2][pos.x].piece_type == Type::Blank {
                        possible_moves[pos.y-2][pos.x] = true;
                    }
                }
                if pos.x < 7 && board.fields[pos.y-1][pos.x+1].piece_type != Type::Blank {
                    possible_moves[pos.y-1][pos.x+1] = true;
                }
                if pos.x <= 7 && pos.x > 0 && board.fields[pos.y-1][pos.x-1].piece_type != Type::Blank {
                    possible_moves[pos.y-1][pos.x-1] = true;
                }
            },
            Type::Rook => {},
            _ => {}
        }
        possible_moves
    } 
}