use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

mod exact;
mod greedy;
mod random;

#[wasm_bindgen]
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub(crate) fn set(&mut self, point: &Point) {
        self.x = point.x;
        self.y = point.y;
    }

    pub(crate) fn get_distance(&self, point: &Point) -> f32 {
        ((self.x - point.x).powi(2) + (self.y - point.y).powi(2)).sqrt()
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum SolveError {
    MalformedInput,
    EmptyInput,
    Unsolvable,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}

#[wasm_bindgen]
pub fn solve_exact(input_array: JsValue, placements: u8) -> Result<Box<[usize]>, SolveError> {
    let input_array: Box<[Point]> = match serde_wasm_bindgen::from_value(input_array) {
        Ok(val) => val,
        Err(_) => {
            return Err(SolveError::MalformedInput);
        }
    };
    if input_array.is_empty() {
        return Err(SolveError::EmptyInput);
    }
    if input_array.len() < placements as usize {
        return Err(SolveError::Unsolvable);
    }
    if input_array.len() > 192 {
        return Err(SolveError::MalformedInput);
    }

    if let Some(result) = exact::solver(&input_array, placements) {
        return Ok(result);
    }

    Err(SolveError::Unsolvable)
}

// This is garbage but will have to be suffice
#[wasm_bindgen]
pub fn solve_random(input_array: JsValue, placements: u8) -> Result<Box<[usize]>, SolveError> {
    let input_array: Box<[Point]> = match serde_wasm_bindgen::from_value(input_array) {
        Ok(val) => val,
        Err(_) => {
            return Err(SolveError::MalformedInput);
        }
    };
    if input_array.is_empty() {
        return Err(SolveError::EmptyInput);
    }
    if input_array.len() < placements as usize {
        return Err(SolveError::Unsolvable);
    }
    if input_array.len() > 192 {
        return Err(SolveError::MalformedInput);
    }

    if let Some(result) = random::solver(&input_array, placements) {
        return Ok(result);
    }

    Err(SolveError::Unsolvable)
}

#[wasm_bindgen]
pub fn solve_greedy(input_array: JsValue, placements: u8) -> Result<Box<[usize]>, SolveError> {
    let input_array: Box<[Point]> = match serde_wasm_bindgen::from_value(input_array) {
        Ok(val) => val,
        Err(_) => {
            return Err(SolveError::MalformedInput);
        }
    };
    if input_array.is_empty() {
        return Err(SolveError::EmptyInput);
    }
    if input_array.len() < placements as usize {
        return Err(SolveError::Unsolvable);
    }
    if input_array.len() > 192 {
        return Err(SolveError::MalformedInput);
    }

    if let Some(result) = greedy::solver(&input_array, placements) {
        return Ok(result);
    }

    Err(SolveError::Unsolvable)
}

pub fn solve_p_dispersion_rs(
    input_array: &[Point],
    placements: u8,
) -> Result<Box<[usize]>, SolveError> {
    if input_array.is_empty() {
        return Err(SolveError::EmptyInput);
    }
    if input_array.len() < placements as usize {
        return Err(SolveError::Unsolvable);
    }
    if input_array.len() > 192 {
        return Err(SolveError::MalformedInput);
    }

    if let Some(result) = greedy::solver(input_array, placements) {
        return Ok(result);
    }

    Err(SolveError::Unsolvable)
}
