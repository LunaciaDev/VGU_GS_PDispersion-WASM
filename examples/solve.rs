use p_dispersion::{Point, solve_p_dispersion_rs};

fn main() {
    let mut input = Vec::with_capacity(150);

    for row in 0..10 {
        for col in 0..15 {
            input.push(Point::new(row as f32, col as f32));
        }
    }

    println!("{:?}", solve_p_dispersion_rs(&input, 7).unwrap());
}