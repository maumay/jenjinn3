use std::fmt::{Display, Formatter};
use std::str::FromStr;

use myopic_core::{
    anyhow::{Error, Result},
    Corner, Line, Reflectable, Side,
};

use crate::{Piece, Square};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Move {
    Standard { moving: Piece, from: Square, dest: Square, capture: Option<Piece> },
    Enpassant { side: Side, from: Square, dest: Square, capture: Square },
    Promotion { from: Square, dest: Square, promoted: Piece, capture: Option<Piece> },
    Castle { corner: Corner },
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Standard { moving, from, dest, capture, .. } => write!(
                f,
                "s{}{}{}{}",
                moving,
                from,
                dest,
                match capture {
                    None => "-".to_string(),
                    Some(p) => p.to_string(),
                }
            ),
            Move::Promotion { from, dest, promoted, capture, .. } => write!(
                f,
                "p{}{}{}{}",
                from,
                dest,
                promoted,
                match capture {
                    None => "-".to_string(),
                    Some(p) => p.to_string(),
                }
            ),
            Move::Enpassant { side, from, dest, capture, .. } => {
                write!(f, "e{}{}{}{}", side, from, dest, capture)
            }
            Move::Castle { corner, .. } => write!(f, "c{}{:?}", corner.0, corner.1),
        }
    }
}

impl Reflectable for Move {
    fn reflect(&self) -> Self {
        match self {
            &Move::Standard { moving, dest, from, capture, .. } => Move::Standard {
                moving: moving.reflect(),
                dest: dest.reflect(),
                from: from.reflect(),
                capture: capture.reflect(),
            },
            &Move::Promotion { from, dest, promoted, capture, .. } => Move::Promotion {
                from: from.reflect(),
                dest: dest.reflect(),
                promoted: promoted.reflect(),
                capture: capture.reflect(),
            },
            &Move::Enpassant { side, from, dest, capture, .. } => Move::Enpassant {
                side: side.reflect(),
                from: from.reflect(),
                dest: dest.reflect(),
                capture: capture.reflect(),
            },
            &Move::Castle { corner, .. } => Move::Castle { corner: corner.reflect() },
        }
    }
}

#[cfg(test)]
impl Move {
    pub fn from(s: &str) -> Result<Move> {
        use myopic_core::anyhow::anyhow;
        match s.chars().next() {
            None => Err(anyhow!("Cannot parse move from empty string!")),
            Some(t) => match t {
                's' => Ok(Move::Standard {
                    moving: slice(s, 1, 2).parse()?,
                    from: slice(s, 3, 2).parse()?,
                    dest: slice(s, 5, 2).parse()?,
                    capture: parse_op(slice(s, 7, 2).as_str())?,
                }),
                'e' => Ok(Move::Enpassant {
                    side: slice(s, 1, 1).parse()?,
                    from: slice(s, 2, 2).parse()?,
                    dest: slice(s, 4, 2).parse()?,
                    capture: slice(s, 6, 2).parse()?,
                }),
                'p' => Ok(Move::Promotion {
                    from: slice(s, 1, 2).parse()?,
                    dest: slice(s, 3, 2).parse()?,
                    promoted: slice(s, 5, 2).parse()?,
                    capture: parse_op(slice(s, 7, 2).as_str())?,
                }),
                'c' => Ok(Move::Castle {
                    corner: Corner(slice(s, 1, 1).parse()?, slice(s, 2, 1).parse()?),
                }),
                _ => Err(anyhow!("Cannot parse {} as a move", s)),
            },
        }
    }
}

#[cfg(test)]
fn slice(s: &str, skip: usize, take: usize) -> String {
    s.chars().skip(skip).take(take).collect()
}

pub fn parse_op<F>(s: &str) -> Result<Option<F>>
where
    F: FromStr<Err = Error>,
{
    match s {
        "-" => Ok(None),
        _ => Ok(Some(FromStr::from_str(s)?)),
    }
}

#[cfg(test)]
mod test {
    use myopic_core::{Class, Flank};

    use crate::moves::Move;
    use crate::{Piece, Square};

    use super::*;

    #[test]
    fn standard() -> Result<()> {
        assert_eq!(
            Move::Standard {
                moving: Piece(Side::W, Class::P),
                from: Square::E2,
                dest: Square::E4,
                capture: None,
            },
            Move::from("swpe2e4-")?
        );
        assert_eq!(
            Move::Standard {
                moving: Piece(Side::B, Class::R),
                from: Square::C4,
                dest: Square::C2,
                capture: Some(Piece(Side::W, Class::P)),
            },
            Move::from("sbrc4c2wp")?
        );
        Ok(())
    }

    #[test]
    fn promotion() -> Result<()> {
        assert_eq!(
            Move::Promotion {
                from: Square::E7,
                dest: Square::E8,
                promoted: Piece(Side::W, Class::Q),
                capture: None,
            },
            Move::from("pe7e8wq-")?
        );
        assert_eq!(
            Move::Promotion {
                from: Square::E7,
                dest: Square::D8,
                promoted: Piece(Side::W, Class::Q),
                capture: Some(Piece(Side::B, Class::B)),
            },
            Move::from("pe7d8wqbb")?
        );
        Ok(())
    }

    #[test]
    fn enpassant() -> Result<()> {
        assert_eq!(
            Move::Enpassant {
                side: Side::B,
                from: Square::D4,
                dest: Square::C3,
                capture: Square::C4,
            },
            Move::from("ebd4c3c4")?
        );
        Ok(())
    }

    #[test]
    fn castle() -> Result<()> {
        assert_eq!(Move::Castle { corner: Corner(Side::B, Flank::K) }, Move::from("cbk")?);
        Ok(())
    }
}

impl Move {
    pub fn moving_side(&self) -> Side {
        match self {
            &Move::Standard { moving: Piece(side, _), .. } => side,
            &Move::Enpassant { side, .. } => side,
            &Move::Promotion { promoted: Piece(side, _), .. } => side,
            &Move::Castle { corner: Corner(side, _), .. } => side,
        }
    }

    /// Convert this move into a human readable uci long format string.
    pub fn uci_format(&self) -> String {
        match self {
            Move::Standard { from, dest, .. } => format!("{}{}", from, dest),
            Move::Enpassant { from, dest, .. } => format!("{}{}", from, dest),
            Move::Castle { corner, .. } => {
                let Line(src, dest) = Line::king_castling(*corner);
                format!("{}{}", src, dest)
            }
            Move::Promotion { from, dest, promoted: Piece(_, class), .. } => {
                format!("{}{}{}", from, dest, class)
            }
        }
    }
}
