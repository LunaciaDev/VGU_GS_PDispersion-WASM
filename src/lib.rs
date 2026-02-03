use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::core::p_solver;

mod core;

#[wasm_bindgen]
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
pub enum SolveError {
    MalformedInput,
    EmptyInput,
    Unsolvable
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}

#[wasm_bindgen]
pub fn solve_p_dispersion(
    input_array: JsValue,
    placements: u32,
) -> Result<Box<[usize]>, SolveError> {
    let input_array: Box<[Point]> = match serde_wasm_bindgen::from_value(input_array) {
        Ok(val) => val,
        Err(_) => {
            return Err(SolveError::MalformedInput);
        }
    };
    if input_array.is_empty() {
        return Err(SolveError::EmptyInput);
    }

    if let Some(result) = p_solver(input_array, placements) {
        return Ok(result);
    }

    Err(SolveError::Unsolvable)
}