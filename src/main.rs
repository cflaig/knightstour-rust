use std::time::Instant;
use ndarray::{ArrayBase, Ix2, OwnedRepr};
use clap::{arg, Arg};
use clap::Command;

const KNIGHT_MOVES: [(i8, i8); 8] = [(2, 1), (2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2), (-2, 1), (-2, -1)];

#[allow(dead_code)]
fn knights_tour_simple(pos: u8, step: u8, board: &mut Vec<i8>, nr_fields: u8, target_pos: u8, knight_jumps: &[i8;8], solutions: &mut u64, nr_nodes: &mut u64, start: &Instant) {
    *nr_nodes += 1;
    let backup = board[pos as usize];
    board[pos as usize] = -(step as i8);
    if step == nr_fields && pos == target_pos {
        if *solutions & 0x3FFF == 0 { //16k
            let elapsed = start.elapsed().as_secs_f32();
            println!("{:6} Solutions in {:8.3}s {:8.2} Solutions/s {:13} Nodes", *solutions + 1, elapsed, (*solutions + 1) as f32/elapsed, nr_nodes);
        }
        *solutions += 1;
    } else {
        for delta in knight_jumps {
            let new_pos = (pos as i8 + delta) as u8;
            if board[new_pos as usize] > 0 {
                knights_tour_simple(new_pos, step+1, board, nr_fields, target_pos, knight_jumps, solutions, nr_nodes, start);
            }
        }
    }
    board[pos as usize] = backup;
}

fn knights_tour(pos: u8, step: u8, board: &mut Vec<i8>, nr_fields: u8, target_pos: u8, knight_jumps: &[i8;8], solutions: &mut u64, nr_nodes: &mut u64, start: &Instant) {
    *nr_nodes += 1;
    let backup = board[pos as usize];
    board[pos as usize] = -(step as i8);
    if step == nr_fields && pos == target_pos {
        if *solutions & 0x7FFF == 0 { //16k
            let elapsed = start.elapsed().as_secs_f32();
            eprintln!("{:6} Solutions in {:8.3}s {:8.2} Solutions/s {:13} Nodes", *solutions + 1, elapsed, (*solutions + 1) as f32/elapsed, nr_nodes);
        }
        *solutions += 1;
    } else {
        let mut moves = [(0, 0u8);7];
        let mut i = 0;
        for delta in knight_jumps {
            let new_pos = (pos as i8 + delta) as u8;
            let reachable = board[new_pos as usize];
            if reachable > 0 {
                board[new_pos as usize] -= 1;
                moves[i] = (board[new_pos as usize], new_pos);
                i += 1;
            }
        };

        build_heap(&mut moves, i);

        while i > 0 && moves[0].0 > 0 {
            knights_tour(moves[0].1, step+1, board, nr_fields, target_pos, knight_jumps, solutions, nr_nodes, start);
            if moves[0].0 == 1 {
                break;
            }
            remove_min_from_heap(&mut moves, &mut i);
        }

        for delta in knight_jumps {
            let new_pos = (pos as i8 + delta) as u8;
            if board[new_pos as usize] >= 0 {
                board[new_pos as usize] +=1;
            }
        };
    }
    board[pos as usize] = backup;
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
            find_knight_tour_on(x_arg.unwrap().clone() as u8, y_arg.unwrap().clone() as u8);
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

fn printboard(board: &Vec<i8>, size_x: usize, size_y: usize) {
    for y in 0..size_y {
        for x in 0..size_x {
            print!("{:^4}", board[y * size_x + x]);
        }
        println!();
    }
}

fn find_knight_tour_on(size_x: u8, size_y: u8) {
    let board_size_x = size_x + 4;
    let board_size_y = size_y + 4;

    let mut board = init_board(size_x, size_y);

    let target_pos =  (size_y + 1) * board_size_x + size_x + 1;
    let pos = target_pos -2 * board_size_x - 1;

    board[target_pos as usize] = 10;
    //printboard(&board, board_size_x, board_size_y);
    let mut solution = 0;
    let mut nr_nodes = 0;

    let mut knight_jumps = [0i8; 8];
    for (i,delta) in KNIGHT_MOVES.iter().enumerate() {
        knight_jumps[i] = delta.1 * board_size_x as i8 + delta.0;
    }

    let start = Instant::now();
    eprintln!("Starting on {:2}x{:<2} ", size_x, size_y);
    knights_tour(pos, 1, &mut board, size_y * size_x, target_pos, &knight_jumps, &mut solution, &mut nr_nodes, &start);
    let elapsed_time = start.elapsed().as_secs_f32();
    println!("On {:2}x{:<2} {:7} Solutions in {:8.3}s {:8.2} Solutions/s {:13} Nodes", size_x, size_y, solution, elapsed_time, solution as f32/elapsed_time, nr_nodes);
}

fn init_board(size_x: u8, size_y: u8) -> Vec<i8> {
    let board_size_x = size_x + 4;
    let board_size_y = size_y + 4;
    let mut board = vec![-99i8; ((board_size_x) * (board_size_y)) as usize];
    for i in 2.. board_size_x - 2 {
        for j in 2..board_size_y - 2 {
            board[(j*(board_size_x) + i) as usize] = 0;
        }
    }
    for i in 2..board_size_x - 2 {
        for j in 2..board_size_y - 2 {
            for delta in KNIGHT_MOVES {
                let new_pos = ((i as i8 + delta.0) as u8, (j as i8 + delta.1) as u8);
                if board[(new_pos.1*board_size_x + new_pos.0) as usize] >= 0 {
                    board[(new_pos.1*board_size_x + new_pos.0) as usize] += 1;
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
