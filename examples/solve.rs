use std::env;

use p_dispersion::{Point, solve_p_dispersion_rs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: solve row_count col_count select_count");
        return;
    }

    let row_count = args[1].parse::<u32>().expect("row_count must be u32");
    let col_count = args[2].parse::<u32>().expect("col_count must be u32");
    let select_count = args[3].parse::<u32>().expect("select_count must be u32");

    let mut input = Vec::with_capacity((row_count * col_count) as usize);

    for row in 0..row_count {
        for col in 0..col_count {
            input.push(Point::new(row as f32, col as f32));
        }
    }

    println!("{:?}", solve_p_dispersion_rs(&input, select_count).unwrap());
}