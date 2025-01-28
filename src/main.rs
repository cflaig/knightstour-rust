use std::time::Instant;
use ndarray::{ArrayBase, Ix2, OwnedRepr};
use clap::{arg, Arg};
use clap::Command;

const KNIGHT_MOVES: [(i8, i8); 8] = [(2, 1), (2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2), (-2, 1), (-2, -1)];

#[allow(dead_code)]
fn knights_tour_simple(pos: (usize, usize), step: usize, board: &mut ArrayBase<OwnedRepr<i8>, Ix2>, nr_fields: usize, target_pos: &(usize, usize), solutions: &mut u64, nr_nodes: &mut u64, start: &Instant) {
    *nr_nodes += 1;
    let backup = board[pos];
    board[pos] = -(step as i8);
    if step == nr_fields && pos == *target_pos {
        if *solutions & 0x3FFF == 0 { //16k
            let elapsed = start.elapsed().as_secs_f32();
            println!("{:6} Solutions in {:8.3}s {:8.2} Solutions/s {:13} Nodes", *solutions + 1, elapsed, (*solutions + 1) as f32/elapsed, nr_nodes);
        }
        *solutions += 1;
    } else {
        for delta in KNIGHT_MOVES {
            let new_pos = ((pos.0 as i8 + delta.0) as usize, (pos.1 as i8 + delta.1) as usize);
            if board[new_pos] > 0 {
                knights_tour_simple(new_pos, step+1, board, nr_fields, target_pos, solutions, nr_nodes, start);
            }
        }
    }
    board[pos] = backup;
}

fn knights_tour(pos: (usize, usize), step: usize, board: &mut ArrayBase<OwnedRepr<i8>, Ix2>, nr_fields: usize, target_pos: &(usize, usize), solutions: &mut u64, nr_nodes: &mut u64, start: &Instant) {
    *nr_nodes += 1;
    let backup = board[pos];
    board[pos] = -(step as i8);
    if step ==nr_fields && pos == *target_pos {
        if *solutions & 0x7FFF == 0 { //16k
            let elapsed = start.elapsed().as_secs_f32();
            eprintln!("{:6} Solutions in {:8.3}s {:8.2} Solutions/s {:13} Nodes", *solutions + 1, elapsed, (*solutions + 1) as f32/elapsed, nr_nodes);
        }
        *solutions += 1;
    } else {
        let mut moves = [(0, (0,0));7];
        let mut i = 0;
        for delta in KNIGHT_MOVES {
            let new_pos = ((pos.0 as i8 + delta.0) as usize, (pos.1 as i8 + delta.1) as usize);
            let reachable = board[new_pos];
            if reachable > 0 {
                board[new_pos] -= 1;
                moves[i] = (board[new_pos], new_pos);
                i += 1;
            }
        };

        build_heap(&mut moves, i);

        while i > 0 && moves[0].0 > 0 {
            knights_tour(moves[0].1, step+1, board, nr_fields, target_pos, solutions, nr_nodes, start);
            remove_min_from_heap(&mut moves, &mut i);
        }

        for delta in KNIGHT_MOVES {
            let new_pos = ((pos.0 as i8 + delta.0) as usize, (pos.1 as i8 + delta.1) as usize);
            if board[new_pos] >= 0 {
                board[new_pos] +=1;
            }
        };
    }
    board[pos] = backup;
}

fn remove_min_from_heap<T: std::cmp::PartialOrd>(arr: &mut [T], n: &mut usize) {
    *n -= 1;
    arr.swap(0, *n );
    if *n > 1 {
        heapify(arr, *n, 0);
    }
}
fn build_heap<T: std::cmp::PartialOrd>(arr: &mut [T], n: usize) {
    if n > 1 {
        for i in (0..n / 2).rev() {
            heapify(arr, n, i);
        }
    }
}
fn heapify<T: std::cmp::PartialOrd>(arr: &mut [T], n: usize, mut i: usize) {
    loop {
        let mut smallest = i;
        let l = 2 * i + 1;
        let r = 2 * i + 2;
        if l < n && arr[l] < arr[smallest] {
            smallest = l;
        }
        if r < n && arr[r] < arr[smallest] {
            smallest = r;
        }
        if smallest == i {
            break;
        }
        arr.swap(i, smallest);
        i = smallest;
    }
}

