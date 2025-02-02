use std::time::Instant;
use tokio::task::JoinSet;
use clap::{arg, Arg};
use clap::Command;

const KNIGHT_MOVES: [(i8, i8); 8] = [(2, 1), (2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2), (-2, 1), (-2, -1)];

fn knights_tour_simple(pos: u8, step: u8, board: u64, nr_fields: u8, target_pos: u8, knight_moves: &[u64;64], solutions: &mut u64, nr_nodes: &mut u64, start: &Instant) {
    *nr_nodes += 1;
    if pos == target_pos {
        if step == nr_fields{
            if *solutions & 0xFFFFF == 0 { //16k
             let elapsed = start.elapsed().as_secs_f32();
                println!("{:6} Solutions in {:8.3}s {:8.2} Solutions/s {:13} Nodes", *solutions + 1, elapsed, (*solutions + 1) as f32/elapsed, nr_nodes);
             }
            *solutions += 1;
        }
    } else {
        //compute reachability
        let mut moves  = knight_moves[pos as usize] & !board;
        let mut additional_moves = [0u16;7];
        // let mut additional_moves = 0u64;
        let mut count_additional_move = 0;
        while moves != 0 {
            let next_pos = moves.trailing_zeros() as u8;
            moves = moves & (moves - 1);
            let reachable_map  = knight_moves[next_pos as usize] & !board;
            let mut nr_reachable = reachable_map.count_ones() as u8;
            if next_pos == target_pos {
                nr_reachable = 10;
            }
            if nr_reachable  == 0 {
                return;
            }
            if nr_reachable == 1 && next_pos != target_pos {
                knights_tour_simple(next_pos, step + 1, board | (1 << next_pos), nr_fields, target_pos, knight_moves, solutions, nr_nodes, start);
                return;
            } else {
                // additional_moves = additional_moves | (1 << next_pos);
                additional_moves[count_additional_move] = ((nr_reachable as u16) << 8) + next_pos as u16;
                count_additional_move +=1;
            }
        }

        build_heap(&mut additional_moves, count_additional_move);
        while count_additional_move > 0 {
            let next_pos = (additional_moves[0] & 0xFF) as u8;
            knights_tour_simple(next_pos, step + 1, board | (1 << next_pos), nr_fields, target_pos, knight_moves, solutions, nr_nodes, start);
            remove_min_from_heap(&mut additional_moves, &mut count_additional_move);
        }

        // while additional_moves != 0 {
        //     let next_pos = additional_moves.trailing_zeros() as u8;
        //     additional_moves = additional_moves & (additional_moves - 1);
        //     knights_tour_simple(next_pos, step + 1, board | (1 << next_pos), nr_fields, target_pos, knight_moves, solutions, nr_nodes, start);
        // }
    }
}

fn precompute_knight_moves(size_x: u8, size_y: u8) -> [u64;64] {
    let mut precomputed = [0u64;64];
    for position in 0..(size_x * size_y) {
        let x = (position % size_x) as i8; // Convert 1D index to 2D coords
        let y = (position / size_x) as i8;

        let mut moves = 0u64;

        // Generate all valid moves
        for &(dx, dy) in KNIGHT_MOVES.iter() {
            let nx = x + dx;
            let ny = y + dy;

            if nx >= 0 && nx < size_x as i8 && ny >= 0 && ny < size_y as i8 {
                let new_position = (ny as usize) * size_x as usize + (nx as usize);
                moves |= 1 << new_position;
            }
        }
        precomputed[position as usize] = moves;
    }

    precomputed
}


fn create_tasks(pos: u8, step: u8, depth: u8, board: u64,  target_pos: u8, knight_moves: [u64;64], vec: &mut Vec<(u64, u8, u8, u32)>) {
    if pos == target_pos {
        return;
    }
    if step == depth {
        vec.push((board.clone(), pos, step, vec.len() as u32));
        return;
    }
        //compute reachbiliy
        let mut moves  = knight_moves[pos as usize] & !board;
        let mut additional_moves = 0u64;//[0u16;7];
        while moves != 0 {
            let next_pos = moves.trailing_zeros() as u8;
            moves = moves & (moves - 1);
            let reachable_map  = knight_moves[next_pos as usize] & !board;
            let mut nr_reachable = reachable_map.count_ones() as u8;
            if next_pos == target_pos {
                nr_reachable = 10;
            }
            if nr_reachable  == 0 {
                return;
            }
            if nr_reachable == 1 && next_pos != target_pos {
                create_tasks(next_pos, step + 1,  depth, board | (1 << next_pos), target_pos, knight_moves, vec);
                return;
            } else {
                additional_moves = additional_moves | (1 << next_pos);
            }
        }

        while additional_moves != 0 {
            let next_pos = additional_moves.trailing_zeros() as u8;
            additional_moves = additional_moves & (additional_moves - 1);
            create_tasks(next_pos, step + 1, depth, board | (1 << next_pos), target_pos, knight_moves, vec);
        }
}

