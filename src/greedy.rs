use crate::Point;

pub fn solver(input_data: &[Point], placements: u8) -> Option<Box<[usize]>> {
    let mut distance = vec![f32::MAX; input_data.len()];
    let mut max_distance = 0.;
    let mut max_distance_idx = 0;

    // do the first iteration
    distance[0] = 0.;
    for (idx, item) in distance.iter_mut().enumerate() {
        if *item == 0. {
            continue;
        }

        let point_dist = input_data[0].get_distance(&input_data[idx]);
        if point_dist > max_distance {
            max_distance = point_dist;
            max_distance_idx = idx;
        }

        *item = point_dist;
    }

    // already did 1 out of placements
    for _ in 1..placements {
        // Pick the largest distance
        let picked_point = max_distance_idx;

        max_distance = 0.;
        distance[max_distance_idx] = 0.;

        for (idx, item) in distance.iter_mut().enumerate() {
            if *item == 0. {
                continue;
            }

            let point_dist = input_data[picked_point].get_distance(&input_data[idx]);

            if point_dist <= *item {
                *item = point_dist
            }

            if *item > max_distance {
                max_distance = *item;
                max_distance_idx = idx;
            }
        }
    }

    // extract result
    Some(
        distance
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| {
                if *item == 0. {
                    return Some(idx);
                }
                None
            })
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    )
}
