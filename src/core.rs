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
    data: Vec<Vec<usize>>,
}

impl AdjacencyMatrix {
    fn new(location_count: usize, point_data: &PointData, neighbour_distance: f32) -> Self {
        let mut ret = Self {
            data: vec![Vec::with_capacity(location_count); location_count],
        };

        for (index, row) in ret.data.iter_mut().enumerate() {
            for (point, distance) in point_data.distance_matrix[index].iter().enumerate() {
                if *distance <= neighbour_distance {
                    row.push(point);
                }
            }
        }

        ret
    }
}

#[derive(Clone)]
struct PointVec {
    data: Vec<bool>,
    true_count: usize,
}

impl PointVec {
    fn new(location_count: usize, fill: bool) -> Self {
        Self {
            data: vec![fill; location_count],
            true_count: if fill { location_count } else { 0 },
        }
    }

    fn len(&self) -> usize {
        self.true_count
    }

    fn next(&self) -> Option<usize> {
        self.data.iter().position(|x| *x)
    }

    fn remove(&mut self, index: usize) {
        self.true_count -= self.data[index] as usize;
        self.data[index] = false;
    }

    fn insert(&mut self, index: usize) {
        self.true_count += !self.data[index] as usize;
        self.data[index] = true;
    }
}

impl From<PointVec> for Box<[usize]> {
    fn from(val: PointVec) -> Self {
        val.data
            .iter()
            .enumerate()
            .filter(|(_index, value)| **value)
            .map(|(index, _value)| -> usize { index })
            .collect()
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
    for point in adjacency_matrix.data[point].iter() {
        new_data.remaining_points.remove(*point);
    }

    if let Some(result) = search(new_data, adjacency_matrix, select_size) {
        return Some(result);
    };

    // Do not pick this point
    search(solve_data, adjacency_matrix, select_size)
}

pub fn p_solver(input_data: &[Point], placements: u32) -> Option<Box<[usize]>> {
    let input_size = input_data.len();
    let mut point_data = PointData::new(input_data.len());
    let mut possible_point_distance = Vec::new();

    for (point_input, point_data) in zip(input_data, point_data.location.iter_mut()) {
        point_data.set(point_input);
    }

    for (point_a, row) in point_data.distance_matrix.iter_mut().enumerate() {
        for (point_b, distance) in row.iter_mut().enumerate() {
            *distance = point_data.location[point_a].get_distance(&point_data.location[point_b]);
            possible_point_distance.push(*distance);
        }
    }

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

    best_result.map(|data| {
        data.selected_points.into()
    })
}
