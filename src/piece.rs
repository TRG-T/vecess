use crate::board::Board;
use crate::pos::Pos;
use crossterm::style::Color;

#[derive(Clone, Copy, PartialEq)]
pub enum Type {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Blank,
}

impl Type {
    pub fn as_str(&self) -> &str {
        match self {
            Type::Pawn => " ♟ ",
            Type::Knight => " ♞ ",
            Type::Bishop => " ♝ ",
            Type::Rook => " ♜ ",
            Type::Queen => " ♚ ",
            Type::King => " ♛ ",
            Type::Blank => "   ",
        }
    }
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub color: Color,
    pub piece_type: Type,
    pub has_moved: bool,
}

impl Piece {
    pub fn new(color: Color, piece_type: Type) -> Piece {
        Piece {
            color,
            has_moved: false,
            piece_type,
        }
    }

    pub fn set_type(&mut self, piece_type: Type) {
        self.piece_type = piece_type;
    }

    pub fn get_piece_moves(&self, pos: &Pos, board: &mut Board) -> [[bool; 8]; 8] {
        let mut possible_moves = [[false; 8]; 8];
        match self.piece_type {
            Type::Pawn => {
                if board.fields[pos.y - 1][pos.x].piece_type == Type::Blank {
                    possible_moves[pos.y - 1][pos.x] = true;
                    if !self.has_moved && board.fields[pos.y - 2][pos.x].piece_type == Type::Blank {
                        possible_moves[pos.y - 2][pos.x] = true;
                    }
                }
                if pos.x < 7 && board.fields[pos.y - 1][pos.x + 1].piece_type != Type::Blank {
                    possible_moves[pos.y - 1][pos.x + 1] = true;
                }
                if pos.x <= 7
                    && pos.x > 0
                    && board.fields[pos.y - 1][pos.x - 1].piece_type != Type::Blank
                {
                    possible_moves[pos.y - 1][pos.x - 1] = true;
                }
            }
            Type::Rook => {
                // up
                for a in (0..pos.y).rev() {
                    if board.fields[a][pos.x].piece_type == Type::Blank {
                        possible_moves[a][pos.x] = true;
                    } else {
                        break;
                    }
                }
                // down
                for a in pos.y + 1..8 {
                    if board.fields[a][pos.x].piece_type == Type::Blank {
                        possible_moves[a][pos.x] = true;
                    } else {
                        break;
                    }
                }
                // left
                for a in (0..pos.x).rev() {
                    if board.fields[pos.y][a].piece_type == Type::Blank {
                        possible_moves[pos.y][a] = true;
                    } else {
                        break;
                    }
                }
                // right
                for a in pos.x + 1..8 {
                    if board.fields[pos.y][a].piece_type == Type::Blank {
                        possible_moves[pos.y][a] = true;
                    } else {
                        break;
                    }
                }
            }
            Type::Bishop => {
                // up-right
                for a in 1..8 {
                    if pos.x + a <= 7 {
                        if board.fields[pos.y - a][pos.x + a].piece_type == Type::Blank {
                            possible_moves[pos.y - a][pos.x + a] = true;
                        } else {
                            break;
                        }
                    }
                }
                // up-left
                for a in 1..8 {
                    if pos.x.checked_sub(a).is_some() {
                        if board.fields[pos.y - a][pos.x - a].piece_type == Type::Blank {
                            possible_moves[pos.y - a][pos.x - a] = true;
                        } else {
                            break;
                        }
                    }
                }
                // down-right
                for a in 1..8 {
                    if pos.x + a <= 7 && pos.y+a <= 7 {
                        if board.fields[pos.y + a][pos.x + a].piece_type == Type::Blank {
                            possible_moves[pos.y + a][pos.x + a] = true;
                        } else {
                            break;
                        }
                    }
                }
                // down-left
                for a in 1..8 {
                    if pos.x.checked_sub(a).is_some() && pos.y+a <= 7 {
                        if board.fields[pos.y + a][pos.x - a].piece_type == Type::Blank {
                            possible_moves[pos.y + a][pos.x - a] = true;
                        } else {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
        possible_moves
    }
}
