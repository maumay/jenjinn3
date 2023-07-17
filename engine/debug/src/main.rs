use clap::{Parser, Subcommand};
use std::str::FromStr;
use hyperopic::moves::Moves;
use hyperopic::node::SearchNode;
use hyperopic::position::Position;
use hyperopic::search::{SearchParameters, Transpositions, TranspositionsImpl, TreeNode};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    SearchPgn {
        #[arg(long)]
        pgn: String,
        #[arg(long)]
        depth: usize,
        #[arg(long)]
        table_size: usize,
    },
    SearchFen {
        #[arg(long)]
        fen: String,
        #[arg(long)]
        depth: usize,
        #[arg(long)]
        table_size: usize,
    },
    Moves {
        #[arg(long)]
        fen: String,
    },
}

fn main() {
    match Cli::parse().command {
        Commands::SearchPgn { pgn, depth, table_size } => {
            run_search(pgn.parse::<Position>().unwrap().into(), depth, table_size);
        }
        Commands::SearchFen { fen, depth, table_size } => {
            run_search(fen.parse::<Position>().unwrap().into(), depth, table_size);
        }
        Commands::Moves { fen } => {
            let board = fen.as_str().parse::<Position>().unwrap();
            let moves: Vec<_> =
                board.moves(&Moves::All).into_iter().map(|m| m.to_string()).collect();
            println!("{}", serde_json::to_string_pretty(&moves).unwrap());
        }
    }
}

//struct DebugTranspositions {
//    store: Vec<Option<(String, TreeNode)>>,
//}
//
//impl DebugTranspositions {
//    pub fn new(size: usize) -> DebugTranspositions {
//        DebugTranspositions { store: vec![None; size] }
//    }
//}
//
//const FEN_PARTS: [FenPart; 4] =
//    [FenPart::Board, FenPart::Active, FenPart::CastlingRights, FenPart::Enpassant];
//
//impl Transpositions for DebugTranspositions {
//    fn get(&self, pos: &Position) -> Option<&TreeNode> {
//        let hash = pos.key;
//        let index = (hash % self.store.len() as u64) as usize;
//        if let Some((existing, n)) = self.store[index].as_ref() {
//            if n.matches(hash) {
//                let new_pos = pos.to_fen_parts(&FEN_PARTS);
//                if existing.as_str() != new_pos.as_str() {
//                    panic!("Collision: {} <-> {}", existing, new_pos)
//                }
//                Some(n)
//            } else {
//                None
//            }
//        } else {
//            None
//        }
//    }
//
//    fn put(&mut self, pos: &Position, n: TreeNode) {
//        let hash = pos.key;
//        let index = (hash % self.store.len() as u64) as usize;
//        if !pos.moves(&Moves::All).contains(&n.get_move()) {
//            panic!("Bad node {} <-> {:?}", pos.to_fen(), n)
//        }
//        self.store[index] = Some((pos.to_fen_parts(&FEN_PARTS), n))
//    }
//}

fn run_search(mut state: SearchNode, depth: usize, table_size: usize) {
    if depth == 0 {
        println!("Static: {}", state.relative_eval());
        println!("Quiescent: {}", hyperopic::search::quiescent::full_search(&mut state).unwrap());
    } else {
        let outcome = hyperopic::search::search(
            state,
            SearchParameters {
                end: depth,
                table: &mut TranspositionsImpl::new(table_size),
            },
        );
        println!("{}", serde_json::to_string_pretty(&outcome.unwrap()).unwrap());
    }
}
