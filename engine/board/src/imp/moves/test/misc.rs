use super::*;

#[test]
fn case_01() -> Result<()> {
    execute_test(TestCase {
        board: "rn1k3r/2q2ppp/2p3b1/1b2pP2/8/BPN2B2/2Q2PP1/R3K2R w KQkq e6 3 10",
        #[rustfmt::skip]
        expected_all: vec![
            "swra1a2-", "swra1b1-", "swra1c1-", "swra1d1-",
            "swba3b2-", "swba3c1-", "swba3b4-", "swba3c5-", "swba3d6-", "swba3e7-", "swba3f8-",
            "swpb3b4-",
            "swnc3a2-", "swnc3a4-", "swnc3b5bb", "swnc3d5-", "swnc3e4-", "swnc3e2-", "swnc3d1-", "swnc3b1-",
            "swqc2a2-", "swqc2b2-", "swqc2b1-", "swqc2c1-", "swqc2d1-", "swqc2d2-", "swqc2e2-", "swqc2d3-", "swqc2e4-",
            "swke1d1-", "swke1d2-", "cwq",
            "swbf3e2-", "swbf3d1-", "swbf3e4-", "swbf3d5-", "swbf3c6bp", "swbf3g4-", "swbf3h5-",
            "swpf5g6bb", "swpf5f6-", "ewf5e6e5",
            "swpg2g3-", "swpg2g4-",
            "swrh1g1-", "swrh1f1-", "swrh1h2-", "swrh1h3-", "swrh1h4-", "swrh1h5-", "swrh1h6-", "swrh1h7bp"
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
            "swnc3b5bb",
            "swbf3c6bp",
            "swpf5g6bb",
            "ewf5e6e5",
            "swrh1h7bp",
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "swnc3b5bb",
            "swbf3c6bp",
            "swpf5g6bb",
            "ewf5e6e5",
            "swrh1h7bp",
            "swra1d1-",
            "swqc2d1-",
            "swqc2d2-",
            "swqc2d3-",
            "swba3e7-",
        ],
    })
}

#[test]
fn case_02() -> Result<()> {
    execute_test(TestCase {
        board: "1r5r/P1k2ppp/2n3b1/b2p4/B2P4/2N5/p4PP1/1R2K2R w Kk - 5 20",
        #[rustfmt::skip]
        expected_all: vec![
            "swba4b3-", "swba4c2-", "swba4d1-", "swba4b5-", "swba4c6bn",
            "swrb1a1-", "swrb1c1-", "swrb1d1-", "swrb1b2-", "swrb1b3-", "swrb1b4-", "swrb1b5-", "swrb1b6-", "swrb1b7-", "swrb1b8br",
            "swke1d1-", "swke1d2-", "swke1e2-", "swke1f1-", "cwk",
            "pa7a8wn-", "pa7a8wb-", "pa7a8wr-", "pa7a8wq-",
            "pa7b8wnbr", "pa7b8wbbr", "pa7b8wrbr", "pa7b8wqbr",
            "swpf2f3-", "swpf2f4-",
            "swpg2g3-", "swpg2g4-",
            "swrh1g1-", "swrh1f1-", "swrh1h2-", "swrh1h3-", "swrh1h4-", "swrh1h5-", "swrh1h6-", "swrh1h7bp",
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
            "swba4c6bn",
            "swrb1b8br",
            "swrh1h7bp",
            "pa7b8wnbr", "pa7b8wbbr", "pa7b8wrbr", "pa7b8wqbr",
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "swba4c6bn",
            "swrb1b8br", "swrb1b7-",
            "swrh1h7bp",
            "pa7a8wn-", "pa7a8wb-", "pa7a8wr-", "pa7a8wq-",
            "pa7b8wnbr", "pa7b8wbbr", "pa7b8wrbr", "pa7b8wqbr",
        ],
    })
}

#[test]
fn case_03() -> Result<()> {
    execute_test(TestCase {
        board: "r3k2r/2q2pp1/2p3b1/1b6/8/1PN2B2/2Q2PP1/R3RK2 w kq - 3 10",
        #[rustfmt::skip]
        expected_all: vec![
            "swkf1g1-",
            "swre1e2-",
            "swqc2e2-", "swqc2d3-",
            "swnc3e2-", "swnc3b5bb",
            "swbf3e2-"
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
            "swkf1g1-",
            "swre1e2-",
            "swqc2e2-", "swqc2d3-",
            "swnc3e2-", "swnc3b5bb",
            "swbf3e2-"
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "swkf1g1-",
            "swre1e2-",
            "swqc2e2-", "swqc2d3-",
            "swnc3e2-", "swnc3b5bb",
            "swbf3e2-"
        ],
    })
}

#[test]
fn case_05() -> Result<()> {
    execute_test(TestCase {
        board: "r3k2r/2q2pp1/2p3b1/1b6/8/1PN2B2/2Q2PP1/R3RK2 b kq - 3 10",
        #[rustfmt::skip]
        expected_all: vec![
            "sbke8f8-", "sbke8d8-", "sbke8d7-",
            "sbqc7e7-", "sbqc7e5-",
            "sbbg6e4-",
            "sbbb5e2-"
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
            "sbke8f8-", "sbke8d8-", "sbke8d7-",
            "sbqc7e7-", "sbqc7e5-",
            "sbbg6e4-",
            "sbbb5e2-"
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "sbke8f8-", "sbke8d8-", "sbke8d7-",
            "sbqc7e7-", "sbqc7e5-",
            "sbbg6e4-",
            "sbbb5e2-"
        ],
    })
}

