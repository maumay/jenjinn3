use std::array;
use std::cmp::{max, min};
use gcd::Gcd;
use itertools::{iterate, Itertools};
use lazy_static::lazy_static;
use crate::{Board, class, Dir, file, lift, Piece, rank, side, SideMap, Square, SquareMap};
use crate::board::iterator::BoardIterator;
use crate::constants::side;

lazy_static! {
    static ref CONTROL: PieceControl = compute_control();
}

pub fn control(piece: Piece, sq: Square, occupied: Board) -> Board {
    use crate::constants::class;
    match class(piece) {
        class::P => CONTROL.pawns[side(piece)][sq],
        class::N => CONTROL.knights[sq],
        class::B => bishop_control(sq, occupied),
        class::R => rook_control(sq, occupied),
        class::Q => bishop_control(sq, occupied) | rook_control(sq, occupied),
        class::K => CONTROL.king[sq],
        _ => panic!("{} is not a valid piece class", class(piece)),
    }
}

fn bishop_control(sq: Square, occupied: Board) -> Board {
    use magic::*;
    CONTROL.bishop[sq][index(occupied & BISHOP_MASKS[sq], BISHOP_MAGICS[sq], BISHOP_SHIFTS[sq])]
}

fn rook_control(sq: Square, occupied: Board) -> Board {
    use magic::*;
    CONTROL.rook[sq][index(occupied & ROOK_MASKS[sq], ROOK_MAGICS[sq], ROOK_SHIFTS[sq])]
}

struct PieceControl {
    pawns: SideMap<SquareMap<Board>>,
    knights: SquareMap<Board>,
    king: SquareMap<Board>,
    rook: SquareMap<Vec<Board>>,
    bishop: SquareMap<Vec<Board>>,
}

fn compute_control() -> PieceControl {
    use crate::constants::dir::*;
    use magic::*;
    PieceControl {
        knights: array::from_fn(|sq| rays(sq, &[NNE, NEE, SEE, SSE, SSW, SWW, NWW, NNW], 1)),
        king: array::from_fn(|sq| rays(sq, &[N, NE, E, SE, S, SW, W, NW], 1)),
        pawns: [
            array::from_fn(|sq| rays(sq, &[NE, NW], 1)),
            array::from_fn(|sq| rays(sq, &[SE, SW], 1)),
        ],
        rook: array::from_fn(|sq| {
            compute_magic_moves(sq, ROOK_MASKS[sq], ROOK_MAGICS[sq], ROOK_SHIFTS[sq], &[N, E, S, W])
        }),
        bishop: array::from_fn(|sq| {
            compute_magic_moves(sq, BISHOP_MASKS[sq], BISHOP_MAGICS[sq], BISHOP_SHIFTS[sq], &[NE, SE, SW, NW])
        }),
    }
}

fn compute_magic_moves(
    sq: Square,
    mask: Board,
    magic: u64,
    shift: usize,
    dirs: &[Dir],
) -> Vec<Board> {
    let mut result = vec![0u64; 1usize << mask.count_ones()];
    for variation in compute_powerset(iter(mask).collect_vec().as_slice()) {
        let index = magic::index(variation, magic, shift);
        result[index] = compute_sliding_control(sq, variation, dirs)
    }
    result
}

fn compute_sliding_control(source: Square, occupancy: Board, dirs: &[Dir]) -> Board {
    let mut control = 0u64;
    for &d in dirs {
        let mut next_sq = next(source, d);
        while let Some(sq) = next_sq {
            control |= lift(sq);
            next_sq = next(sq, d);
            if contains(occupancy, sq) {
                break
            }
        }
    }
    control
}

fn compute_powerset(squares: &[Square]) -> Vec<Board> {
    if squares.is_empty() {
        vec![0]
    } else {
        let (head, rest) = (squares[0], &squares[1..]);
        compute_powerset(rest).into_iter()
            .flat_map(|r| [r, r | lift(head)].into_iter()).collect()
    }
}

