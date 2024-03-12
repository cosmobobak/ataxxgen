use ataxxgen::{Board, perft};

fn main() {
    // run benchmarks...

    println!("Starting position: \n{}", Board::default());

    // // perft depth 4 on a 15x15 board:
    // let start_time = std::time::Instant::now();
    // let count = perft::perft(board::Board::<15>::new(), 4);
    // let elapsed = start_time.elapsed();
    // println!("perft depth 4 on a 15x15 board: {} nodes in {}.{:03}s", count, elapsed.as_secs(), elapsed.subsec_millis());
    // println!("nodes per second: {:.2}", count as f64 / elapsed.as_secs_f64());

    // // perft depth 4 on a 17x17 board:
    // let start_time = std::time::Instant::now();
    // let count = perft::perft(board::Board::<17>::new(), 4);
    // let elapsed = start_time.elapsed();
    // println!("perft depth 4 on a 17x17 board: {} nodes in {}.{:03}s", count, elapsed.as_secs(), elapsed.subsec_millis());
    // println!("nodes per second: {:.2}", count as f64 / elapsed.as_secs_f64());

    // // perft depth 4 on a 19x19 board:
    // let start_time = std::time::Instant::now();
    // let count = perft::perft(board::Board::<19>::new(), 4);
    // let elapsed = start_time.elapsed();
    // println!("perft depth 4 on a 19x19 board: {} nodes in {}.{:03}s", count, elapsed.as_secs(), elapsed.subsec_millis());
    // println!("nodes per second: {:.2}", count as f64 / elapsed.as_secs_f64());

    perft::generate_depth_n_fens(Board::default(), |fen| println!("{fen}"), 2);
}