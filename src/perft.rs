use crate::Board;

pub fn perft(board: &Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    if depth == 1 {
        let mut count = 0;
        board.generate_moves(|_| {
            count += 1;
            false
        });
        return count;
    }

    let mut count = 0;
    board.generate_moves(|mv| {
        let mut board = *board;
        board.make_move(mv);
        count += perft(&board, depth - 1);
        false
    });

    count
}

pub fn generate_depth_n_fens(board: Board, mut fen_receiver: impl FnMut(String) + Copy, depth: u8) {
    if depth == 0 {
        fen_receiver(board.fen());
        return;
    }

    board.generate_moves(|mv| {
        let mut board = board;
        board.make_move(mv);
        generate_depth_n_fens(board, fen_receiver, depth - 1);
        false
    });
}