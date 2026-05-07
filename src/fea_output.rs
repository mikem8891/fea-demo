#![allow(static_mut_refs)]
use std::sync::Mutex;

use wasm_bindgen::prelude::*;

const FACES: usize = 64;
const VERTICES: usize = FACES;

pub static OUTPUTS: Mutex<Outputs> = Mutex::new(Outputs::new());

pub struct Outputs {
    pub nodes: OutputNodes,
    pub elements: OutputElements,
}

impl Outputs {
    const fn new() -> Outputs {
        Outputs { 
            nodes: OutputNodes::new(),
            elements: OutputElements::new(),
        }
    }
}

pub struct OutputNodes {
    pub len: usize,
    pub positions: [f32; 2 * VERTICES],
    pub known: [u8; 2 * VERTICES],
    pub displacements: [f32; 2 * VERTICES],
    pub forces: [f32; 2 * VERTICES],
}

impl OutputNodes {
    const fn new() -> OutputNodes {
        OutputNodes { 
            len: 0,
            positions: [0.0; 2 * FACES], 
            known: [0; 2 * VERTICES],
            displacements: [0.0; 2 * FACES], 
            forces: [0.0; 2 * FACES] 
        }
    }
}

pub struct OutputElements {
    pub positions: [f32; 3 * 2 * FACES],
    pub displacements: [f32; 3 * 2 * FACES],
    pub stresses: [f32; 3 * 3 * FACES],
}

impl OutputElements {
    const fn new() -> OutputElements {
        OutputElements { 
            positions: [0.0; 3 * 2 * FACES], 
            displacements: [0.0; 3 * 2 * FACES], 
            stresses: [0.0; 3 * 3 * FACES], 
        }
    }
}

#[wasm_bindgen]
pub fn max_faces() -> usize {
    FACES
}
#[wasm_bindgen]
pub fn max_vertices() -> usize {
    VERTICES
}

#[wasm_bindgen]
pub fn get_nodes_len() -> usize {
    OUTPUTS.lock().unwrap().nodes.len
}

#[wasm_bindgen]
pub fn get_node_positions() -> *const f32 {
    OUTPUTS.lock().unwrap().nodes.positions.as_ptr()
}

#[wasm_bindgen]
pub fn get_node_displacements() -> *const f32 {
    OUTPUTS.lock().unwrap().nodes.displacements.as_ptr()
}

#[wasm_bindgen]
pub fn get_element_positions() -> *const f32 {
    OUTPUTS.lock().unwrap().elements.positions.as_ptr()
}

#[wasm_bindgen]
pub fn get_element_displacements() -> *const f32 {
    OUTPUTS.lock().unwrap().elements.displacements.as_ptr()
}

#[wasm_bindgen]
pub fn get_element_stresses() -> *const f32 {
    OUTPUTS.lock().unwrap().elements.stresses.as_ptr()
}