pub fn next(square: Square, (dr, df): Dir) -> Option<Square> {
    let next_r = (rank(square) as isize) + dr;
    let next_f = (file(square) as isize) + df;
    if 0 <= min(next_f, next_r) && max(next_f, next_r) < 8 {
        Some(8 * (next_r as usize) + next_f as usize)
    } else {
        None
    }
}

pub const fn contains(board: Board, square: Square) -> bool {
    board & lift(square) != 0
}

pub fn rays(source: Square, dirs: &[Dir], depth: usize) -> Board {
    dirs.iter()
        .flat_map(|&d| iterate(Some(source), move |op| op.and_then(|sq| next(sq, d))).take_while(|op| op.is_some()).skip(1).take(depth))
        .filter_map(|x| x)
        .fold(0u64, |a, n| a | lift(n))
}

pub fn cord(from: Square, dest: Square) -> Board {
    let dr = rank(dest) as isize - rank(from) as isize;
    let df = file(dest) as isize - file(from) as isize;
    let gcd = (df.abs() as u32).gcd(dr.abs() as u32) as isize;
    lift(from) | rays(from, &[(dr / gcd, df / gcd)], gcd as usize)
}

pub fn iter(board: Board) -> impl Iterator<Item=Square> {
    BoardIterator(board)
}

#[rustfmt::skip]
mod iterator {
    use crate::{Square, SquareMap};

    const DEBRUIJN64: u64 = 0x03f79d71b4cb0a89u64;

    const BITSCAN: SquareMap<usize> = [
         0, 47,  1, 56, 48, 27,  2, 60,
        57, 49, 41, 37, 28, 16,  3, 61,
        54, 58, 35, 52, 50, 42, 21, 44,
        38, 32, 29, 23, 17, 11,  4, 62,
        46, 55, 26, 59, 40, 36, 15, 53,
        34, 51, 20, 43, 31, 22, 10, 45,
        25, 39, 14, 33, 19, 30,  9, 24,
        13, 18,  8, 12,  7,  6,  5, 63,
    ];


    /// The iterator implementation struct produced by a bitboard. It simply
    /// wraps a long value used to track the remaining set bits.
    pub(super) struct BoardIterator(pub u64);

    /// The implementation uses the 'de bruijn' forward bitscan method for
    /// determining the LSB of the encapsulated u64 value. The LSB represents
    /// the next square to be returned.
    impl Iterator for BoardIterator {
        type Item = Square;
        fn next(&mut self) -> Option<Square> {
            if self.0 == 0 {
                None
            } else {
                let lsb = bitscan(self.0);
                self.0 ^= 1u64 << lsb as u64;
                Some(lsb)
            }
        }
    }

    fn bitscan(x: u64) -> usize {
        BITSCAN[(x ^ x - 1).wrapping_mul(DEBRUIJN64).wrapping_shr(58) as usize]
    }

    #[cfg(test)]
    mod bitscan_test {
        use super::bitscan;

        #[test]
        fn test() {
            assert_eq!(0, bitscan(1u64));
            assert_eq!(1, bitscan(2u64));
            assert_eq!(1, bitscan(0b10u64));
            assert_eq!(10, bitscan(0b1001110000000000u64));
            assert_eq!(21, bitscan(0b1001111011000000000000000000000));
        }
    }
}

#[rustfmt::skip]
mod magic {
    use crate::SquareMap;

    pub(super) fn index(occupancy: u64, magic: u64, shift: usize) -> usize {
        occupancy.wrapping_mul(magic).wrapping_shr(shift as u32) as usize
    }

    pub(super) const BISHOP_SHIFTS: SquareMap<usize> = [
        58, 59, 59, 59, 59, 59, 59, 58,
        59, 59, 59, 59, 59, 59, 59, 59,
        59, 59, 57, 57, 57, 57, 59, 59,
        59, 59, 57, 55, 55, 57, 59, 59,
        59, 59, 57, 55, 55, 57, 59, 59,
        59, 59, 57, 57, 57, 57, 59, 59,
        59, 59, 59, 59, 59, 59, 59, 59,
        58, 59, 59, 59, 59, 59, 59, 58,
    ];

