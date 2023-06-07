use std::collections::BTreeSet;

use enumset::EnumSet;

use crate::{BitBoard, Corner, Piece, Side, Square};

/// Chess is a symmetric game and this trait represents a component of
/// the game which can be reflected to it's symmetric opposite component.
pub trait Reflectable {
    fn reflect(&self) -> Self;
}

impl Reflectable for Side {
    fn reflect(&self) -> Self {
        match self {
            Side::W => Side::B,
            Side::B => Side::W,
        }
    }
}

impl Reflectable for Corner {
    fn reflect(&self) -> Self {
        Corner(self.0.reflect(), self.1)
    }
}

impl Reflectable for BitBoard {
    fn reflect(&self) -> Self {
        self.into_iter().map(|sq| sq.reflect()).collect()
    }
}

impl Reflectable for Square {
    fn reflect(&self) -> Self {
        let (fi, ri) = (self.file_index(), self.rank_index());
        ((8 * (7 - ri) + fi) as usize).into()
    }
}

/// We reflect a piece to it's correspondent on the opposite side.
impl Reflectable for Piece {
    fn reflect(&self) -> Self {
        match self {
            Piece::WP => Piece::BP,
            Piece::WN => Piece::BN,
            Piece::WB => Piece::BB,
            Piece::WR => Piece::BR,
            Piece::WQ => Piece::BQ,
            Piece::WK => Piece::BK,
            Piece::BP => Piece::WP,
            Piece::BN => Piece::WN,
            Piece::BB => Piece::WB,
            Piece::BR => Piece::WR,
            Piece::BQ => Piece::WQ,
            Piece::BK => Piece::WK,
        }
    }
}

impl Reflectable for i32 {
    fn reflect(&self) -> Self {
        -(*self)
    }
}

impl<T: Reflectable> Reflectable for Vec<T> {
    fn reflect(&self) -> Self {
        self.into_iter().map(|t| t.reflect()).collect()
    }
}

impl<T: Reflectable> Reflectable for Option<T> {
    fn reflect(&self) -> Self {
        self.as_ref().map(|t| t.reflect())
    }
}

impl<T1, T2> Reflectable for (T1, T2)
where
    T1: Reflectable,
    T2: Reflectable,
{
    fn reflect(&self) -> Self {
        (self.0.reflect(), self.1.reflect())
    }
}

impl<T1, T2, T3> Reflectable for (T1, T2, T3)
where
    T1: Reflectable,
    T2: Reflectable,
    T3: Reflectable,
{
    fn reflect(&self) -> Self {
        (self.0.reflect(), self.1.reflect(), self.2.reflect())
    }
}

impl<T: Reflectable + Ord> Reflectable for BTreeSet<T> {
    fn reflect(&self) -> Self {
        self.iter().map(|x| x.reflect()).collect()
    }
}

impl<T: Reflectable + enumset::EnumSetType> Reflectable for EnumSet<T> {
    fn reflect(&self) -> Self {
        self.iter().map(|z| z.reflect()).collect()
    }
}
