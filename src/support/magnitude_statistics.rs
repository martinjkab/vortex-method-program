use std::fmt::Debug;

use cgmath::{InnerSpace, Vector3};

pub struct MagnitudeStatistics {
    pub max: f32,
    pub min: f32,
    pub mean: f32,
    pub median: f32,
}

impl Debug for MagnitudeStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MagnitudeStatistics")
            .field("max", &self.max)
            .field("min", &self.min)
            .field("mean", &self.mean)
            .field("median", &self.median)
            .finish()
    }
}

impl MagnitudeStatistics {
    pub fn from_vectors(vectors: &Vec<Vector3<f32>>) -> Self {
        let mut magnitudes = vectors
            .iter()
            .map(|vector| vector.magnitude().abs())
            .collect::<Vec<f32>>();

        magnitudes.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let max = magnitudes.last().unwrap();
        let min = magnitudes.first().unwrap();
        let mean = magnitudes.iter().sum::<f32>() / magnitudes.len() as f32;
        let median = magnitudes[magnitudes.len() / 2];

        Self {
            max: *max,
            min: *min,
            mean,
            median,
        }
    }
}
