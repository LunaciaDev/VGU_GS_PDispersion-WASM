use std::{cell::RefCell, iter::zip, rc::Rc};

use crate::Point;

#[derive(Debug)]
struct PointData {
    location: Box<[Point]>,
    distance_matrix: Box<[Box<[f32]>]>,
}

impl PointData {
    fn new(location_count: usize) -> Self {
        Self {
            location: vec![Point::default(); location_count].into_boxed_slice(),
            distance_matrix: vec![vec![0.; location_count].into_boxed_slice(); location_count]
                .into_boxed_slice(),
        }
    }
}

struct AdjacencyMatrix {
    data: Box<[PointVec]>,
}

impl AdjacencyMatrix {
    fn new(location_count: u8, point_data: &PointData, neighbour_distance: f32) -> Self {
        let mut data = vec![PointVec::new(location_count, false); location_count as usize];

        for (index, row) in data.iter_mut().enumerate() {
            for (point, distance) in point_data.distance_matrix[index].iter().enumerate() {
                if *distance <= neighbour_distance {
                    row.insert(point);
                }
            }
        }

        Self {
            data: data.into_boxed_slice(),
        }
    }
}

#[derive(Clone)]
struct PointVec {
    data: [u64; 3],
    true_count: u8,
}

impl PointVec {
    fn new(location_count: u8, fill: bool) -> Self {
        assert!(location_count < 64 * 3);

        if fill {
            let mut ret = Self {
                data: [u64::MAX; 3],
                true_count: location_count,
            };

            for sector in (location_count as usize / 64 + 1)..3 {
                ret.data[sector] = 0;
            }
            ret.data[location_count as usize / 64] &= 2_u64.pow(location_count as u32 % 64) - 1;

            ret
        } else {
            Self {
                data: [0; 3],
                true_count: 0,
            }
        }
    }

    fn reset(&mut self, location_count: u8, fill: bool) {
        if fill {
            for sector in self.data.iter_mut() {
                *sector = u64::MAX;
            }

            for sector in (location_count as usize / 64 + 1)..3 {
                self.data[sector] = 0;
            }
            self.data[location_count as usize / 64] &= 2_u64.pow(location_count as u32 % 64) - 1;

            self.true_count = location_count;
        } else {
            for sector in self.data.iter_mut() {
                *sector = 0;
            }
            self.true_count = 0;
        }
    }

    fn len(&self) -> u8 {
        self.true_count
    }

    fn copy(&mut self, copy_src: &PointVec) {
        for sector in 0..self.data.len() {
            self.data[sector] = copy_src.data[sector];
        }
        self.true_count = copy_src.true_count;
    }

    fn next(&self) -> Option<usize> {
        self.data
            .iter()
            .position(|value| *value != 0)
            .map(|index| 64 * index + self.data[index].trailing_zeros() as usize)
    }

    fn subtract_and_copy(&mut self, lhs: &PointVec, rhs: &PointVec) {
        self.true_count = 0;

        for index in 0..self.data.len() {
            self.data[index] = lhs.data[index] & !rhs.data[index];
            self.true_count += self.data[index].count_ones() as u8;
        }
    }

    fn remove(&mut self, index: usize) {
        let sector = index / 64;
        let bitmask = 1 << (index % 64);

        self.true_count -= (self.data[sector] & bitmask == bitmask) as u8;
        self.data[sector] &= !bitmask;
    }

    fn insert_and_copy(&mut self, lhs: &PointVec, index: usize) {
        let target_sector = index / 64;
        let bitmask = 1 << (index % 64);

        for sector in 0..self.data.len() {
            if sector == target_sector {
                self.true_count = lhs.true_count + (lhs.data[sector] & bitmask != bitmask) as u8;
                self.data[sector] = lhs.data[sector] | bitmask;
                continue;
            }

            self.data[sector] = lhs.data[sector]
        }
    }

    fn insert(&mut self, index: usize) {
        let sector = index / 64;
        let bitmask = 1 << (index % 64);

        self.true_count += (self.data[sector] & bitmask != bitmask) as u8;
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
            // copy out the u64 ref
            let mut value = *value;

            while value > 0 {
                result.push(index * 64 + value.trailing_zeros() as usize);
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
    fn new(location_count: u8) -> Self {
        Self {
            selected_points: PointVec::new(location_count, false),
            remaining_points: PointVec::new(location_count, true),
        }
    }

    fn reset(&mut self, location_count: u8) {
        self.selected_points.reset(location_count, false);
        self.remaining_points.reset(location_count, true);
    }
}

struct SolveStack {
    data: Box<[Rc<RefCell<SolveData>>]>,
    idx: u8,
}

impl SolveStack {
    fn new(max_search_depth: u8, location_count: u8) -> Self {
        Self {
            data: {
                let mut v = Vec::with_capacity(max_search_depth as usize);
                (0..max_search_depth)
                    .for_each(|_| v.push(Rc::new(RefCell::new(SolveData::new(location_count)))));
                v.into_boxed_slice()
            },
            idx: 0,
        }
    }

    fn alloc(&mut self) -> Rc<RefCell<SolveData>> {
        assert!((self.idx as usize) < self.data.len());

        let ret = self.data[self.idx as usize].clone();
        self.idx += 1;

        ret
    }

    fn dealloc(&mut self) {
        self.idx -= 1;
    }

    fn reset(&mut self, location_count: u8) {
        self.idx = 0;
        self.data[0].borrow_mut().reset(location_count);
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
    solve_data: Rc<RefCell<SolveData>>,
    stack: &mut SolveStack,
    adjacency_matrix: &AdjacencyMatrix,
    select_size: u8,
) -> Option<Rc<RefCell<SolveData>>> {
    let mut mut_solve_data = solve_data.borrow_mut();

    if mut_solve_data.selected_points.len() >= select_size {
        drop(mut_solve_data);
        return Some(solve_data);
    }

    if mut_solve_data.remaining_points.len() < select_size - mut_solve_data.selected_points.len() {
        return None;
    }

    // pick next point
    let point = mut_solve_data
        .remaining_points
        .next()
        .expect("At least one point remaining");
    mut_solve_data.remaining_points.remove(point);

    // Pick this point
    let new_data = stack.alloc();
    let mut mut_new_data = new_data.borrow_mut();

    mut_new_data
        .selected_points
        .insert_and_copy(&mut_solve_data.selected_points, point);
    mut_new_data.remaining_points.subtract_and_copy(
        &mut_solve_data.remaining_points,
        &adjacency_matrix.data[point],
    );

    drop(mut_new_data);

    if let Some(result) = search(new_data, stack, adjacency_matrix, select_size) {
        return Some(result);
    };

    stack.dealloc();
    drop(mut_solve_data);

    // Do not pick this point
    search(solve_data, stack, adjacency_matrix, select_size)
}

pub fn naive_solver(input_data: &[Point], placements: u8) -> Option<Box<[usize]>> {
    let input_size = input_data.len() as u8;
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
    let mut best_result = PointVec::new(input_size, false);
    let mut stack = SolveStack::new(input_size, input_size);

    while left_index < right_index {
        stack.reset(input_size);
        let target = left_index.midpoint(right_index);

        match search(
            stack.alloc(),
            &mut stack,
            &AdjacencyMatrix::new(input_size, &point_data, possible_point_distance[target]),
            placements,
        ) {
            Some(result) => {
                left_index = target + 1;
                best_result.copy(&result.borrow().selected_points);
            }
            None => right_index = target,
        }
    }

    if best_result.true_count == 0 {
        None
    } else {
        Some(best_result.into())
    }
}
