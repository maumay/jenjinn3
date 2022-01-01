use myopic_brain::{Board, EvalBoard, anyhow};

pub fn get(n: usize) -> anyhow::Result<Vec<EvalBoard<Board>>> {
    let mut roots = vec![];
    for &fen in POSITIONS.iter().take(n) {
        roots.push(fen.parse::<EvalBoard<Board>>()?);
    }
    Ok(roots)
}

pub static POSITIONS: [&'static str; 500] = [
    "r1bq1k1r/pp3pbp/3p1np1/1BnPp1B1/1P2P3/2N2P2/P2Q2PP/R3K1NR b KQ b3 0 11",
    "r1bq1rk1/ppp2nbp/3pp1pn/3P1p2/2P1P1P1/2N1BP1P/PP1Q4/2KR1BNR b - g3 0 11",
    "r1bq1r1k/ppp1n1bp/3p1np1/3Ppp2/2P1P3/1QN1BP2/PP2B1PP/1K1R2NR b - - 6 11",
    "r1bqk2r/1p1n1nbp/p1pp1pp1/4p2P/2PPP3/2N1BP2/PP1QN1P1/2KR1B1R b kq - 0 11",
    "r1bqk2r/ppp1npb1/3pn1p1/7p/2PNP2P/2N1B3/PP1QBPP1/R3K2R w KQkq - 2 11",
    "r1bqnrk1/pp1n1pbp/3p2p1/2pP4/2P5/2NB4/PP1BNPPP/R2Q1RK1 w - - 3 11",
    "r1bqk2r/pppn2b1/3p3p/3PpnpP/2P5/2N5/PP1NBPP1/R1BQK2R b KQkq - 1 11",
    "rnb1qk1r/1pp3bp/3p1n2/p2Ppp2/2P5/2N2NP1/PPQ2P1P/R1B1KB1R b KQ - 1 11",
    "r1bq1rk1/ppp1n1bp/3p1np1/3Pp3/2P1Pp2/2NB1P1P/PP2NBP1/R2QK2R b KQ - 1 11",
    "r3k2r/pp1bppbp/1q1p1np1/2pP4/2P1P3/2N1BP2/PP1QB1PP/R4RK1 b kq - 2 11",
    "r1b1k2r/ppppnppp/1b4q1/1B1nP3/3P4/P1N2N2/1P3PPP/R1BQ1RK1 w kq - 3 11",
    "r1bq1rk1/1pp2pb1/3p1npp/p1nPp3/2P1P3/2N1B2P/PP1NBPP1/R2QK2R b KQ - 1 11",
    "r1bqr1k1/pp1n1pb1/2pp1npp/4p3/2PPP2B/2N2N2/PP2BPPP/R2QR1K1 w - - 2 11",
    "r1bq1rk1/1pp3bp/n2p2p1/p2Ppp1n/2P1P3/2NBBP2/PP1QN1PP/2KR3R b - - 1 11",
    "r1b1k2r/1pqnnpbp/p2p2p1/2pPp3/1PP1P3/P1N1BN2/4BPPP/R2Q1RK1 b kq b3 0 11",
    "r1bq1rk1/pp1n1nbp/2pp1pp1/4p3/2PPP3/2N1BN1P/PPQ1BPP1/3R1RK1 b - - 3 11",
    "r1b1k2r/1pp1qpb1/p2p1npp/2nPp3/2P1P3/2N1BN1P/PP2BPP1/R2QK2R w KQkq - 2 11",
    "r1bq1rk1/pp1n1pbp/3p2p1/3Pp1B1/4P1n1/2N2N2/PP2BPPP/R2Q1RK1 b - - 2 11",
    "r1bqk2r/p3nppp/1bpp4/4p3/B3P3/2P5/PP2QPPP/RNB2RK1 b kq - 1 11",
    "r1bqk2r/ppp1n1b1/3p1npp/3Pp3/2P1Pp2/2NBBN2/PP3PPP/R2Q1RK1 w kq - 0 11",
    "rnb2rk1/3nppbp/P2p2p1/q1pP4/4P3/2N1BP2/PP2B1PP/R2QK1NR w KQ - 1 11",
    "r1bqk2r/ppp1n3/3p2pn/3Ppp1p/2P1P2P/2N2P2/PP1Q2P1/R3KBNR w KQkq - 0 11",
    "r1b1k2r/pp2ppbp/2np2p1/q7/2P1P3/1PNQ3P/P3NPP1/R1B2RK1 b kq - 0 11",
    "r1bqk2r/ppp2pb1/3p1n1p/6p1/2P1P3/2N2QB1/PP3PPP/R3KB1R b KQkq - 1 11",
    "rnbq1rk1/ppp3b1/3p2pp/3Ppn2/2P1B3/2N1BN2/PP3PPP/R2QK2R w KQ - 1 11",
    "r1bq1rk1/pppnnpb1/3p2p1/3Pp1Pp/2P1P2P/2N5/PP3P1N/R1BQKB1R w KQ - 1 11",
    "r1bq1rk1/3n1pbp/p2p1np1/1ppPp3/2P1P3/2N1BN1P/PP2BPP1/R2Q1RK1 w - - 0 11",
    "r2q1rk1/ppp1n1bp/2npp1p1/8/2P1P2P/2N1B3/PP1QBPP1/R3K2R b KQ - 1 11",
    "r1bq1rk1/3n1pbp/pp1ppnp1/2p5/2PPP3/2NBBP2/PP2N1PP/2RQ1RK1 w - - 0 11",
    "r1b2rk1/ppppnppp/1q6/4p3/1bB1PP2/8/PBPPQ1PP/RN3R1K b - - 2 11",
    "r1bq1rn1/pp1nppbk/3p2pp/2pP4/2P1PP1N/2NB4/PP4PP/R1BQ1RK1 b - - 2 11",
    "rn1q1rk1/ppp3bp/3p2p1/3Ppb2/2P1B3/4BP2/PP4PP/R2QK1NR w KQ - 1 11",
    "r1bq1rk1/p3ppbp/1p1p2p1/2pPn2n/2P1P2P/1PN1BP2/P2QN1P1/R3KB1R b KQ - 0 11",
    "r2qk2r/1b1n1pbp/p1pp1np1/1p2P3/2P1P3/P1N1BN1P/1P1Q1PP1/2R1KB1R b Kkq - 0 11",
    "r3k1nr/p1p1q1pp/2p5/2bp1bN1/4p3/3P3P/PPP1QPP1/RNB2RK1 w kq - 0 11",
    "r1bq1rk1/pp3ppp/1bpp1n2/4p3/3PP3/2P1B2P/PP1NBPP1/R2Q1RK1 b - - 0 11",
    "r3k1nr/pp3pbp/1qnpb1p1/1Np5/2P1P3/4B3/PP1QNPPP/3RKB1R b Kkq - 3 11",
    "r1b2rk1/pp2ppbp/1q1p1np1/2n5/2PNP3/2NBBP2/PP1Q2PP/R3K2R w KQ - 3 11",
    "r1bq1rk1/pp2ppbp/1n1p2p1/2pPn3/2P1PB2/1PN5/P3BPPP/R2QNRK1 b - - 0 11",
    "r1bn1rk1/ppp1qppp/1b1p4/1B1Pp3/4P1n1/2P2N2/PP1NQPPP/R1BR2K1 w - - 3 11",
    "r1b1r1k1/1p1nppbp/p2p1np1/q1pP4/P1P1P3/2N4P/1P1BBPP1/R2QK1NR w KQ - 1 11",
    "r1bqr1k1/ppp2ppp/2np1n2/bB4B1/3PPQ2/2N2N2/PP3PPP/R4RK1 b - - 5 11",
    "r1bq1r2/pp2ppbk/3p1npp/2pPn3/2P1P2P/2N1BP2/PP1QB1P1/R3K1NR b KQ - 0 11",
    "r1bq1rk1/p1pnn1bp/1p1p2p1/3Pp3/2P1Pp2/2N2P2/PP1QNBPP/R3KB1R w KQ - 0 11",
    "r1b2rk1/1pqn1pbp/p1pp1np1/3Pp3/2P1P3/2N2N2/PPQBBPPP/R1R3K1 b - - 3 11",
    "r2q1rk1/1p2ppbp/p1npbnp1/6B1/2P1P3/2N5/PPNQBPPP/R3K2R w KQ - 3 11",
    "r1bq1rk1/ppp1npbp/3p2p1/3P4/2P1PNn1/2N5/PP2B1PP/R1BQ1RK1 b - - 2 11",
    "r1b2rk1/2ppnpp1/p1nb1q1p/1p2p3/3PP3/1BP1BN2/PP1NQPPP/R4RK1 b - - 1 11",
    "r3k2r/ppp2ppp/1bnqb3/1B1nN3/3P4/8/PP3PPP/RNBQR1K1 w kq - 4 11",
    "r1bqk2r/1pp1npb1/p2p2p1/4n2p/2PNP2P/2N1B3/PP1QBPP1/2KR3R b kq - 1 11",
    "r1bqk2r/ppp3b1/3p1npp/2PPpn2/8/2N2N2/PP2BPPP/R1BQ1RK1 w kq - 1 11",
    "rnbqr1k1/pp3pbp/3p2p1/2pP4/2P2Pn1/2N2N2/PP2B1PP/R1BQ1RK1 w - - 1 11",
    "r1bq1rk1/pp3ppp/2p2n2/3p4/B2PP1n1/2P5/PP2Q1PP/RNB3KR w - d6 0 11",
    "r1bq1rk1/1pp3bp/2np2pn/4pp2/1pPPP3/P1N1BP2/4N1PP/1R1QKB1R w K - 0 11",
    "rnbq1rk1/4n1bp/p2pp1p1/1ppP2B1/2P1P3/2NB4/PP2NPP1/R2QK2R b KQ - 1 11",
    "r2qk2r/ppp3bp/3p1np1/3Ppb2/2P5/2N1B3/PP1QBPPP/R3K2R w KQkq - 2 11",
    "r1bqk2r/1p1p1ppp/2p3n1/p3b2Q/P3P3/1B6/1PP2PPP/RNB2RK1 b kq a3 0 11",
    "r2q1rk1/2pbppbp/2np1np1/1p1P4/1P2P3/P1N1BP2/4N1PP/R2QKB1R b KQ - 0 11",
    "rnb1r1k1/pp2qpbp/3p1np1/2pP2B1/2P5/2N2N2/PP1QBPPP/R3K2R w KQ - 3 11",
    "r2q1rk1/1ppb1pbp/3p1np1/p1nPp3/2P1P3/2N1B3/PPQ1BPPP/R3NRK1 b - - 3 11",
    "r1bqr1k1/pppn1pbp/2np2p1/8/2PNP3/2N1B3/PP1QBPPP/3R1RK1 b - - 6 11",
    "r1bq1rk1/pppnn1bp/3p2p1/3Ppp2/2P1P3/2NN4/PP1BBPPP/R2Q1RK1 b - - 1 11",
    "r1bqk2r/ppp3bp/3p1nn1/3PppBQ/2P1P3/2N5/PP3P1P/R3KBNR w KQkq - 4 11",
    "r1bq1rk1/ppppbppp/2n5/1B1nP3/3P4/5N1P/PP3PP1/RNBQR1K1 w - - 1 11",
    "r2qk2r/1bp1nppp/pbnp4/1p6/3PP3/2N1BN2/PPB2PPP/R2Q1RK1 b kq - 1 11",
    "r1bqk2r/ppp1n1bp/3p1n2/3Pp1p1/2P1Pp2/2NB1P2/PP1Q1BPP/R3K1NR w KQkq - 0 11",
    "rn1k3r/pp2npbp/2p3p1/4p3/2P1P3/2N1Bb2/PP2BPPP/R4RK1 w - - 0 11",
    "r2qk2r/1ppb1pb1/3p1npp/p1nPp1B1/2P1P1P1/2N2P2/PP1QB2P/R3K1NR w KQkq - 0 11",
    "rnb2rk1/1pq1ppbp/p2p1np1/8/2P1PP2/2NBBN2/PP2Q1PP/R4RK1 b - - 3 11",
    "r3k2r/ppp1nppp/1bpqb3/6B1/3PP3/N4N2/PP2QPPP/R4RK1 b kq - 6 11",
    "1rbq1rk1/1p1n1pbp/p2pPnp1/2p5/P1P1PP2/2NB1NP1/1P5P/R1BQK2R b KQ - 0 11",
    "r1bq1rk1/1p1n1pbp/2p2np1/p3p3/2P1P3/2N2N2/PP3PPP/R1BQRBK1 w - - 0 11",
    "r2k3r/pp1n1pbp/2p1bnp1/4p3/2P1P3/2N1BPPN/PP5P/2KR1B1R b - - 1 11",
    "rn3rk1/pp3pb1/3pbnpp/q1p5/2P1P3/2N1B3/PP1QBPPP/R3K1NR w KQ - 2 11",
    "r1bq1rk1/pppnn1bp/3p2pB/3Ppp2/2P1P2P/2N2P2/PP1QN1P1/R3KB1R b KQ - 1 11",
    "r1bq1rk1/1pppbppp/p7/1B1QP3/1n1P4/5N2/PP3PPP/RNB2RK1 w - - 1 11",
    "r1bq1rk1/1p1n1pbp/p2p1np1/3Pp3/4P2P/2N1B3/PP1QBPP1/R3K1NR w KQ - 0 11",
    "r2q1k1r/pppb2bp/n2p1n2/3Ppp2/2P4Q/2NB4/PP1B1PPP/R3K1NR w KQ - 8 11",
    "r2qk2r/p1pb1ppp/1bp2n2/4P3/8/5N2/PP3PPP/RNBQR1K1 b kq - 0 11",
    "r2k2nr/pppn3p/4bppb/4p3/2P1P3/2N1BN2/PP2BPPP/R4RK1 w - - 4 11",
    "r2qr1k1/1ppbppbp/p1np2p1/6Pn/2PPP3/2N1BP2/PP1QN2P/R3KB1R w KQ - 1 11",
    "r1bq1rk1/ppp1n1bp/3p1n2/3PppB1/2P5/2N5/PP1QBPPP/R3K1NR w KQ - 4 11",
    "r1bq1rk1/p4pbp/2pp1np1/4p3/2PnP3/2NB4/PP2NPPP/1RBQ1RK1 w - - 0 11",
    "r1b2rk1/p1pp1ppp/2p2q2/2b5/4P3/1NP5/PP3PPP/RN1Q1RK1 b - - 2 11",
    "r3k1nr/ppq1ppbp/1Npp2p1/8/2PPP1b1/4BN2/PP2BPPP/R2Q1RK1 b kq - 0 11",
    "r1b1k2r/ppp2ppp/2pb1n2/4N3/3P4/2P5/PP3PPP/RNB2RK1 w kq - 1 11",
    "r2qk2r/pp3ppp/1bppbn2/4p1B1/B2PP3/N1P5/PP3PPP/R2Q1RK1 w kq - 4 11",
    "r1bqk2r/p4pbp/2pp1np1/8/2PpP3/2N2P2/PP2B1PP/R2QK1NR w KQkq - 0 11",
    "r1bq1rk1/1p1nn1bp/p1pp2p1/3PpP2/P1P1P3/2NBBP2/1P5P/R2QK1NR b KQ - 0 11",
    "r2qk2r/pppbn1bp/3p1np1/3Pp3/2P5/2NBB1N1/PP3PPP/R2Q1RK1 b kq - 6 11",
    "1r1q1rk1/p1pbnppp/2pp4/2b5/3NP3/2N1B3/PPPQ1PPP/3R1RK1 b - - 7 11",
    "r1b1k2r/pp2n1bp/1q1pp1p1/2p5/2PnPN2/2N1B3/PP1Q1PPP/R3KB1R w KQkq - 2 11",
    "r2qk2r/1ppbn1bp/p2p1np1/3Ppp2/2P1P3/2N1BP1P/PP1Q2P1/2KR1BNR w kq - 1 11",
    "r1bq1rk1/pp1nppbp/3p2p1/2pP4/2P1P1n1/2N2N2/PP1BBPPP/R1Q1K2R w KQ - 5 11",
    "r1bqk2r/1pp2p2/3p1npB/p1nPp2p/2P1P2P/2N3N1/PP2BPP1/R2QK2R b KQkq - 0 11",
    "r1bq1r2/pp1n1pbk/2pp1np1/3Pp2p/2P1P3/2NBBP2/PP1QN1PP/2KR3R b - - 4 11",
    "rnbqk2r/pp2n1b1/3p3p/2pPpppP/2P1P3/2N2N2/PP1B1PP1/R2QKB1R w KQkq - 0 11",
    "r1b1k2r/pppp1ppp/3b4/1B2P3/3Q1Pnq/7P/PPP3P1/RNB2RK1 b kq - 0 11",
    "r2q1rk1/ppp1n1bp/3p2p1/4nb2/2PN1P2/2N1B3/PP2B1PP/R2QK2R b KQ f3 0 11",
    "r1bq1rk1/pppnn1bp/3p2p1/3Ppp2/2P1P1P1/2N2P2/PP2B2P/R1BQNRK1 b - - 0 11",
    "r1bqk2r/pp3ppp/1bpp2n1/4p3/B2PP3/2P3Q1/PP3PPP/RNB2RK1 w kq - 4 11",
    "rnb2rk1/4ppbp/3p1np1/qNpP2B1/4P3/3B1P2/PP1Q2PP/R3K1NR b KQ - 0 11",
    "r1bqrnk1/ppp2pbp/3p1np1/8/2PNP3/2N1BP2/PP2B1PP/R2Q1RK1 w - - 1 11",
    "r1b2rk1/1pq2pbp/n1pp1np1/p2Pp3/P1P1P3/2N5/1P2BPPP/R1BQNRK1 w - - 1 11",
    "r1bq1rk1/2n1ppbp/pp1p1np1/2pP2B1/P1P1P3/2NB1P2/1P2N1PP/R2Q1RK1 b - - 1 11",
    "r1b1k2r/ppp1q1pp/2n2n2/2Ppp3/B7/2P5/PP2QPPP/RNB2RK1 b kq - 3 11",
    "r1bqk2r/1ppnnpb1/3p2p1/p2Pp1Pp/2P1P2P/2N1BP2/PP6/R2QKBNR w KQkq a6 0 11",
    "r1b2rk1/1p1nqpbp/2pp1np1/p2Pp3/2P1P3/2N2N2/PPQ1BPPP/R1BR2K1 w - a6 0 11",
    "r2q1rk1/2p2pp1/p1npbn1p/1pb1p3/4P3/1B1PNN1P/PPP2PP1/R1BQ1RK1 b - - 3 11",
    "r1bqnrk1/ppp1npb1/3p2pp/3Pp3/2P1P3/2N2N2/PP1BBPPP/2RQ1RK1 w - - 0 11",
    "r2qk2r/1bpnnpb1/pp1pp1pp/8/2PPPP2/2N2N2/PP2B1PP/R1BQ1RK1 w kq - 0 11",
    "r1b1k2r/pppp1ppp/1b3n2/4p3/BP2P3/2PP2q1/P4PPP/RNB2RK1 w kq - 0 11",
    "rn2k1nr/pppb1ppp/1b1p2q1/1B1P4/4P3/5N1P/PP1N1PP1/R1BQ1RK1 w kq - 1 11",
    "r1bq1rk1/1ppnn1bp/3p4/p2Ppp2/2P2P2/P1NB4/1P2N1PP/1RBQK2R b K f3 0 11",
    "r3k2r/pp1bppbp/1q1p1np1/2pP4/2P1P3/2N1B3/PP1QBPPP/2KR3R w kq - 1 11",
    "r1bq1rk1/pppn2bp/3ppnp1/8/2P1PP2/2N1B2P/PP2B1P1/R2QK1NR w KQ - 0 11",
    "rnb2rk1/ppppnpp1/5q1p/1B1Pp3/4P3/2P1PN2/PP4PP/RN1Q1RK1 w - - 1 11",
    "r1bqnrk1/ppp1n1bp/3p2p1/2PPpp2/4P3/2N1BP2/PP1NB1PP/R2QK2R b KQ - 0 11",
    "r2q1rk1/pp3pb1/n2pbnpp/2p5/2P1P3/2N1B3/PPQ1BPPP/R3K1NR w KQ - 0 11",
    "r1bq1rk1/3nppbp/p1pp1np1/6B1/2NPP3/2N4P/PP2BPP1/R2Q1RK1 b - - 0 11",
    "r1b1k2r/3nppbp/p1pp1np1/q7/PpPPP3/4BP1N/1P1Q2PP/R2NKB1R w KQkq - 4 11",
    "rnbq1rk1/pp3pb1/3p2pp/2pPp3/2P1Pn2/2NN3P/PP2BPP1/R1BQ1RK1 b - - 5 11",
    "r1bq1rk1/pppnn1bp/3p2p1/3Ppp2/2P1P1P1/2N1BP2/PP1QN2P/R3KB1R w KQ f6 0 11",
    "r2qk2r/pppb4/2np1p2/1Bb1pp1p/4P2N/2NP3P/PPP2PP1/R2Q1RK1 w kq - 0 11",
    "r1b1qrk1/pp1n1pbp/n1pp2p1/3Pp1B1/2P1P3/2NB4/PP1QNPPP/R3K2R w KQ - 2 11",
    "rnbq1rk1/5pbp/p2p1np1/1p1Pp3/4P1P1/2N1BP2/PP1QN2P/R3KB1R b KQ - 0 11",
    "r1b1k2r/ppq1ppbp/3p1np1/3P4/N1PBP3/3B4/PP3PPP/R2QK2R b KQkq - 2 11",
    "r1bqk2r/pppn2b1/3p1ppn/4p2p/2PPP2B/2N2P1P/PP1QN1P1/R3KB1R b KQkq - 0 11",
    "r1bqr1k1/1ppn1pbp/3p2p1/p3p2n/2PPP3/2N2NPP/PP3PB1/R1BQR1K1 w - - 3 11",
    "r1bq1k1r/ppp1n1bp/3p1n2/3Ppp1Q/2P1P3/2N1B3/PP3P1P/2KR1BNR w - - 4 11",
    "rnbq1rk1/5pbp/p2p1np1/1p1Pp3/4P1P1/2N1BN1P/PP3P2/R2QKB1R w KQ b6 0 11",
    "r1bq1rk1/ppp1npb1/2np2pp/8/2PNPP2/2N1B3/PP4PP/R2QKB1R w KQ - 3 11",
    "r1bqk2r/1pP2ppp/p1n2n2/8/1b1P4/3B1N2/PP3PPP/RNBQ1K1R b kq - 0 11",
    "r1b2rk1/1pqn1pbp/p1pp1np1/4p3/2PPP3/2NBBP2/PP1QN1PP/2R2RK1 b - - 3 11",
    "r1bq1rk1/1pp3pp/1bnp1n2/pB3P2/P2P4/5N2/1P3PPP/RNBQ1RK1 w - a6 0 11",
    "r3k2r/pppqn1pp/3b1p2/2p1p3/4P1b1/3PBNN1/PPP2PPP/R2QR1K1 b kq - 3 11",
    "r3k2r/1ppbnpbp/4n1p1/1N2p3/2P1P3/4BN2/PP3PPP/R3KB1R b KQkq - 4 11",
    "r1bq1rk1/1p1n1pbp/p2p1np1/2pPp3/2P1P3/2NBB2P/PP2NPP1/R2Q1RK1 w - - 0 11",
    "1rbq1rk1/2p1ppbp/2np1np1/1p6/1P1PP3/P1N1BP2/3QN1PP/R3KB1R b KQ b3 0 11",
    "rn2k1nr/pp4bp/2p1bpp1/4p3/2P1P3/2N1B1P1/PP3P1P/2KR1BNR w - - 1 11",
    "1rbqk2r/2pnnpbp/1p1pp1p1/p7/P1PPP3/2N1BNP1/1P3PBP/R2Q1RK1 w k - 0 11",
    "r1bq1rk1/p1p1npb1/2pp2p1/7p/2P1P2P/2N1B3/PP1Q1PP1/R3KB1R w KQ h6 0 11",
    "r1b2rk1/pppp1pp1/1b5p/4P3/4q3/2P2N2/PP1N1PPP/R2Q1RK1 b - - 1 11",
    "rnb2rk1/1pp1qppp/1b1p1n2/pB1Pp1B1/P3P3/2P2N2/1PQ2PPP/RN2R1K1 b - - 1 11",
    "r1bq1rk1/1ppn1pbp/1n1p2p1/p2Pp3/2P1P1P1/1PN1BP2/P2QN2P/R3KB1R b KQ g3 0 11",
    "r3k2r/p1pb1ppp/2pp1n2/2b4q/2B1P3/5N2/PPP2PPP/R1BQ1RK1 w kq - 6 11",
    "r4rk1/p1pb1ppp/2pp1q1n/2b3Q1/4P3/2NB4/PPP2PPP/R1B2RK1 b - - 7 11",
    "r1bqk2r/b1ppnppp/p5n1/1p4B1/3PP3/1BN2N2/PP3PPP/R2Q1RK1 b kq - 4 11",
    "r1bq1rk1/1p3pbp/2pp1np1/p1nPp1B1/2P1P3/2N2P2/PPB1N1PP/R2QK2R w KQ - 0 11",
    "r2qk1nr/2p3pp/pbpp1p2/8/3PP1b1/4BN2/PP3PPP/RN1Q1RK1 w kq - 2 11",
    "r1bq1rk1/ppp2pp1/1b1p1Bnp/1B2p3/3PP3/2P2N1P/PP2QPP1/RN3RK1 b - - 0 11",
    "r2qk1nr/1b1n1pb1/pp1pp1p1/2p4p/2PPP2P/1NN1BP2/PP1Q2P1/R3KB1R b KQkq - 1 11",
    "r1b2rk1/pp1nqpbp/3p1np1/3pp1P1/2P1P3/1PNB1N1P/P4P2/R1BQK2R b KQ - 0 11",
    "r2qk2r/1pp1nppp/pbbp4/8/P1NNP3/2P5/1P3PPP/R1BQ1RK1 b kq - 2 11",
    "r1bq1rk1/pp1n1pb1/3p1npp/2pPp3/2P1P3/2N1B3/PP2BPPP/R2QNR1K b - - 1 11",
    "r1bq1rk1/pp3pbp/2n1pnp1/3P4/4P1P1/2N1BP2/PP1Q3P/R3KBNR b KQ - 0 11",
    "r1bqk2r/ppb1nppp/2p5/3p4/4P3/3B4/PPPN1PPP/R1BQ1RK1 w kq - 2 11",
    "r1bqk3/ppp2p2/3p1npr/2nPp2p/2P1P2P/2N3N1/PP1Q1PP1/R3KB1R b KQq - 1 11",
    "r2q1rk1/2p1nppp/p1np4/1p2p3/4P1b1/1BPPPN2/PP1NQ1PP/R3K2R b KQ - 1 11",
    "r1bqk2r/1p3pb1/2pp1npp/p1nPp3/2P1P3/2N1B2P/PPB2PP1/R2QK1NR w KQkq - 0 11",
    "r2qk1nr/ppp3bp/3p4/3Ppp2/2P3b1/2N1BN2/PP2QPPP/R3K2R w KQkq - 0 11",
    "r2qk2r/ppp1n1b1/3pPnpp/4pp2/2P1P3/2N2B2/PP3PPP/R1BQK2R b KQkq - 2 11",
    "r1b2rk1/ppqnppb1/2pp1np1/6Bp/2PPP3/2N3N1/PP1QBPPP/R3K2R b KQ - 7 11",
    "rnb1k2r/pp5p/3p1np1/q1pPp3/2P2P2/1QP5/P1B3PP/R1B1K1NR b KQkq - 2 11",
    "r2qk1nr/1p2npb1/p2p2p1/2pPp2p/2P1P1bP/P1NB2N1/1P3PP1/R1BQK2R w KQkq - 1 11",
    "r1bqnrk1/pp2ppbp/2n3p1/3pP3/3P4/2NBBP2/PP1Q2PP/R3K1NR w KQ - 2 11",
    "r1bqk2r/2p1npbp/np1p2p1/p2Pp3/2P1P1PP/2N1B3/PP2BP2/R1NQK2R b KQkq h3 0 11",
    "r3k1nr/ppp2ppp/2n3q1/1B1pP3/3P2b1/2P2N2/P4PPP/R1BQ1RK1 w kq - 1 11",
    "r1bqk2r/pp3pp1/1bpp1n1p/4P3/B3P3/2P2P2/PP4PP/RNBQ1R1K b kq - 0 11",
    "r1bq1rk1/1p3pbp/n1pp1np1/p3p1B1/P1PPP1P1/2N2P2/1P1QN2P/R3KB1R w KQ e6 0 11",
    "r1bqk2r/5ppp/2pp1n2/pp2p3/P2bP3/1B1P4/1PPN1PPP/R1BQ1RK1 w kq a6 0 11",
    "r1bq1rk1/ppp1n1bp/3pn1p1/5p2/2PNP3/2N1B3/PP2BPPP/R2QK2R w KQ f6 0 11",
    "r1b2rk1/pp1nqpbp/2p2np1/4p3/2P1P3/2N2N1P/PPQ1BPP1/R1B2RK1 w - - 1 11",
    "r2k2nr/pp1n2bp/2p1bpp1/4p3/2P1PP2/2N1BN2/PP2B1PP/R4RK1 b - - 1 11",
    "r1bqk2r/1pp1n1pp/p2p1p2/8/2BQP3/8/PPP2PPP/RN3RK1 b kq - 0 11",
    "r2qk2r/2p2ppp/p1pp1Bb1/2bNp3/4P1P1/3P1N1P/PPP2P2/R2QK2R b KQkq - 0 11",
    "rnb2rk1/2q2pbp/p1pp1np1/1p2p1B1/2PPP3/P1N2N2/1P1QBPPP/3RK2R w K e6 0 11",
    "r1bqk2r/1pppnppp/p7/3nP3/2BP4/1Q3N2/PP1N1PPP/R3K2R b KQkq - 3 11",
    "r1b2rk1/1p1nppbp/p2p2p1/q1p3Pn/2PPP3/2N1BP2/PP1QN2P/R3KB1R w KQ - 1 11",
    "r1bqk1nr/3nppbp/p2p2p1/8/1pPpP3/3BB3/PP1QNPPP/R2N1RK1 w kq - 0 11",
    "r1bk3r/ppp2pbp/6p1/4n2n/2P1PB2/2N5/PP2B1PP/2KR2NR b - - 3 11",
    "r1bq1rk1/pp3pbp/3p1np1/2nPp3/4P3/2N1BP2/PPB1N1PP/R2QK2R b KQ - 4 11",
    "r1bq1rk1/1pp2pbp/p2p1np1/8/2PBP3/2N2P2/PP1QB1PP/R3K2R b KQ - 0 11",
    "r1b2rk1/ppp2pp1/1b3q1p/2p1N3/3PP3/2P5/PP3PPP/RN1Q1RK1 w - - 0 11",
    "r2q1rk1/pppbnppp/2np4/1B6/3PP3/5N1P/PP1N1PP1/R2QR1K1 b - - 0 11",
    "r1bq1rk1/1p3pbp/n1pp1np1/p2Pp3/P1P1P3/2NBBP2/1P1Q2PP/3RK1NR b K - 0 11",
    "r1bqk2r/ppp3bp/3p1np1/3Ppn2/2P1N3/3B1P2/PP2NBPP/R2QK2R b KQkq - 2 11",
    "r1bqr1k1/ppp2pbp/2np2p1/8/2P1PBn1/2N5/PP1QNPPP/R3KB1R w KQ - 9 11",
    "r1b1k2r/ppppqpp1/1b5p/1B1Pp3/3P4/2P5/PP3PPP/R1BQ1RK1 b kq - 0 11",
    "r1bq1rk1/ppppnppp/1b6/3P4/2B1P3/8/PB1P1PPP/RN1Q1RK1 b - - 2 11",
    "r1bq1rk1/p1p2ppp/1bp5/3pPn2/3P4/5N1P/PP3PP1/RNBQ1RK1 w - - 0 11",
    "r2qk2r/ppp1nppp/1bB1b3/4N3/3P4/8/PP3PPP/RNBQR1K1 b kq - 0 11",
    "2bq1rk1/1ppn2bp/3p1ppn/r3p3/2PPP3/2N2N2/P3BPPP/R1BQR1K1 w - - 0 11",
    "r1bq1rk1/pp3pbp/2pp1np1/4n3/2P1P3/1PN1BP2/P2Q2PP/3RKBNR b K - 1 11",
    "1rbq1rk1/1pn1ppbp/p2p1np1/2pP2B1/P1P1P3/2NB1N1P/1P3PP1/R2QK2R w KQ - 1 11",
    "1rb1k1nr/2p1qppp/1bnp4/1p2p3/3PP3/1BP2N1P/1P3PP1/RNBQ1RK1 b k - 0 11",
    "r1bqr1k1/pp1n1pbp/2pp1np1/8/1PPNP3/2N5/P3BPPP/1RBQ1RK1 w - - 1 11",
    "r1bq2nr/ppp1nkb1/3p2pp/1BPPP3/4p3/2N1B2N/PP4PP/R2QK2R b KQ - 0 11",
    "r1b1qrk1/ppp3bp/n2p2p1/3Ppp1n/2P1P3/2N2NPP/PP3PBK/R1BQ1R2 b - - 1 11",
    "r1bqnrk1/ppp2pbp/3p2p1/1P1Pp3/4P3/4BP2/PP1Q2PP/2R1KBNR b K - 4 11",
    "r1bq1rk1/1p1n1pbp/p2p1np1/3pp3/2P1P3/2N4P/PPB1NPP1/R1BQ1RK1 w - - 0 11",
    "rnb2rk1/1p3pbp/p2p1np1/q2Pp3/P3P3/2N1BP2/1P1Q2PP/R3KBNR w KQ - 1 11",
    "r1bq1rk1/1pp1n1bp/p2p1np1/3Ppp2/2P1P3/2N1BP2/PP1QN1PP/1K1R1B1R b - - 1 11",
    "r3k1nr/1p1bppbp/p2p2p1/q1pP4/2P1P3/2N1B3/PP1QBPPP/R4RK1 w kq - 2 11",
    "rn3rk1/pp2pp1p/2pp1npQ/q7/2PPP3/2NB1b2/PP3PPP/R3K2R w KQ - 0 11",
    "r1bq1r2/p1pnnpbk/1p1pp1pp/8/2PPP3/2N1BN2/PP1QBPPP/3R1RK1 w - - 0 11",
    "r1bq1rk1/pp1p1pp1/1bp2n1p/4p3/BP2P3/2PP1Q1P/P2N1PP1/R1B2RK1 b - - 1 11",
    "r2qk2r/1ppbnpp1/pbnp3p/6B1/2BPP3/2N2N2/PP3PPP/R2Q1RK1 w kq - 0 11",
    "r1br2k1/ppp2pbp/5np1/2n1p1B1/2P1P3/2N2N1P/PP2BPP1/R4RK1 b - - 2 11",
    "rnb2rk1/1p3pbp/p2p1np1/q1pP2B1/P3P3/2N2PN1/1P4PP/R2QKB1R b KQ - 0 11",
    "r1bq1rk1/pp3ppp/1bpp1n2/4P3/P1B1P3/1QP5/1P3PPP/RNB2RK1 b - - 0 11",
    "r1bq1rk1/pp1nnpb1/2pQ2pp/4p3/2P1P3/2N1BN2/PP2BPPP/R4RK1 w - - 2 11",
    "r1bqk2r/4ppb1/pnpp1np1/1p5p/2PPP3/1PN1BPN1/P2QB1PP/R3K2R b KQkq - 0 11",
    "r1bk3r/pppn3p/5ppn/4p3/2P1P3/2N5/PP2BPPP/2KR2NR w - - 0 11",
    "r2k2nr/ppp2pbp/4b1p1/4p3/2P1PP2/2NB1P2/PP5P/R1B1K2R b KQ - 0 11",
    "r3k2r/pp2nppp/2pp4/4n3/4P1bq/4Q3/PPPP2PP/RNB1RBK1 w kq - 4 11",
    "r3k1nr/pp2p1bp/1q1p2p1/2pP4/2PnP1b1/2N1B3/PP1QN1PP/R3KB1R w KQkq - 1 11",
    "r1bq1rk1/2p1npp1/p2p1n1p/1pb1p3/4P3/1BPP1N1P/PP2QPP1/RNB2RK1 w - - 2 11",
    "rn1q1rk1/1p1b1pbp/p2p1np1/3Pp3/4P3/2NBB2P/PP2NPP1/R2Q1RK1 b - - 1 11",
    "r2qk1nr/pppb2bp/3p2p1/3Pp3/2P3P1/2N1B3/PP2BP1P/R2QK2R w KQkq - 1 11",
    "1rbq1rk1/1pp2pbp/p2p1np1/3Pp3/2PnP3/2N1BP2/PP1QN1PP/R3KB1R b KQ - 2 11",
    "r1bqk1nr/pppn1p1p/3p2pQ/3Pp3/2P1P3/2N2N2/PP3PPP/R3KB1R w KQkq - 1 11",
    "r1bq1rk1/1p1n1pbp/2pp1np1/p7/2PpP3/1PN2N2/P4PPP/R1BQRBK1 w - - 0 11",
    "r1b2rk1/pp1nppbp/2n3p1/q1p1P3/2P2P2/2N2N2/PP1BB1PP/R2QK2R b KQ - 2 11",
    "rnbq1rk1/ppp1bppp/3p1n2/1B1P4/8/2P2N2/PP3PPP/RNBQR1K1 w - - 2 11",
    "r1bq1rk1/p3ppbp/1p1p1np1/2n5/2PNP3/2N1B2P/PPQ1BPP1/R4RK1 b - - 1 11",
    "r1bqr1k1/pp1n1pbp/2pp1np1/8/2PNP3/2N2P2/PP4PP/R1BQRBK1 b - - 0 11",
    "rnk4r/pppb2bp/5ppn/4p3/2P1P3/2N1B2P/PP2BPP1/2KR2NR b - - 0 11",
    "r2q1rk1/pppb1ppp/1bnp4/1B6/2N1P3/2P1B3/PP3PPP/R2Q1RK1 b - - 1 11",
    "r1bq1rk1/1p1nppbp/2np2p1/pN6/2P1P3/2N1B3/PP2BPPP/R2Q1RK1 w - - 2 11",
    "r1bq1rk1/ppp2pbp/2Np2p1/2n5/2P1P3/2N1B3/PP1QBPPP/R4RK1 b - - 0 11",
    "rn1q1rk1/4p1bp/p1pp1ppn/1p2P3/2PP1PbN/2N1B3/PP1Q2PP/R3KB1R w KQ - 0 11",
    "r1bq1rk1/pppnn1bp/3p2p1/3Ppp2/2P1P3/2N1B3/PP2BPPP/R2QNRK1 w - f6 0 11",
    "r2q1knr/p1p2ppp/2pp4/8/3bP3/5Q2/PP3PPP/RNBR2K1 w - - 0 11",
    "r1bqk2r/1ppnnpb1/p2p2p1/3Pp1Pp/2P1P3/2N1B3/PP1QBP1P/R3K1NR w KQkq - 0 11",
    "r1b2rk1/3nppbp/p1pp1np1/qp6/2PPP3/1PN1BPN1/P2Q2PP/R3KB1R w KQ - 1 11",
    "r2q1rk1/1ppb1pp1/p1np1n1p/4p3/B3P3/2NPQN2/PPP2PPP/R4RK1 w - - 2 11",
    "rnb1qk1r/1pp3bp/3p1n2/p2Ppp2/2P5/2N4N/PPQ2PPP/R1B1KB1R w KQ a6 0 11",
    "r1bqr1k1/ppp2pbp/3p1np1/8/2PBP3/2NQ4/PP2BPPP/R4RK1 b - - 6 11",
    "r1bq1rk1/1p1n1pbp/p2p1np1/3Pp3/P3P3/2N1BP2/1P2N1PP/R2QKB1R w KQ - 1 11",
    "r1bq1rk1/pp2npbp/3p1np1/2pPp3/2P1P1P1/2N2N2/PP2BP1P/R1BQ1R1K w - - 1 11",
    "rnb1k2r/4pp1p/pqpp1npQ/8/2BPP3/2N2P2/PP4PP/2KR2NR b kq - 2 11",
    "r2q1rk1/p2n1pbp/bp1p1np1/2pPp3/2P1P1P1/2N1BP2/PP1QN2P/R3KB1R w KQ - 1 11",
    "r3qrk1/1ppb1pbp/n2p1np1/p2Pp1B1/2P1P3/2N5/PP1NBPPP/1R1QK2R w K - 6 11",
    "r1bq1rk1/1pp2pbp/3p2p1/p1nPp2n/2P1P3/2NBBP2/PP1Q2PP/2KR2NR b - - 3 11",
    "r1bqk2r/1pp2pb1/3p2p1/p1nPp2p/2P1P1nP/2N1B2N/PPQ1BPP1/R3K2R w KQkq - 2 11",
    "r2q1rk1/1ppnppbp/p1np2p1/8/2PPP3/2N1B3/PP2NPPP/1R1Q1RK1 b - - 3 11",
    "r1b1k2r/1pppn1pp/p1n2q2/2b3B1/3p4/2PB1N2/PP3PPP/RN1QR1K1 b kq - 1 11",
    "r1bqk2r/pppp1pp1/7p/1Bb1p3/3NPB2/3P4/PPP1Q1PK/RN2R3 b kq - 0 11",
    "r1bq1rk1/ppn2pbp/3p1np1/2pP4/P3P3/2N5/1P1NBPPP/R1BQ1RK1 b - a3 0 11",
    "r1b1k2r/ppp1nppp/2q5/8/3P4/2P2N2/P4PPP/R1BQ1RK1 b kq - 0 11",
    "r1bq1rk1/1ppnppbp/n2p2p1/p2P4/2PNP3/P1N1BP2/1P4PP/1R1QKB1R b K - 2 11",
    "r2q3r/2pknBpp/p1np4/2b1p3/Pp2P1b1/2P2N1P/1P1P1PP1/RNBQ1RK1 b - - 0 11",
    "rnbq1rk1/3nppbp/p1p3p1/1p2N3/2PP1B2/2N4P/PP2BPP1/R2Q1RK1 b - - 0 11",
    "r1bq1rk1/ppp1n1bp/2Np2p1/8/2P1Pp2/2N1B3/PP1QBPPP/2KR3R b - - 0 11",
    "r1bq1rk1/pp2nppp/1bpp4/4p3/B2PPP2/2P5/PP4PP/RNBQR2K b - - 2 11",
    "r2k3r/pppnnpbp/4b1p1/8/2P1PB2/2N2N2/PP2B1PP/R3K2R b KQ - 0 11",
    "r2qk2r/pp1bn1bp/2pp1np1/3Ppp2/2P1P3/2NBBP1P/PP1QN1P1/R3K2R b KQkq - 1 11",
    "r1bqk2r/pp3pp1/1bpp1n1p/4p3/B1NPP3/2P5/PP3PPP/R1BQR1K1 b kq - 3 11",
    "rnbq1r2/pp3pbk/3p1npp/3Pp3/4P3/2NBB2P/PP1Q1PP1/R3K1NR b KQ - 0 11",
    "r2qk1nr/b1p2ppp/p1pp4/P7/1P1PP1b1/4BN2/5PPP/RN1QK2R b KQkq - 2 11",
    "r1bq1rk1/ppp3bp/2np2p1/4pn2/2PPN3/4BP2/PP1QN1PP/2KR1B1R w - - 1 11",
    "r1bq1rk1/1ppn1pbp/5np1/p3p3/1PP1P3/P1N1BN2/4BPPP/R2Q1RK1 b - - 0 11",
    "r1bqk2r/ppp1npbp/3p2p1/8/2PBP3/2N4P/PP1QBPP1/R3K2R b KQkq - 0 11",
    "r1bqk2r/pp3pp1/1bpp1n1p/4p1B1/B2PP3/N1P5/PP3PPP/R2Q1RK1 w kq - 0 11",
    "r1bq1rk1/ppp1n1bp/3p1np1/3Pp3/2P1N3/3BBP2/PPQ1N1PP/R3K2R b KQ - 0 11",
    "r1bq1rk1/1ppn1pb1/3p1n1p/p2Pp1pP/2P1P3/2N3N1/PP2BPP1/R1BQK2R w KQ - 0 11",
    "r1bq1rk1/1pp1n1bp/3p1np1/p2Ppp2/1PP1P3/P1NBBP2/4N1PP/R2QK2R b KQ - 2 11",
    "r1bqk2r/5pp1/p1pp4/4p2p/4P1n1/N1P2N1P/PP3PP1/R1BQ1RK1 b kq - 1 11",
    "r1b2rk1/pp2ppbp/3p1np1/q1pP4/2P1PP2/2N5/PP1BB1PP/R2Q1RK1 b - f3 0 11",
    "rnbq1rk1/pp4bp/3p2p1/3Ppp1n/4P3/2N1BP2/PP1Q2PP/1K1R1BNR b - - 1 11",
    "2rq1rk1/pp1bppbp/2np1np1/8/2P1P3/1NN1B3/PP2BPPP/R2Q1RK1 w - - 5 11",
    "rnb1k2r/ppp2n1p/3p2p1/3PppQ1/2P1P3/2N2P2/PP4PP/2KR1BNR b kq - 0 11",
    "r2q1rk1/ppp1nppp/1b1pbn2/1B6/3NP3/2P4P/PP1N1PP1/R1BQR1K1 b - - 0 11",
    "2r1k1nr/ppp2p1p/3pbqp1/1N2B3/2P1P3/8/PP3PPP/R2QKB1R b KQk - 0 11",
    "r1bq1rk1/1ppnn1bp/p2p4/3Ppp2/2P3B1/2N2N2/PP3PPP/R1BQ1RK1 w - - 0 11",
    "r1bq1rk1/ppp1np1p/3p2p1/8/2PQP3/2N5/PP2BPPP/R3K2R w KQ - 1 11",
    "r1b1qrk1/pppnn1bp/3p2p1/3Pp1B1/2P1N3/5N2/PP1QBPPP/R3K2R b KQ - 0 11",
    "r1bqr1k1/ppn2pbp/3ppnp1/2pP2B1/2P1P3/2NB1N1P/PP3PP1/R2QR1K1 b - - 7 11",
    "r2q1rk1/p1p2pbp/2Qp1np1/1N6/2P1PPb1/4p3/PP4PP/R3KBNR w KQ - 0 11",
    "r1bq1rk1/1pp2pbp/p2p1np1/3Pp3/2P1P3/4BP2/PP2n1PP/R1NQKB1R w KQ - 0 11",
    "r2q1rk1/pppbn1bp/2np2p1/5p2/2PNP3/2N1B3/PP1QBPPP/4RRK1 b - - 3 11",
    "r2qk1nr/1ppbn1bp/p2p2p1/3Pp3/2P1Pp2/2N2P2/PP1Q1BPP/1K1R1BNR b kq - 1 11",
    "rn1q1rk1/4ppbp/b2p1np1/1BpP4/4P3/2N2N2/PP3PPP/R1BQ1RK1 w - - 1 11",
    "r1b2rk1/2qnppbp/p1pp1np1/1p4B1/2PPP1PP/2N2P2/PP1Q4/2KR1BNR w - b6 0 11",
    "r1bq1rk1/p1p2ppp/1bp2n2/1B6/3P4/2P2Q2/PP3PPP/RNB2RK1 w - - 0 11",
    "r1b1qrk1/1ppn1pbp/n2p2p1/p2Pp1B1/2P1P3/2N5/PP1NBPPP/R2Q1RK1 w - - 6 11",
    "r1bq1rk1/pp1n1pbp/2pp2pn/4P3/2P1P3/2N2N1P/PP3PP1/R1BQRBK1 b - - 0 11",
    "1rbq1rk1/2pn1pbp/p2p1np1/1p1Pp3/2P1P1P1/2N1BP2/PP1QN2P/R3KB1R w KQ b6 0 11",
    "r1b1k2r/ppp3pp/2pb4/8/3PpPNq/2P5/PP1N2PP/R1BQ1RK1 b kq - 0 11",
    "r1bqk2r/2p1npb1/1p1p2p1/p1nPp2p/2P1P2P/2N1BN2/PP1QBPP1/2KR3R b kq - 3 11",
    "r2qk2r/p1p2ppp/1bpp1n2/4p1Bb/4P3/2PB1N1P/PP3PP1/RN1Q1RK1 b kq - 2 11",
    "r2q1rk1/1pp2ppp/1bnp1n2/pB4B1/P2PP1b1/N4N2/1P3PPP/R2Q1RK1 w - - 1 11",
    "r1b2rk1/pppp1ppp/1bn1q3/1BnQP1B1/8/2P2N2/PP3PPP/RN2R1K1 w - - 6 11",
    "r2q1rk1/ppp1npp1/2n1b2p/1B1pP3/1b1P4/5N1P/PP2NPP1/R1BQ1RK1 b - - 2 11",
    "rn1k2nr/p5bp/1pp1p1p1/4p3/1PP1P3/2N1B3/P4PPP/R3KB1R b KQ b3 0 11",
    "r1bqr1k1/pppn1pb1/5npp/2P1p3/4P3/2N2N2/PPQ1BPPP/R1BR2K1 b - - 0 11",
    "1rbq1rk1/ppp2pbp/2np2p1/6B1/2P1P3/2N5/PP1QBPPP/R3K2R b KQ - 1 11",
    "r2q1rk1/ppp3pp/1bnp4/4pb2/8/2NPBN1P/PPP2PP1/R2Q1RK1 w - - 0 11",
    "rnbq1rk1/ppp1npp1/1b1p3p/3P4/4P3/5N1P/PPB2PP1/RNBQ1RK1 b - - 0 11",
    "r3qrk1/1ppb1pbp/n2p1np1/p2Pp1B1/2P1P1P1/2N4P/PP1NBP2/R2QK2R b KQ g3 0 11",
    "r3k1nr/ppp3pp/2pb4/3bNP2/3PpB1q/2P5/PP4PP/RN1Q1RK1 b kq - 2 11",
    "r1bqk1nr/ppp1n2p/3p2p1/3Pp3/2P1Pp2/2N2P2/PP2BQPP/R3K1NR b KQkq - 1 11",
    "r2q1rk1/pp1b1pbp/n2p1np1/3Pp3/4P3/2N1BP2/PP1Q2PP/R1N1KB1R b KQ - 2 11",
    "r1bq1rk1/p1pn1nbp/1p1p1pp1/4p3/2PPP3/BPN2N2/P2QBPPP/R4RK1 w - - 0 11",
    "r2qr1k1/ppp2pbp/2n1bnp1/2B1p1N1/2P1P3/2N5/PP2BPPP/R2Q1RK1 b - - 7 11",
    "r1b1k2r/1pp1np2/p1np1qpp/4p3/B3P3/P1PPPN2/1P2Q1PP/RN2K2R w KQkq - 0 11",
    "r1bqk2r/ppp3bp/3p1n1n/3P1p1N/2P1p3/2N5/PPB2PPP/R1BQK2R b KQkq - 3 11",
    "r1b2rk1/pp1npp2/3p1npQ/q1pP4/2P1P3/2N5/PP2BPPP/R3K1NR b KQ - 0 11",
    "r1bq1rk1/pp2n1bp/2Pp1np1/4pp2/2P1P3/2NBBP2/PP1QN1PP/R3K2R b KQ - 0 11",
    "r1b2rk1/pp1nqpbp/2p2np1/4p3/2P1P3/2NNB3/PP2BPPP/R2Q1RK1 b - - 3 11",
    "r2qk2r/pppbn1bp/2n3p1/4p3/2P5/2N1B2P/PP1QNPP1/2KR1B1R b kq - 3 11",
    "r1bqk1nr/ppp2nbp/3p2p1/3Ppp2/1PP1P3/2N1BP2/P2QN1PP/R3KB1R w KQkq - 1 11",
    "r1bq3r/ppp1nkbp/3p1np1/1BPPp3/4Pp2/2N1BP1P/PP4P1/R2QK1NR w KQ - 0 11",
    "r1b1k2r/pppnqp1p/3p1np1/3Pp3/2P1P3/2NB1P2/PP1Q2PP/2KR2NR b kq - 0 11",
    "r1bq1rk1/ppppnpp1/1b5p/1Bn1P3/8/2P2N2/PP3PPP/RNBQR1K1 w - - 0 11",
    "r3k1nr/ppp2pp1/1bbp1q1p/8/3PP3/2N1BN1P/PP3PP1/R2Q1RK1 b kq - 1 11",
    "r1bq1rk1/1ppn1pbp/p2p2p1/3Pp3/2P1PnP1/2N1BP2/PP1Q3P/2KR1BNR w - - 0 11",
    "r1b2rk1/ppp2ppp/3p4/3Bp3/3bP2q/2PP1Q2/PP3PPP/R1B1K2R b KQ - 0 11",
    "r2q1rk1/pp1n1pbp/2pp1np1/3Pp3/2P1P3/2N1BB1P/PP3PP1/R2Q1RK1 b - - 0 11",
    "r1b2rk1/p1p1npp1/1bpp1q1p/4p3/P3P3/2NPBN1P/1PP2PP1/R2Q1RK1 w - - 0 11",
    "r1bq1rk1/1pp3bp/n2p1np1/p2Pp3/2P1PpP1/2NB1P1P/PPQ5/R1B1K1NR w KQ - 1 11",
    "r1b1k1nr/p3pp1p/1pn3p1/2p1P3/2P5/2P1B3/P4PPP/2KR1BNR w - - 1 11",
    "r1b2rk1/3nppbp/p2p1np1/qPpP4/4P1P1/2N1BP2/PP1Q3P/R3KBNR w KQ - 1 11",
    "r1bq1rk1/p1p1n1bp/2pp2p1/2P2p2/4P3/2N1B3/PP1Q1PPP/2KR1B1R b - - 0 11",
    "r1bq1rk1/ppn1pnbp/2pp1pp1/2PP4/2B1PB2/2N2N2/PP3PPP/R2Q1RK1 b - - 2 11",
    "r1bqk2r/ppp1n1bp/3p3n/3Pp1pP/2P1PpP1/2N5/PP1B1P2/R2QKBNR w KQkq - 1 11",
    "rn1qr1k1/pp3pbp/3p1np1/2pP1b2/2P5/2N2N1P/PP2BPP1/R1BQ1RK1 w - - 1 11",
    "r1bqk2r/1ppnnpb1/p3p1pp/8/2BPP3/2N1BP2/PP1QN1PP/R4RK1 b kq - 0 11",
    "r1bq1rk1/1pp2nbp/p1np1pp1/4p3/2PPP3/2N1BN1P/PP1Q1PP1/2KR1B1R w - - 0 11",
    "r1bq1rk1/p1p2ppp/1bp5/3pP3/3Pn3/4BN2/PPQ2PPP/RN3RK1 b - - 1 11",
    "r1b2rk1/pp1n1pbp/2p2np1/q2Pp1B1/2P1P3/2N5/PP1QB1PP/R3K1NR w KQ - 1 11",
    "r3k1nr/pppb1p1p/3p1qp1/8/2PQP3/2N5/PP2BPPP/R3K2R w KQkq - 1 11",
    "r1bqk2r/b1pp1pp1/p1n2n1p/1p2p3/1P2P3/2PP1N1P/P1BN1PP1/R1BQ1RK1 b kq - 0 11",
    "r2qk1nr/ppp1n1bp/3p2p1/3Pp3/2P1PpP1/2N2B2/PP1B1P1P/R2QK2R b KQkq - 1 11",
    "rnbq1r2/ppp1n1kp/3p2p1/3Ppp2/2P1P3/2N5/PP1QNPPP/R3KB1R w KQ - 0 11",
    "r1bqk2r/pp3ppp/1bpp4/4n3/B3PB2/2N5/PPP2PPP/R2Q1RK1 w kq - 0 11",
    "1rbq1rk1/1p1nppbp/p2p2p1/2pP4/2P1P1n1/2N2N2/PPQBBPPP/R4RK1 b - - 8 11",
    "r1bq1rk1/1pp2pbp/n2p1np1/p2Pp1B1/2P1P3/2N2N1P/PP3PP1/R2QKB1R w KQ - 2 11",
    "r1bqk2r/ppp2nbp/3p1pp1/2P5/3QP3/2N1B2P/PP3PP1/R3KB1R b KQkq - 0 11",
    "r1b1k2r/ppppqpQp/8/1B2P3/8/8/PPP2bPP/RNB3K1 w kq - 0 11",
    "r2qk2r/1ppb1pp1/pbnp1n1p/1B6/4PB2/1NN5/PPP2PPP/R2Q1RK1 w kq - 0 11",
    "r1bq1rk1/1pp2pp1/p2p1nnp/2b1p3/B3P3/2PP1N1P/PP1N1PP1/R1BQR1K1 w - - 4 11",
    "r1bqnrk1/1ppn2bp/p2p2p1/3Ppp2/2P1P3/2N1BP2/PP1Q2PP/R1N1KB1R w KQ f6 0 11",
    "rnbqk2r/4ppb1/p1p2npp/3p4/3PP3/1BN1BP2/PP1Q2PP/2KR2NR w kq - 0 11",
    "r1bqk2r/ppp3bp/3p1np1/3P1n2/2P2B2/2NB1N2/PP4PP/R2QK2R b KQkq - 3 11",
    "r1b2rk1/ppqn1pbp/3p1np1/2pPp1B1/2P1P3/2N2N2/PP2BPPP/2RQ1RK1 w - - 0 11",
    "rnq2rk1/pp2ppbp/3pbnp1/8/2P1P3/2N1B3/PP1NBPPP/R2QK2R b KQ - 5 11",
    "rq3rk1/pp1b1pbp/n1pp1np1/3Pp3/2P1P3/1PN5/P2NBPPP/R1BQ1RK1 w - - 1 11",
    "r1bq1rk1/1p3pbp/p2p1np1/n1pPp3/2P1P3/2N1BP2/PP1Q2PP/R1N1KB1R w KQ c6 0 11",
    "r1bq1rk1/pp1n1pb1/3p1npp/2pPp3/2P1P3/P1N2N2/1PQ1BPPP/R1B2RK1 w - - 0 11",
    "rn1qk2r/pp2nbb1/3p1pp1/2pPp2p/2P1P2P/2N1B3/PP1QNPP1/R3KB1R w KQkq c6 0 11",
    "rn1q1rk1/1p3pbp/p2p1np1/2pP4/P3PBb1/2N2N2/1P2BPPP/R2QK2R w KQ - 1 11",
    "r1b2rk1/2p1qpp1/p1np1n1p/1pb1p3/4P3/1BPP1N1P/PP2QPP1/RNB2RK1 w - - 0 11",
    "r3k1nr/2pbqppp/pb1p4/np2p3/3PP3/2P2N1P/PPB2PP1/RNBQ1RK1 w kq - 1 11",
    "r1bq1rk1/pp3pbp/3p1np1/2nNp1B1/2P1P3/2N2P2/PP2B1PP/R2QK2R b KQ - 4 11",
    "r2q1rk1/pppb1ppp/2np1n2/1B6/3PP3/2N2N2/PP1Q1PPP/R3R1K1 b - - 2 11",
    "r1bq1rk1/pp1nppbp/2np2p1/8/2P1P3/2N1BN1P/PP3PP1/2RQKB1R w K - 3 11",
    "r3k1nr/pp1n1pbp/2p1b1p1/4p3/2P1PP2/2N1B1P1/PPK4P/3R1BNR b - - 2 11",
    "rnbq1r2/pp3pbk/3p1npp/2pP4/2P5/2N1B2P/PP1QBPP1/R3K1NR b KQ - 0 11",
    "r1b1r1k1/ppqn1pbp/2pp1np1/4P3/2P1P3/2N1BN1P/PP1QBPP1/R4RK1 b - - 0 11",
    "r1bq1rk1/ppp1np1p/3p2p1/8/2PQP1P1/2N5/PP2BP1P/R3K2R b KQ - 0 11",
    "r1bq1rk1/1p1nppbp/n1pp2p1/p7/P1PPP3/2N1BP2/1P1Q2PP/R1N1KB1R w KQ - 4 11",
    "r1bq1rk1/ppp1n1bp/3p2p1/3Pp2n/2P1P3/2N1BP2/PP1Q3P/R3KBNR b KQ - 2 11",
    "r1b1k2r/ppp4p/2pb1np1/3qN3/3PpB2/2P5/PP2QPPP/RN3RK1 b kq - 5 11",
    "r1bqk2r/1ppn1pb1/p2p1np1/3Pp1Bp/2P1P3/2N2P2/PP1Q2PP/R1N1KB1R b KQkq - 3 11",
    "r1bqr1k1/ppp2pbp/3p1np1/8/2PBP3/2NB4/PP3PPP/R2Q1RK1 w - - 5 11",
    "r3k1nr/1p1bppbp/1q1p2p1/p1pP4/2P1P3/2N1B3/PP1QnPPP/3RKB1R w Kkq - 0 11",
    "r1b1k1nr/1pqn1pbp/p1p3p1/4p3/1PP1P3/P1N1BN2/4BPPP/R2QK2R b KQkq - 1 11",
    "rnbqnrk1/5pbp/p2pp1p1/1PpP2P1/4P3/1QN1B3/PP2BP1P/R3K1NR b KQ - 1 11",
    "r1bq1rk1/ppp1n1bp/3p2p1/2n2p2/2PNPP2/2N2B2/PP4PP/R1BQ1RK1 w - - 2 11",
    "r2qk2r/1pp2ppp/pbnp1n2/8/2BPP1b1/2N1BN2/PP1Q1PPP/R4RK1 b kq - 5 11",
    "r1bqr1k1/1ppn1pbp/p2p1np1/8/2PNP3/2N3PP/PP3PB1/R1BQ1RK1 w - - 0 11",
    "r1bq1rk1/pppnnpb1/3p2p1/3Pp1Pp/2P1P2N/2N1B3/PP2BP1P/R2QK2R b KQ - 4 11",
    "rn2k2r/ppp3b1/3p1qpp/3Ppb2/2P5/4BP2/PP1Q2PP/2KR1BNR b kq - 1 11",
    "r2q1rk1/pppn1pbp/2npb1p1/8/2P1PN2/2N1BP2/PP4PP/R2QKB1R w KQ - 7 11",
    "r1bq3r/ppp1nkb1/3P1npp/1B1Ppp2/4P3/2N1BP1N/PP4PP/R2QK2R b KQ - 0 11",
    "r2q1r1k/pppb1p2/2np1n1p/1Bb1p1p1/3PP3/2P2N1P/PP3PP1/R1BQRNK1 b - - 0 11",
    "r1bqk2r/pp4pp/2pp1nn1/6B1/1Q1Pp3/2P5/PP2BPPP/RN2K2R b KQkq - 1 11",
    "rnbq1rk1/ppp3bp/3p2p1/3P4/2P1nB2/2NB4/PPQ2P1P/R3K1NR w KQ - 0 11",
    "r2q1rk1/pp1n1pbp/3p1np1/2pP4/P3PPb1/2NB1N2/1P4PP/R1BQ1RK1 b - a3 0 11",
    "r1bq1r1k/1ppp2pp/pbn2n2/4p3/PPB1P3/2P2N2/5PPP/RNBQ1RK1 w - - 2 11",
    "r1bq1rk1/ppp1n1bp/3p2p1/3Ppp1n/2P1P3/P1NBBP2/1P2N1PP/R2QK2R w KQ f6 0 11",
    "rn1qk1nr/1p3pbp/p1pBb1p1/4p3/2P1P3/2N2N1P/PP3PP1/2RQKB1R b Kkq - 5 11",
    "r1bqnrk1/ppp1n1bp/3p2p1/2PPpp2/4P3/2N1B3/PP1QBPPP/2KR2NR w - f6 0 11",
    "r2qr1k1/ppp2pbp/2np1np1/3Pp3/2P1P3/2N1BB2/PP3PPP/1R1Q1RK1 b - - 0 11",
    "r2q1rk1/2pbppb1/p1np1np1/1p4Pp/2PPP2P/2N1BP2/PP1QN3/R3KB1R b KQ - 0 11",
    "rnb1k2r/ppp2n1p/3p2p1/3Pppq1/2P1P3/2N1QP2/PP4PP/2KR1BNR w kq - 1 11",
    "r2qk2r/pp2npbp/3p2p1/2pPp3/2PnP3/2N1BB2/PP3PPP/R2Q1RK1 w kq c6 0 11",
    "r2qk1nr/2pbn1bp/1p1p2p1/p2Ppp2/P1P1P3/1QN2N2/1P2BPPP/R1B2RK1 b kq - 1 11",
    "r4rk1/pp1bppbp/n2p1np1/q1pP4/2P1PN2/P1N1B3/1P2BPPP/R2Q1RK1 b - - 0 11",
    "r1b1qrk1/pppp2pp/1bn5/1B2p1B1/3PR3/2P2N2/PP3PPP/R2Q2K1 b - - 0 11",
    "r2q1rk1/ppp1n1bp/3p1np1/3Ppb2/2P5/2NBB1N1/PP3PPP/R2QK2R w KQ - 4 11",
    "r3k1nr/pp2ppbp/1q1p2p1/2pP4/2P1P3/2N1B3/PP1QbPPP/2KR3R w kq - 0 11",
    "r1b2r1k/ppppq1pp/1bn2n2/1B2p3/1PN1P3/2P2N2/P4PPP/R1BQR1K1 b - - 4 11",
    "r1bqk1nb/ppp1n3/3p2p1/3Pp3/2P1Pp2/2N5/PP1BNPP1/R2QKB2 b Qq - 1 11",
    "r2qk1nr/pp1bp1bp/3p2p1/2pP1p2/2P1P3/2N1BP2/PP1QB1PP/R4RK1 b kq - 1 11",
    "r2q1rk1/1pp1nppp/pbn5/3p2B1/B2PP1b1/5N2/PP2QPPP/RN1R2K1 b - - 5 11",
    "r1bq1rk1/p1p1n1bp/2pp2p1/5p2/2P1P3/2N1B3/PP1QBPPP/R3K2R w KQ - 2 11",
    "r1bq1rk1/pppn2bp/3p2p1/3Pp2n/2P1Pp2/1QNB1P2/PP1BN1PP/R3K2R w KQ - 4 11",
    "r1b2r1k/ppppq1pp/5n2/1BP1n3/4p3/1QP1B2P/PP1N1PP1/R3K2R b KQ - 0 11",
    "r1bqk2r/1pp2p1p/3p1np1/p1nPp3/2P1P1P1/2N1QP2/PP2B2P/R3K1NR b KQkq g3 0 11",
    "r1bqk2r/pp2npb1/2pp1np1/6Bp/2PNP2P/2N5/PP1QBPP1/2KR3R b kq - 3 11",
    "r2qknnr/ppp2p2/3p2pb/2PPp1Np/4P2P/2N5/PP2QPP1/R1B1K2R b KQkq - 0 11",
    "r2q1rk1/p1p2ppp/2pp1n2/2b3B1/4P3/2P2Q2/PP3PPP/RN3RK1 b - - 1 11",
    "r2q1rk1/ppp1npbp/3pb1p1/7P/2PBP3/2N5/PP3PP1/R2QKB1R w KQ - 1 11",
    "r1bq1rk1/pppn2b1/3p2pp/3Ppn2/2P5/2N3P1/PP2NPBP/R1BQ1RK1 w - - 0 11",
    "r1b2rk1/ppp2ppp/2p5/4P3/4nP2/2P5/PP4PP/RNBq1RK1 w - - 0 11",
    "rnbqk2r/4ppbp/p1p2np1/8/3Pp3/1BN1BP2/PP1Q2PP/R3K1NR w KQkq - 0 11",
    "r2k2nr/p1pn2bp/1p2bpp1/2P1p3/4P3/1NN1B3/PP3PPP/R3KB1R b KQ - 0 11",
    "r1bq1rk1/ppp2nbp/3pp1pn/3P1p2/2P1P3/2N1BP1P/PP1Q2P1/2KR1BNR w - - 2 11",
    "r1bq1rk1/pp1n1pbp/3p2p1/2pP3n/2P2P2/2NB3P/PP2N1P1/R1BQK2R w KQ - 1 11",
    "1rbq1rk1/1p2ppbp/p2p1np1/n1pP2B1/2P1P3/2N2P2/PPNQ2PP/R3KB1R b KQ - 1 11",
    "r1bq1rk1/ppp2pbp/3p2p1/2n5/2PNP1n1/2N5/PP3PPP/R1BQRBK1 w - - 3 11",
    "r1bq1rk1/pp3ppp/1bpp1n2/4p3/2BPP3/2P1BQ2/PP1N1PPP/R4RK1 b - - 5 11",
    "r2q1rk1/ppp1np1p/3pb1p1/4b3/2P1P3/2N1B3/PP1Q1PPP/2KR1B1R w - - 4 11",
    "r1bqr1k1/pp1n1pbp/2pp1np1/8/2PBP1P1/2N2P2/PP1QB2P/2KR2NR b - - 0 11",
    "1rbq1rk1/1pp2pb1/p1np1np1/3Pp2p/2P1P2P/2N1BP2/PP1Q2P1/R1N1KB1R b KQ - 0 11",
    "r1b2rk1/1pppnppp/pbn3q1/6B1/BP1pP3/N1P2N2/P4PPP/R2Q1RK1 w - - 2 11",
    "r1b1r1k1/ppp2pbp/n4np1/4p1B1/2P1P3/2N5/PP2BPPP/2KRN2R b - - 5 11",
    "r1bq1rk1/pp3pbp/2p2np1/4p1B1/2P1P3/2N5/PP2BPPP/R2Q1RK1 w - - 0 11",
    "r1b1k1nr/ppp2p1p/3p2p1/8/2PqP3/2N5/PP3PPP/2KR1B1R w kq - 0 11",
    "r1b2rk1/ppp2pp1/1bnp1qnp/1B2p3/4P3/2PPBN2/PP1QNPPP/R4RK1 w - - 0 11",
    "r1bq1rk1/2p1npbp/1p1p1np1/p2Pp3/1PP1P3/B1N2N2/P3BPPP/R2Q1RK1 w - - 0 11",
    "r1b2rk1/3nppbp/pqpp1np1/8/3PP3/2N1B3/PP2BPPP/R1NQK2R w KQ - 0 11",
    "r1bqnrk1/pppn1pbp/3p2p1/3Pp3/1PP1P1P1/2N1BP2/P3B2P/R2QK1NR w KQ - 1 11",
    "r1bq1rk1/pp4bp/n2p1np1/2pPp3/2P1Pp2/P1N2P2/1P2BBPP/1R1QK1NR b K - 1 11",
    "rnbq1rk1/1ppp1ppp/pb1P2n1/8/2B1P3/2N2N2/PP3PPP/R1BQ1RK1 b - - 1 11",
    "r1q2rk1/pppb1ppp/1bnp4/1B2p1Bn/3PP3/N1P2N1P/PP3PPK/R2QR3 b - - 8 11",
    "r1bq1r1k/ppp3pp/1bnp1pn1/4p3/2BPP3/P1P1BN2/1P3PPP/RN1QR1K1 b - - 0 11",
    "r2q1rk1/ppp1nppp/1b1pb3/4p3/4P3/1BNPBQ2/PPP2PPP/R4RK1 w - - 4 11",
    "r1bq1rk1/pp3pbp/n2p2p1/2pP4/2P2Bn1/2NB4/PP1QNPPP/R4RK1 b - - 4 11",
    "r1bqk2r/b1p1nppp/p2p4/np6/3PP3/5N2/PPBB1PPP/RN1Q1RK1 b kq - 1 11",
    "r3k1nr/1p2ppbp/p2p2p1/q1pP4/2PnPP2/2N1BB2/PP4PP/R2QK2R w KQkq - 1 11",
    "r1b1k1nr/1p3ppp/1bpp4/p3p3/P2PP2q/2P5/1PB2PPP/RNBQ1RK1 w kq a6 0 11",
    "r2q1rk1/1ppbppbp/2np2p1/p2P3n/2P1P3/1NN1BP2/PP1Q2PP/R3KB1R b KQ - 0 11",
    "r2n1rk1/ppp1qppp/1b1pbn2/4p3/BPN1P3/2PP1N2/P4PPP/R1BQ1RK1 w - - 5 11",
    "r1bq1rk1/1pp2pp1/1bnp1n1p/pB6/P2PP2B/5N2/1P3PPP/RN1QR1K1 b - - 0 11",
    "r2q1rk1/1ppb1pbp/3p1np1/p1nPp1B1/2P1P3/2N5/PPQNBPPP/R4RK1 b - - 3 11",
    "r1bqk2r/pppnnpb1/3p2pp/3P4/2P1PB2/2N2N2/PP2B1PP/R2Q1RK1 w kq - 0 11",
    "r1bqkn1r/pppn1pb1/6pp/4p3/2P1P3/2N1BN2/PPQ1BPPP/R4RK1 w kq - 2 11",
    "r1bqnrk1/1pp2pbp/3p2p1/p1nPp3/2P1P3/2N5/PPQ1BPPP/R1B1NRK1 w - - 2 11",
    "r2qk2r/pbp1npb1/1p1ppnpp/3P4/2PNP3/2N1B3/PP1QBPPP/R4RK1 b kq - 1 11",
    "r2qk2r/1p1nppbp/p2p1np1/2pP4/P1P1P3/2N2B1P/1P3PP1/R1BQK2R w KQkq - 1 11",
    "r1b2rk1/1pqn1pbp/2pp1np1/p2Pp1B1/2P1P3/2N5/PPQNBPPP/R4RK1 b - - 1 11",
    "rn1qr1k1/pp3pbp/3p1np1/2pP4/2P2Pb1/2N2N2/PP2B1PP/R1BQ1RK1 w - - 5 11",
    "r1bq1rk1/pppnn1bp/3p2p1/3Ppp2/1PP1P3/2N1BN2/P3BPPP/R2Q1RK1 w - f6 0 11",
    "r1bq1rk1/ppp1n1bp/3p2p1/3PppNn/1PP1P3/2N5/P3BPPP/R1BQR1K1 b - - 1 11",
    "r2q1rk1/pppbnpbp/3p1np1/3Pp3/2P1P3/2N1B2P/PP1NBPP1/R2QK2R w KQ - 3 11",
    "r1b2rk1/1p1nppbp/p2p1np1/q7/2PpP3/P1NBBP2/1P1QN1PP/R3K2R w KQ - 0 11",
    "r1bq1rk1/pp1nnpbp/2p3p1/3pP3/3P1P2/2N1BN2/PP2B1PP/R2Q1RK1 b - - 0 11",
    "rnbq1r2/pp3pbk/3p1npp/2pP4/4P3/2N1BP2/PP1QN1PP/R3KB1R b KQ - 2 11",
    "1rbq1rk1/1pp1npbp/p2p1np1/3Pp3/1PP1P3/P1N1BP2/3QN1PP/R3KB1R b KQ - 2 11",
    "r1b1qrk1/1ppp1ppp/pbn5/2nQP1B1/2B5/2P2N2/PP3PPP/RN2R1K1 b - - 1 11",
    "r1bq1rk1/1pp2pb1/3p1npp/p1nPp3/2P1P2B/2N2N2/PP2BPPP/R2Q1RK1 w - - 2 11",
    "r1bq1rk1/pp1n1pb1/3p1np1/2pP3p/4P3/2N2PN1/PP2B1PP/R1BQK2R w KQ h6 0 11",
    "r1bq1rk1/1ppp1ppp/p1n5/1B2P3/1b1P4/5N2/PP1N1PPP/R2Q1RK1 w - - 0 11",
    "r2qk1nr/1pp1npb1/p2p2p1/3Pp2p/2P1P2P/2N1BB2/PP3PP1/R2Q1RK1 b kq h3 0 11",
    "r2qk1nr/pp2n1bp/3p2p1/1B1Ppp2/Q3P3/4B3/PP3PPP/R3K1NR b KQkq - 0 11",
    "rnbq1rk1/ppp3bp/3p1p1n/4P1p1/2P1P3/2N2NB1/PP2BPPP/R2QK2R b KQ - 0 11",
    "r1bq1rk1/ppp1n1bp/3p1np1/3Pp3/2P1P1P1/2N1B2P/PP2N3/R2QKB1R b KQ - 2 11",
    "r1bq1rk1/pp2npbp/3p2p1/2pPN3/4PB2/2N5/PP2BPPP/R2Q1RK1 b - - 0 11",
    "r1bq1rk1/1p3pbp/2pp1np1/p1nPp1B1/2P1P3/2N3N1/PP1QBPPP/R4RK1 b - - 1 11",
    "r2q1rk1/pp3pbp/2npbnp1/2pN2B1/2P1P2P/5P2/PP1Q2P1/2KR1BNR b - h3 0 11",
    "r2qk2r/1bpnnpb1/pp1pp1pp/8/2PPP3/2N1B3/PPBQNPPP/3R1RK1 b kq - 3 11",
    "r1b2rk1/pp1n1pbp/3p1np1/q2Pp3/4P3/2N1BN1P/PP2BPP1/R2QK2R w KQ - 1 11",
    "r1bq1knr/1pp1n1bp/p2p2p1/2PPp3/4Pp2/2N1BN2/PP2BPPP/R2QK2R w KQ - 0 11",
    "r2qk2r/pp1b1nbp/n2p1pp1/2pPp3/2P1P2P/P1N1B2N/1P2BPP1/R2QK2R w KQkq - 1 11",
    "r2q1rk1/pb2ppbp/1p1p1np1/2pP4/1nP1P3/2NBBP2/PP1QN1PP/R4RK1 b - - 0 11",
    "r1bq1rk1/2pn1pbp/pp3np1/4p3/2P1P3/2N1BN1P/PP2BPP1/R2Q1RK1 w - - 0 11",
    "r1bq1knr/2pn1p2/1p1p2pb/p2Pp2p/1PP1P3/P1N2N2/4BPPP/1RBQ1RK1 b - - 3 11",
    "r3k1nr/2pb1ppp/p1nq4/4p3/Q3P3/2P2N2/PP3PPP/RNB2RK1 w kq - 1 11",
    "r2q1rk1/pbpnnpb1/1p1pp1pp/8/2PPPP2/2N1BN2/PPQ1B1PP/R4RK1 w - - 2 11",
    "r1bqk1nr/p1b2ppp/2pp4/1p2p3/B2PP3/N1P5/PP2QPPP/R1B2RK1 w kq b6 0 11",
    "r1bq1rk1/1p1n1pbp/2pp2pn/4p3/1pPPP3/P1N2N2/4BPPP/1RBQ1RK1 w - - 0 11",
    "r1bq1rk1/1p2ppbp/2pp1np1/p7/PnPPP3/2NBBN1P/1P1Q1PP1/3RK2R b K - 5 11",
    "r2qk2r/1ppb1p2/n2p2pn/Q2Pp2p/2P1P3/2N2N2/PP2BPPP/R3K2R b KQkq - 0 11",
    "r2qk2r/1pp3bp/n2p2p1/p2Ppb1n/2P5/2NBBP2/PP1Q2PP/R3K1NR w KQkq - 0 11",
    "r2qrnk1/pppb1pbp/3p1np1/3Pp3/2P1P3/2N1B3/PP2BPPP/R2QNRK1 w - - 3 11",
    "r1b1k2r/1p1np2p/p2p1ppn/q1pP4/2P1PP2/1QPB1N2/P2B2PP/R3K2R b KQkq - 3 11",
    "rn1q1rk1/4ppbp/b2p1np1/1NpP4/4P3/3B1P2/PP2N1PP/R1BQK2R w KQ - 1 11",
    "r1b1q1k1/ppp2ppp/3p1n2/2b5/3nP3/5N2/PP1N1PPP/R1BQ1RK1 w - - 0 11",
    "r1bq1rk1/pp2npbp/2p3p1/2P1p3/B2nP3/2N1BN1P/PP3PP1/R2QK2R b KQ - 0 11",
    "rn1qr1k1/pppb1ppp/1b1p1n2/3Pp3/4P3/N1P2N1P/PP3PP1/R1BQRBK1 b - - 2 11",
    "r1bq1rk1/p1p1n1bp/2pp2p1/5p2/2P1P3/2N1B3/PP2BPPP/R2Q1RK1 w - - 0 11",
    "r1bqk2r/1p3ppp/2p2n2/p1bpP3/P2p4/1B1P4/1PPN1PPP/R1BQ1RK1 b kq - 0 11",
    "rnbq1rk1/ppp3bp/3p4/4p2n/2PP1p2/2N2P2/PP1QNBPP/2KR1B1R b - - 1 11",
    "r1bq1rk1/1ppn1pbp/p2p2p1/3Pp3/P1PnP1PP/2N1B3/1P2BP2/R2QK1NR w KQ - 1 11",
    "r1bq1rk1/pp1p1ppp/n1pb2n1/3Pp3/B1P1P3/8/PP3PPP/RNBQNRK1 w - - 4 11",
    "rnbq1rk1/4ppbp/pp1n2p1/2P5/2P5/2NBBP2/PP2N1PP/R2QK2R b KQ - 0 11",
    "r1bqk2r/ppp1n2p/3p2pn/3Pp3/2P1Pp2/2NB1P2/PP1QN1PP/R3K2R b KQkq - 1 11",
    "r1bqk2r/1pp1n1b1/3p1pp1/p1nPp1Bp/2P1P2P/2N4N/PPB2PP1/R2QK2R w KQkq - 0 11",
    "r1bqk2r/b1p1nppp/p2p4/np6/3PP3/5N2/PPB2PPP/RNBQ1RK1 w kq - 0 11",
    "r1b1k1nr/pp1p1pp1/2pb3p/4p3/B3P3/2PPBP2/PP1N1P1P/R4RK1 b kq - 1 11",
    "r1bq1rk1/pp2n1bp/3p1np1/2pPpp2/2P1P3/2NBBP1P/PP1Q2P1/R3K1NR w KQ c6 0 11",
    "r1bq1rk1/pp2npbp/1n1p2p1/8/3BP3/1BN2N2/PP3PPP/R2QK2R b KQ - 0 11",
    "2kr2nr/pppq1p1p/3p2p1/2b1n3/4P3/2N1N3/PPP2PPP/R1BQ1RK1 w - - 2 11",
    "r1bq1rk1/pp2n1bp/3p2pn/2pPpp2/2P1P3/2NNBP2/PP2B1PP/R2QK2R b KQ - 1 11",
    "r2nk1nr/pp3pbp/2p1b1p1/4p3/2P1P3/2N1B3/PP1NBPPP/2KR3R b kq - 5 11",
    "r1bq1rk1/pp2n1bp/2pp1np1/3Ppp2/2P1P3/2NBBP1P/PP1QN1P1/R3K2R b KQ - 3 11",
];