#[inline(always)]
fn remove_min_from_heap<T: std::cmp::PartialOrd>(arr: &mut [T], n: &mut usize) {
    *n -= 1;
    arr.swap(0, *n );
    if *n > 1 {
        heapify(arr, *n, 0);
    }
}

#[inline(always)]
fn build_heap<T: std::cmp::PartialOrd>(arr: &mut [T], n: usize) {
    if n > 1 {
        for i in (0..n / 2).rev() {
            heapify(arr, n, i);
        }
    }
}

#[inline(always)]
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
            .arg(Arg::new("depth")
                .help("depth to crate tasks")
                .long("depth")
                .short('d')
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
            let d_arg = sub_m.get_one::<usize>("depth");
            find_knight_tour_on(x_arg.unwrap().clone() as u8, y_arg.unwrap().clone() as u8, d_arg.unwrap().clone() as u8);
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
                            find_knight_tour_on(x,y, 6);
                        }
                    }
                }
            }
        }
        _ => unreachable!("Exhausted list of subcommands"),
    }

}

#[allow(dead_code)]
fn printboard(board: &Vec<i8>, size_x: usize, size_y: usize) {
    for y in 0..size_y {
        for x in 0..size_x {
            print!("{:^4}", board[y * size_x + x]);
        }
        println!();
    }
}

fn find_knight_tour_on(size_x: u8, size_y: u8, depth: u8) {
    let target_pos = size_y * size_x - 1;
    let start = target_pos - 1  - size_x * 2;

    let knight_moves  = precompute_knight_moves( size_x, size_y);

    tokio::runtime::Runtime::new().unwrap().block_on(process_knight_tour_parallel(size_x, size_y, depth,1 << start, target_pos, start, knight_moves ));
}

async fn process_knight_tour_parallel(size_x: u8, size_y: u8, depth: u8, board: u64, target_pos: u8, pos: u8, knight_jumps: [u64;64]) {
    eprintln!("Starting on {:2}x{:<2} ", size_x, size_y);
    let nr_fields = size_y * size_x;

    let start = Instant::now();
    let mut tasks = vec![];
    create_tasks(pos, 1, depth, board, target_pos, knight_jumps, &mut tasks);
    eprintln!("Created {:6} Tasks", tasks.len());
    // if tasks.len() > 500 {
    //     tasks = tasks[0..500].to_vec();
    // }

    let mut set = JoinSet::new();

    let mut solutions = 0;
    let mut nr_nodes = 0;
    for task in tasks {
        set.spawn(run_task(target_pos, knight_jumps, start, nr_fields, task));
    }

    let mut output = vec![];
    while let Some(res) = set.join_next().await {
        match res {
            Ok(res) => {
                solutions += res.0;
                nr_nodes += res.1;
                let task = res.2;
                let elapsed_time = start.elapsed().as_secs_f32();
                output.push((task, solutions, elapsed_time, solutions as f32 / elapsed_time, nr_nodes));
                //eprintln!("Task: {} has {:8} Solutions in {:8.3}s {:8.2} Solutions/s {:13} Nodes",task, solutions, elapsed_time, solutions as f32 / elapsed_time, nr_nodes );
            }

            Err(e) => {
                eprintln!("Error: {}", e);
            }
        };

    }
    for output in output.iter() {
        let (task, solutions, elapsed_time, sol_per_s, nr_nodes) = output;
        eprintln!("Task: {} has {:8} Solutions in {:8.3}s {:8.2} Solutions/s {:13} Nodes",task, solutions, elapsed_time, sol_per_s, nr_nodes );
    }
    // for i in 1..100 {
    //     if i  < output.len() {
    //         let (task, solutions, elapsed_time, sol_per_s, nr_nodes) = output[i];
    //         eprintln!("Task: {} has {:8} Solutions in {:8.3}s {:8.2} Solutions/s {:13} Nodes",task, solutions, elapsed_time, sol_per_s, nr_nodes );
    //     }
    // }
    let elapsed_time = start.elapsed().as_secs_f32();
    println!("On {:2}x{:<2} {:7} Solutions in {:8.3}s {:8.2} Solutions/s {:13} Nodes", size_x, size_y, solutions, elapsed_time, solutions as f32 / elapsed_time, nr_nodes);
}

async fn run_task(target_pos: u8, knight_jumps: [u64;64], start: Instant, nr_fields: u8, task: (u64, u8, u8, u32)) -> (u64, u64, u32) {
    let mut solutions = 0;
    let mut nodes = 0;
    knights_tour_simple(task.1, task.2, task.0, nr_fields, target_pos, &knight_jumps, &mut solutions, &mut nodes, &start);
    //eprintln!("Task: {} has {:7} Solutions testet {:13} Nodes", task.3, solutions, nodes);
    (solutions, nodes, task.3)
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
