#![allow(static_mut_refs)]
use wasm_bindgen::prelude::*;

const FACES: usize = 32;

/// Vertices for the WebGL canvas
pub static mut VERTICES: [f32; 3 * 2 * FACES] = [0.0; 3 * 2 * FACES];
pub static mut DISPLACEMENTS: [f32; 3 * 2 * FACES] = [0.0; 3 * 2 * FACES];
pub static mut STRESSES: [f32; 3 * 3 * FACES] = [0.0; 3 * 3 * FACES];
pub static mut FORCES: [f32; 3 * 2 * FACES] = [0.0; 3 * 2 * FACES];

#[wasm_bindgen]
pub fn max_faces() -> usize {
    FACES
}

#[wasm_bindgen]
pub fn get_vertices() -> *const f32 {
    unsafe { VERTICES.as_ptr() }
}

#[wasm_bindgen]
pub fn get_vertices_len() -> usize {
    unsafe { VERTICES.len() }
}

#[wasm_bindgen]
pub fn get_displacements() -> *const f32 {
    unsafe { DISPLACEMENTS.as_ptr() }
}

#[wasm_bindgen]
pub fn get_displacements_len() -> usize {
    unsafe { DISPLACEMENTS.len() }
}

#[wasm_bindgen]
pub fn get_stresses() -> *const f32 {
    unsafe { STRESSES.as_ptr() }
}

#[wasm_bindgen]
pub fn get_stresses_len() -> usize {
    unsafe { STRESSES.len() }
}

#[wasm_bindgen]
pub fn get_forces() -> *const f32 {
    unsafe { FORCES.as_ptr() }
}
