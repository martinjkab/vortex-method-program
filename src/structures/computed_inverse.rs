use nalgebra::DMatrix;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ComputedInverse {
    pub original_checksum: u32,
    pub inverse: DMatrix<f32>,
}