#[test]
fn case_06() -> Result<()> {
    execute_test(TestCase {
        board: "8/8/8/8/8/2k1R3/8/B6K b - - 3 10",
        #[rustfmt::skip]
        expected_all: vec![
            "sbkc3c2-", "sbkc3d2-", "sbkc3c4-", "sbkc3b4-",
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
            "sbkc3c2-", "sbkc3d2-", "sbkc3c4-", "sbkc3b4-",
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "sbkc3c2-", "sbkc3d2-", "sbkc3c4-", "sbkc3b4-",
        ],
    })
}

#[test]
fn case_07() -> Result<()> {
    execute_test(TestCase {
        board: "7k/8/8/3PpP2/8/8/8/7K w - e6 3 10",
        #[rustfmt::skip]
        expected_all: vec![
            "swpd5d6-", "swpf5f6-",
            "ewd5e6e5", "ewf5e6e5",
            "swkh1g1-", "swkh1g2-", "swkh1h2-",
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
            "ewd5e6e5", "ewf5e6e5",
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "ewd5e6e5", "ewf5e6e5",
        ],
    })
}

#[test]
fn case_08() -> Result<()> {
    execute_test(TestCase {
        board: "4k2b/1KP2rP1/8/2P5/8/8/8/8 w - - 3 10",
        #[rustfmt::skip]
        expected_all: vec![
            "swkb7b8-", "swkb7a8-", "swkb7a7-", "swkb7a6-", "swkb7b6-", "swkb7c6-", "swkb7c8-",
            "swpc5c6-",
            "pg7g8wn-", "pg7g8wb-", "pg7g8wr-", "pg7g8wq-",
            "pg7h8wnbb", "pg7h8wbbb", "pg7h8wrbb", "pg7h8wqbb",
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
            "pg7h8wnbb", "pg7h8wbbb", "pg7h8wrbb", "pg7h8wqbb",
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "pg7g8wn-", "pg7g8wb-", "pg7g8wr-", "pg7g8wq-",
            "pg7h8wnbb", "pg7h8wbbb", "pg7h8wrbb", "pg7h8wqbb",
        ],
    })
}

#[test]
fn case_10() -> Result<()> {
    execute_test(TestCase {
        board: "6k1/8/1K6/2Pp4/8/4b3/8/8 w - - 3 10",
        #[rustfmt::skip]
        expected_all: vec![
            "swkb6b5-", "swkb6c6-", "swkb6c7-", "swkb6b7-", "swkb6a7-", "swkb6a6-", "swkb6a5-"
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
        ],
    })
}

#[test]
fn case_11() -> Result<()> {
    execute_test(TestCase {
        board: "r1bk2br/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q7 b - - 3 10",
        #[rustfmt::skip]
        expected_all: vec![
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
        ],
    })
}

#[test]
fn case_12() -> Result<()> {
    execute_test(TestCase {
        board: "6rk/5p2/4pPp1/8/8/6P1/6PK/7R w - - 3 10",
        #[rustfmt::skip]
        expected_all: vec![
            "swkh2h3-", "swkh2g1-",
            "swpg3g4-",
            "swrh1g1-", "swrh1f1-", "swrh1e1-", "swrh1d1-", "swrh1c1-", "swrh1b1-", "swrh1a1-",
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "swkh2g1-"
        ],
    })
}

#[test]
fn case_13() -> Result<()> {
    execute_test(TestCase {
        board: "4R3/1p1Q2rk/6p1/2p1BpP1/p1P1pP2/P7/1P6/K2q4 w - - 2 2",
        #[rustfmt::skip]
        expected_all: vec![
            "swqd7d1bq",
            "swka1a2-",
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
            "swqd7d1bq",
            "swka1a2-",
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "swqd7d1bq",
            "swka1a2-",
        ],
    })
}

#[test]
fn case_14() -> Result<()> {
    execute_test(TestCase {
        board: "8/8/3p4/2pP3R/2Pk1pPQ/3B4/2K2P2/8 b - g3 0 1",
        #[rustfmt::skip]
        expected_all: vec![
            "sbpf4f3-"
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
        ],
    })
}

#[test]
fn case_15() -> Result<()> {
    execute_test(TestCase {
        board: "1k6/b2P1r2/p5p1/4qp1p/7P/7K/8/8 w - - 0 1",
        #[rustfmt::skip]
        expected_all: vec![
            "pd7d8wn-", "pd7d8wb-", "pd7d8wr-", "pd7d8wq-",
            "swkh3g2-"
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "pd7d8wn-", "pd7d8wb-", "pd7d8wr-", "pd7d8wq-",
        ],
    })
}

#[test]
fn case_16() -> Result<()> {
    execute_test(TestCase {
        board: "8/bk1P1r2/p5p1/4qp1p/7P/7K/8/8 w - - 0 1",
        #[rustfmt::skip]
        expected_all: vec![
            "pd7d8wn-", "pd7d8wb-", "pd7d8wr-", "pd7d8wq-",
            "swkh3g2-"
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "pd7d8wn-", "pd7d8wb-", "pd7d8wr-", "pd7d8wq-",
        ],
    })
}

#[test]
fn case_17() -> Result<()> {
    execute_test(TestCase {
        board: "1r4rk/5pBp/ppbp1P2/5P2/2P3R1/3n3p/PP5P/5NK1 b - - 1 3",
        #[rustfmt::skip]
        expected_all: vec![
            "sbrg8g7wb"
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
            "sbrg8g7wb"
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "sbrg8g7wb"
        ],
    })
}
