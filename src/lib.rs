use core::fmt;
use std::{collections::HashSet, iter::zip};

#[derive(Default, Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct NoPossibleDispersion;

impl fmt::Display for NoPossibleDispersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cannot find a p-dispersion solution to the provided problem"
        )
    }
}

impl std::error::Error for NoPossibleDispersion {}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    fn set(&mut self, point: &Point) {
        self.x = point.x;
        self.y = point.y;
    }

    fn get_distance(&self, point: &Point) -> f32 {
        ((self.x - point.x).powi(2) + (self.y - point.y).powi(2)).sqrt()
    }
}

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

#[derive(Clone)]
struct SolveData {
    select_size: usize,
    selected_points: HashSet<usize>,
    remaining_points: HashSet<usize>,
    adjacency_matrix: Vec<Vec<usize>>,
}

impl SolveData {
    fn new(
        location_count: usize,
        point_data: &PointData,
        neighbour_distance: f32,
        select_size: usize,
    ) -> Self {
        let mut data = Self {
            select_size,
            selected_points: HashSet::new(),
            adjacency_matrix: vec![Vec::new(); location_count],
            remaining_points: HashSet::new(),
        };

        data.selected_points.reserve(location_count);
        data.remaining_points.reserve(location_count);
        for i in 0..location_count {
            data.remaining_points.insert(i);
        }
        for (index, row) in data.adjacency_matrix.iter_mut().enumerate() {
            for (point, distance) in point_data.distance_matrix[index].iter().enumerate() {
                if *distance <= neighbour_distance {
                    row.push(point);
                }
            }
        }

        data
    }
}

fn search(mut solve_data: Box<SolveData>) -> Option<Box<SolveData>> {
    if solve_data.selected_points.len() >= solve_data.select_size {
        return Some(solve_data);
    }

    if solve_data.remaining_points.len() < solve_data.select_size - solve_data.selected_points.len()
    {
        return None;
    }

    // pick next point
    let point = *solve_data
        .remaining_points
        .iter()
        .next()
        .expect("Must have at least 1 remaining");

    // Pick this point
    let mut new_data = solve_data.clone();
    new_data.selected_points.insert(point);
    new_data.remaining_points.remove(&point);
    for point in new_data.adjacency_matrix[point].iter() {
        new_data.remaining_points.remove(point);
    }

    if let Some(result) = search(new_data) {
        return Some(result);
    };

    // Do not pick this point
    solve_data.remaining_points.remove(&point);
    search(solve_data)
}

pub fn solve_p_dispersion(
    input_array: &[Point],
    placements: u32,
) -> Result<Box<[usize]>, NoPossibleDispersion> {
    let input_size = input_array.len();

    let mut point_data = PointData::new(input_size);
    let mut possible_point_distance = Vec::new();

    for (point_input, point_data) in zip(input_array, point_data.location.iter_mut()) {
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
    let mut right_index = possible_point_distance.len();
    let mut largest_distance: f32 = 0.;
    let mut best_result: Option<Box<SolveData>> = None;

    while left_index < right_index {
        let target = (left_index + right_index) / 2;

        match search(Box::new(SolveData::new(
            input_size,
            &point_data,
            possible_point_distance[target],
            placements as usize,
        ))) {
            Some(result) => {
                right_index = target - 1;
                if possible_point_distance[target] > largest_distance {
                    largest_distance = possible_point_distance[target];
                    best_result = Some(result);
                }
            }
            None => {
                left_index = target + 1;
            }
        }
    }

    // [FIXME] Bandaid solution to the termination of the binary search

    if let Some(result) = search(Box::new(SolveData::new(
        input_size,
        &point_data,
        possible_point_distance[left_index],
        placements as usize,
    ))) && possible_point_distance[left_index] > largest_distance
    {
        largest_distance = possible_point_distance[left_index];
        best_result = Some(result);
    }

    if let Some(result) = search(Box::new(SolveData::new(
        input_size,
        &point_data,
        possible_point_distance[right_index],
        placements as usize,
    ))) && possible_point_distance[right_index] > largest_distance
    {
        best_result = Some(result);
    }

    match best_result {
        Some(data) => {
            // so it is possible...
            Ok(data
                .selected_points
                .iter()
                .cloned()
                .collect::<Box<[usize]>>())
        }
        None => Err(NoPossibleDispersion),
    }
}