    pub const ROOK_SHIFTS: SquareMap<usize> = [
        52, 53, 53, 53, 53, 53, 53, 52,
        53, 54, 54, 54, 54, 54, 54, 53,
        53, 54, 54, 54, 54, 54, 54, 53,
        53, 54, 54, 54, 54, 54, 54, 53,
        53, 54, 54, 54, 54, 54, 54, 53,
        53, 54, 54, 54, 54, 54, 54, 53,
        53, 54, 54, 54, 54, 54, 54, 53,
        52, 53, 53, 53, 53, 53, 53, 52,
    ];

    pub const BISHOP_MASKS: SquareMap<u64> = [
        18049651735527936, 70506452091904, 275415828992, 1075975168,
        38021120, 8657588224, 2216338399232, 567382630219776,
        9024825867763712, 18049651735527424, 70506452221952, 275449643008,
        9733406720, 2216342585344, 567382630203392, 1134765260406784,
        4512412933816832, 9024825867633664, 18049651768822272, 70515108615168,
        2491752130560, 567383701868544, 1134765256220672, 2269530512441344,
        2256206450263040, 4512412900526080, 9024834391117824, 18051867805491712,
        637888545440768, 1135039602493440, 2269529440784384, 4539058881568768,
        1128098963916800, 2256197927833600, 4514594912477184, 9592139778506752,
        19184279556981248, 2339762086609920, 4538784537380864, 9077569074761728,
        562958610993152, 1125917221986304, 2814792987328512, 5629586008178688,
        11259172008099840, 22518341868716544, 9007336962655232, 18014673925310464,
        2216338399232, 4432676798464, 11064376819712, 22137335185408,
        44272556441600, 87995357200384, 35253226045952, 70506452091904,
        567382630219776, 1134765260406784, 2832480465846272, 5667157807464448,
        11333774449049600, 22526811443298304, 9024825867763712, 18049651735527936,
    ];

    pub const ROOK_MASKS: SquareMap<u64> = [
        282578800148862, 565157600297596, 1130315200595066, 2260630401190006,
        4521260802379886, 9042521604759646, 18085043209519166, 36170086419038334,
        282578800180736, 565157600328704, 1130315200625152, 2260630401218048,
        4521260802403840, 9042521604775424, 18085043209518592, 36170086419037696,
        282578808340736, 565157608292864, 1130315208328192, 2260630408398848,
        4521260808540160, 9042521608822784, 18085043209388032, 36170086418907136,
        282580897300736, 565159647117824, 1130317180306432, 2260632246683648,
        4521262379438080, 9042522644946944, 18085043175964672, 36170086385483776,
        283115671060736, 565681586307584, 1130822006735872, 2261102847592448,
        4521664529305600, 9042787892731904, 18085034619584512, 36170077829103616,
        420017753620736, 699298018886144, 1260057572672512, 2381576680245248,
        4624614895390720, 9110691325681664, 18082844186263552, 36167887395782656,
        35466950888980736, 34905104758997504, 34344362452452352, 33222877839362048,
        30979908613181440, 26493970160820224, 17522093256097792, 35607136465616896,
        9079539427579068672, 8935706818303361536, 8792156787827803136, 8505056726876686336,
        7930856604974452736, 6782456361169985536, 4485655873561051136, 9115426935197958144,
    ];