fn main() {

    let matches = clap::command!()
        .version("v0.0.1")
        .propagate_version(true)
        .arg(arg!(
            -d --debug "Turn debugging information on"
        ))
        .subcommand(Command::new("benchmark")
            .about("Runs a benchmark")
            .arg(Arg::new("x_axis")
                .help("x-dimension of the board")
                .long("x_axis")
                .short('x')
                .num_args(1)
                .default_value("6")
                .value_parser(clap::value_parser!(usize)))
                .arg(Arg::new("y_axis")
                    .help("y-dimension of the board")
                    .long("y_axis")
                    .short('y')
                    .num_args(1)
                    .default_value("6")
                    .value_parser(clap::value_parser!(usize)))
            )
        .get_matches();

    let _debug = matches.get_flag("debug");

    match matches.subcommand() {
        Some(("benchmark", sub_m)) => {
            let x_arg = sub_m.get_one::<usize>("x_axis");
            let y_arg = sub_m.get_one::<usize>("y_axis");
            find_knight_tour_on(x_arg.unwrap().clone(), y_arg.unwrap().clone());
        }
        None => {
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
        _ => unreachable!("Exhausted list of subcommands"),
    }

}

fn find_knight_tour_on(size_x: usize, size_y: usize) {
    let mut board = init_board(size_x, size_y);

    let target_pos = (size_x + 1, size_y + 1);
    let pos = (target_pos.0 - 2, target_pos.1 - 1);

    board[target_pos] = 10;
    //println!("{:?}", board);
    let mut solution = 0;
    let mut nr_nodes = 0;

    let start = Instant::now();
    eprintln!("Starting on {:2}x{:<2} ", size_x, size_y);
    knights_tour(pos, 1, &mut board, size_y * size_x, &target_pos, &mut solution, &mut nr_nodes, &start);
    let elapsed_time = start.elapsed().as_secs_f32();
    println!("On {:2}x{:<2} {:7} Solutions in {:8.3}s {:8.2} Solutions/s {:13} Nodes", size_x, size_y, solution, elapsed_time, solution as f32/elapsed_time, nr_nodes);
}

fn init_board(size_x: usize, size_y: usize) -> ArrayBase<OwnedRepr<i8>, Ix2> {
    let mut board = ndarray::Array2::<i8>::from_elem((size_x + 4, size_y + 4), -99);
    for i in 2..board.nrows() - 2 {
        for j in 2..board.ncols() - 2 {
            board[[i, j]] = 0;
        }
    }
    for i in 2..board.nrows() - 2 {
        for j in 2..board.ncols() - 2 {
            for delta in KNIGHT_MOVES {
                let new_pos = ((i as i8 + delta.0) as usize, (j as i8 + delta.1) as usize);
                if board[new_pos] >= 0 {
                    board[new_pos] += 1;
                }
            }
        }
    }
    board
}

#[cfg(test)]
mod tests {
    use rand::rng;
    use rand::seq::SliceRandom;

    use super::*;

    #[test]
    fn bin_heap_test() {
        for j in 1..20 {
            let mut heap = (1..j).collect::<Vec<i8>>();
            heap.shuffle(&mut rng());

            let mut size = heap.len();

            build_heap(&mut heap, size);

            if j > 1 {
                for i in 0..(heap.len() - 1) / 2 {
                    assert!(heap[i] < heap[2 * i + 1]);
                    if j > 2 {
                        assert!(heap[i] < heap[2 * i + 2]);
                    }
                }
            }

        for i in 1..size + 1 {
                assert_eq!(heap[0], i as i8);
                remove_min_from_heap(&mut heap, &mut size);
            }
        }
    }
}
