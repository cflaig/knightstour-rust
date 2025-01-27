use std::time::Instant;
use ndarray::{ArrayBase, Ix2, OwnedRepr};

const KNIGHT_MOVES: [(i8, i8); 8] = [(2, 1), (2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2), (-2, 1), (-2, -1)];

fn knights_tour_simple(pos: (usize, usize), step: usize, board: &mut ArrayBase<OwnedRepr<i8>, Ix2>, nr_fields: usize, solutions: &mut u64, nr_nodes: &mut u64, start: &Instant) {
    *nr_nodes += 1;
    let backup = board[pos];
    board[pos] = -(step as i8);
    if step == nr_fields /*&& pos == (2,2)*/ {
        if *solutions % 1000 == 0 {
            let elapsed = start.elapsed().as_secs_f32();
            println!("{:6} Solutions in {:8.3}s {:8.2} Solutions/s {:13} Nodes", *solutions + 1, elapsed, (*solutions + 1) as f32/elapsed, nr_nodes);
        }
        *solutions += 1;
    } else {
        for delta in KNIGHT_MOVES {
            let new_pos = ((pos.0 as i8 + delta.0) as usize, (pos.1 as i8 + delta.1) as usize);
            if board[new_pos] > 0 {
                knights_tour_simple(new_pos, step+1, board, nr_fields, solutions, nr_nodes, start);
            }
        }
    }
    board[pos] = backup;
}

fn knights_tour(pos: (usize, usize), step: usize, board: &mut ArrayBase<OwnedRepr<i8>, Ix2>, nr_fields: usize, solutions: &mut u64, nr_nodes: &mut u64, start: &Instant) {
    *nr_nodes += 1;
    let backup = board[pos];
    board[pos] = -(step as i8);
    if step ==nr_fields && pos == (2,2) {
        if *solutions % 10000 == 0 {
            let elapsed = start.elapsed().as_secs_f32();
            eprintln!("{:6} Solutions in {:8.3}s {:8.2} Solutions/s {:13} Nodes", *solutions + 1, elapsed, (*solutions + 1) as f32/elapsed, nr_nodes);
        }
        *solutions += 1;
    } else {
        let mut moves = [((0,0),0);8];
        for (i,delta) in KNIGHT_MOVES.iter().enumerate() {
            let new_pos = ((pos.0 as i8 + delta.0) as usize, (pos.1 as i8 + delta.1) as usize);
            board[new_pos] -= 1;
            moves[i] = (new_pos, board[new_pos]);
        };

        moves.sort_by(|a,b| a.1.cmp(&b.1));

        for (new_pos, nr_reachable) in moves.iter() {
            if *nr_reachable == 0 {
                break;
            }
            if *nr_reachable > 0 {
                knights_tour(*new_pos, step+1, board, nr_fields, solutions, nr_nodes, start);
            }
        }

        for delta in KNIGHT_MOVES {
            let new_pos = ((pos.0 as i8 + delta.0) as usize, (pos.1 as i8 + delta.1) as usize);
            board[new_pos] +=1;
        };
    }
    board[pos] = backup;
}

fn main() {
    for fields in 18..55 {
        for x in 3..7 {
            if x == 4 {
                continue; //no closed cycle on dim 4
            }
            if fields % 2 == 1 {
                continue; //no closed cycle due different amount of fields of each color
            }
            if fields % x == 0 {
                let y = fields / x;
                if y >= x {
                    find_knight_tour_on(x,y);
                }
            }
        }
    }
}

fn find_knight_tour_on(size_x: usize, size_y: usize) {
    let mut board = init_board(size_x, size_y);

    let pos = (4usize, 3usize);
    board[(2, 2)] = 10;
    //println!("{:?}", board);
    let mut solution = 0;
    let mut nr_nodes = 0;

    let start = Instant::now();
    eprintln!("Starting on {:2}x{:<2} ", size_x, size_y);
    knights_tour(pos, 1, &mut board, size_y * size_x, &mut solution, &mut nr_nodes, &start);
    let elapsed_time = start.elapsed().as_secs_f32();
    println!("On {:2}x{:<2} {:7} Solutions in {:8.3}s {:8.2} Solutions/s {:13} Nodes", size_x, size_y, solution, elapsed_time, solution as f32/elapsed_time, nr_nodes);
}

fn init_board(size_x: usize, size_y: usize) -> ArrayBase<OwnedRepr<i8>, Ix2> {
    let mut board = ndarray::Array2::<i8>::from_elem((size_y + 4, size_x + 4), -99);
    for i in 2..board.nrows() - 2 {
        for j in 2..board.ncols() - 2 {
            board[[i, j]] = 0;
        }
    }
    for i in 2..board.nrows() - 2 {
        for j in 2..board.ncols() - 2 {
            for delta in KNIGHT_MOVES {
                let new_pos = ((i as i8 + delta.0) as usize, (j as i8 + delta.1) as usize);
                board[new_pos] += 1;
            }
        }
    }
    board
}
