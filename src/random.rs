use crate::Point;
use rand::seq::index::sample;

pub fn solver(input_data: &[Point], placements: u8) -> Option<Box<[usize]>> {
    let mut rng = rand::rng();

    Some(
        sample(&mut rng, input_data.len(), placements as usize)
            .into_vec()
            .into_boxed_slice(),
    )
}
