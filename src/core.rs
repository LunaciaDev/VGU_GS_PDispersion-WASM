use std::iter::zip;

use crate::Point;

#[derive(Debug)]
struct PointData {
    location: Vec<Point>,
    distance_matrix: Vec<Vec<f32>>,
}

impl PointData {
    fn new(location_count: usize) -> Self {
        Self {
            location: vec![Point::default(); location_count],
            distance_matrix: vec![vec![0.; location_count]; location_count],
        }
    }
}

struct AdjacencyMatrix {
    data: Vec<PointVec>,
}

impl AdjacencyMatrix {
    fn new(location_count: usize, point_data: &PointData, neighbour_distance: f32) -> Self {
        let mut ret = Self {
            data: vec![PointVec::new(location_count, false); location_count],
        };

        for (index, row) in ret.data.iter_mut().enumerate() {
            for (point, distance) in point_data.distance_matrix[index].iter().enumerate() {
                if *distance <= neighbour_distance {
                    row.insert(point);
                }
            }
        }

        ret
    }
}

#[derive(Clone)]
struct PointVec {
    data: Vec<u32>,
    true_count: usize,
}

impl PointVec {
    fn new(location_count: usize, fill: bool) -> Self {
        let sector_count = location_count / 32 + !location_count.is_multiple_of(32) as usize;
        if fill {
            let mut ret = Self {
                data: vec![u32::MAX; sector_count],
                true_count: location_count,
            };

            if !location_count.is_multiple_of(32) {
                *ret.data.last_mut().unwrap() &= 2_u32.pow(location_count as u32 % 32) - 1;
            }

            ret
        } else {
            Self {
                data: vec![0; sector_count],
                true_count: 0,
            }
        }
    }

    fn len(&self) -> usize {
        self.true_count
    }

    fn next(&self) -> Option<usize> {
        self.data
            .iter()
            .position(|value| *value != 0)
            .map(|index| 32 * index + self.data[index].trailing_zeros() as usize)
    }

    fn subtract(&mut self, rhs: &PointVec) {
        self.true_count = 0;
        for (lhs, rhs) in zip(self.data.iter_mut(), rhs.data.iter()) {
            *lhs &= !rhs;
            self.true_count += lhs.count_ones() as usize;
        }
    }

    fn remove(&mut self, index: usize) {
        let sector = index / 32;
        let bitmask = 1 << (index % 32);

        self.true_count -= (self.data[sector] & bitmask == bitmask) as usize;
        self.data[sector] &= !bitmask;
    }

    fn insert(&mut self, index: usize) {
        let sector = index / 32;
        let bitmask = 1 << (index % 32);

        self.true_count += (self.data[sector] & bitmask != bitmask) as usize;
        self.data[sector] |= bitmask;
    }
}

impl From<PointVec> for Box<[usize]> {
    fn from(val: PointVec) -> Self {
        let mut result: Vec<usize> = Vec::new();
        for (index, value) in val
            .data
            .iter()
            .enumerate()
            .filter(|(_index, value)| **value != 0)
        {
            // copy out the u32 ref
            let mut value = *value;

            while value > 0 {
                result.push(index * 32 + value.trailing_zeros() as usize);
                value ^= 1 << value.trailing_zeros();
            }
        }

        result.into_boxed_slice()
    }
}

#[derive(Clone)]
struct SolveData {
    selected_points: PointVec,
    remaining_points: PointVec,
}

impl SolveData {
    fn new(location_count: usize) -> Self {
        Self {
            selected_points: PointVec::new(location_count, false),
            remaining_points: PointVec::new(location_count, true),
        }
    }
}

impl Point {
    fn set(&mut self, point: &Point) {
        self.x = point.x;
        self.y = point.y;
    }

    fn get_distance(&self, point: &Point) -> f32 {
        ((self.x - point.x).powi(2) + (self.y - point.y).powi(2)).sqrt()
    }
}

fn search(
    mut solve_data: Box<SolveData>,
    adjacency_matrix: &AdjacencyMatrix,
    select_size: usize,
) -> Option<Box<SolveData>> {
    if solve_data.selected_points.len() >= select_size {
        return Some(solve_data);
    }

    if solve_data.remaining_points.len() < select_size - solve_data.selected_points.len() {
        return None;
    }

    // pick next point
    let point = solve_data
        .remaining_points
        .next()
        .expect("At least one point remaining");
    solve_data.remaining_points.remove(point);

    // Pick this point
    let mut new_data = solve_data.clone();

    new_data.selected_points.insert(point);
    new_data
        .remaining_points
        .subtract(&adjacency_matrix.data[point]);

    if let Some(result) = search(new_data, adjacency_matrix, select_size) {
        return Some(result);
    };

    // Do not pick this point
    search(solve_data, adjacency_matrix, select_size)
}

pub fn p_solver(input_data: &[Point], placements: u32) -> Option<Box<[usize]>> {
    let input_size = input_data.len();
    let mut point_data = PointData::new(input_data.len());

    for (point_input, point_data) in zip(input_data, point_data.location.iter_mut()) {
        point_data.set(point_input);
    }

    for (point_a, row) in point_data.distance_matrix.iter_mut().enumerate() {
        for (point_b, distance) in row.iter_mut().enumerate() {
            *distance = point_data.location[point_a].get_distance(&point_data.location[point_b]);
        }
    }

    let mut possible_point_distance = point_data
        .distance_matrix
        .first()
        .expect("Distance matrix must not be empty")
        .clone();
    possible_point_distance.sort_by(f32::total_cmp);

    let mut left_index = 0;
    let mut right_index = possible_point_distance.len() - 1;
    let mut best_result: Option<Box<SolveData>> = None;

    while left_index < right_index {
        let target = left_index.midpoint(right_index);

        match search(
            Box::new(SolveData::new(input_size)),
            &AdjacencyMatrix::new(input_size, &point_data, possible_point_distance[target]),
            placements as usize,
        ) {
            Some(result) => {
                left_index = target + 1;
                best_result = Some(result);
            }
            None => right_index = target,
        }
    }

    best_result.map(|data| data.selected_points.into())
}