    pub const BISHOP_MAGICS: SquareMap<u64> = [
        7728262862096860416, 220677507089776865, 3386521924206592, 4613942907922024450,
        1270580313697035264, 5075362929222160, 2311477087219744816, 6896689110597697,
        2450292586280190512, 36172034782470656, 576619120637050894, 4508071795621952,
        9817849395557959680, 9229008176677126208, 11547229745261322272, 1152923171205222400,
        13988323674017630208, 1234285382376461056, 6210080462669056, 5629672407703560,
        145241133417506307, 6918091990483665994, 1970325929074704, 6922173374756194304,
        14125545729882657284, 1174350986501685512, 9081967124218370, 2306407196252012896,
        9552139207841947908, 9241670113680183318, 4756279494363385860, 4684871853554139648,
        9587760252870688, 1304226949500930, 1152958892603873280, 577058920992931920,
        563516893429792, 1157426204819718340, 2308519364401562626, 1128649366044928,
        1162073857658462208, 2324421470224146592, 4684311099581958144, 720611408294987776,
        36072850565627968, 2326109791685970064, 4612251176027554816, 92332601609683008,
        2577289649332228, 4612390358990487808, 72066666150756352, 3498065461877670915,
        2252074828046336, 4434830885632, 577626269026615688, 4693885679539683457,
        1153485623899718656, 3692970803156095250, 81351784116257792, 35328866124808,
        72057594323272192, 306262504353374726, 9140495749376, 9185389143196176,
    ];

    pub const ROOK_MAGICS: SquareMap<u64> = [
        2341874280142213664, 594475425959256064, 108095255871488641, 144119629074153488,
        144120144474409072, 4755959553358037504, 432346148410294784, 36029896534859904,
        144678277649285376, 594616025744410508, 581105226859151496, 1315332604825702400,
        1153484540627394704, 146930521992073260, 615304578832794632, 1154891833739067522,
        9225800308394049536, 9226829176558854144, 585469326216153089, 10698248273543712,
        2252350844600392, 5846244066938863668, 2450099484550496768, 288232575192825924,
        666533073417961472, 11556271830356460160, 175924008976512, 1443685572161831944,
        18296204201838592, 18577666299921416, 18159122252034, 1128725995388996,
        9242094795738841400, 576531123237036037, 11294189891358721, 108103985465919488,
        5188709791718966432, 1153484471748534912, 230931868684289, 6971572790272983105,
        431696826957828, 1157495473515298944, 2451119294454497344, 4688820058456129568,
        38843564507660305, 81346285449838598, 162982876361785347, 72063111003242508,
        144255930163282176, 211108415671424, 581567161361797632, 35789965623552,
        1224996708144972032, 50102722221052416, 1441228851271173120, 9386068972664819200,
        72571307467906, 288218347995265, 3458870070174812241, 2306973444874447105,
        145804072854750210, 93731193081563201, 8800401621508, 857903159050370,
    ];
}

#[cfg(test)]
mod test {
    use crate::constants::piece::*;
    use crate::constants::square::*;
    use crate::constants::dir::*;
    use crate::board;
    use crate::test::assert_boards_equal;

    #[test]
    fn control() {
        assert_boards_equal(
            super::control(WB, E3, board!(C5, A7, E4, H6, F2)),
            board!(~E3 => C1, C5, H6, F2)
        );
        assert_boards_equal(
            super::control(BR, D5, board!(D5, D2, D1, E8)),
            board!(~D5 => D2, A5, H5, D8)
        );
        assert_boards_equal(
            super::control(BB, C8, board!(A7, B4, C6, C8, D1, D2, D5, D6, E1, E8, F8)),
            board!(~C8 => A6, H3)
        );
        assert_boards_equal(
            super::control(BK, E8, board!(A7, B4, C6, C8, D1, D2, D5, D6, E1, E8, F8)),
            board!(D8, D7, E7, F7, F8)
        );
        assert_boards_equal(
            super::control(BN, C6, board!(A7, B4, C6, C8, D1, D2, D5, D6, E1, E8, F8)),
            board!(A7, A5, B4, D4, E5, E7, D8, B8)
        );
    }

    #[test]
    fn next() {
        assert_eq!(Some(A2), super::next(A1, N));
        assert_eq!(Some(B5), super::next(D4, NWW));
        assert_eq!(None, super::next(A8, N));
        assert_eq!(None, super::next(C2, SSE));
        assert_eq!(None, super::next(H6, NE));
        assert_eq!(None, super::next(A7, W));
    }
}

