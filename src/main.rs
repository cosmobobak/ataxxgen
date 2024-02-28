

fn main() {
    let board = ataxxgen::Board::new();
    for depth in 0.. {
        let nodes = ataxxgen::perft(&board, depth);
        println!("depth {}: {}", depth, nodes);
    }
}