#![no_main]
#[macro_use]
mod utils;
use math;
pub mod fea;
pub mod fea_output;

use wasm_bindgen::prelude::*;
pub use fea::Lin2DStaticModel;

#[wasm_bindgen]
pub fn main() {
    fea::test::square();
    unsafe {
        fea_output::VERTICES[0] = 0.0;
        fea_output::VERTICES[1] = 0.0;
        fea_output::VERTICES[2] = 1.0;
        fea_output::VERTICES[3] = 0.0;
        fea_output::VERTICES[4] = 0.0;
        fea_output::VERTICES[5] = 1.0;
   }
}

#[wasm_bindgen]
pub fn init_fea(e: f64, nu: f64, g: f64) -> Lin2DStaticModel {
    let elasticity = fea::plane_stress_matrix(e, nu, g);
    Lin2DStaticModel::new(elasticity)
}
