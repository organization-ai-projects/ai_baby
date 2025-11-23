use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Neuron {
    pub v: f32,           // potentiel
    pub threshold: f32,   // seuil
    pub leak: f32,        // fuite
    pub refractory: u8,   // ticks restants de repos
    pub fired_count: u32, // pour stats
}

impl Neuron {
    pub fn new() -> Self {
        Self {
            v: 0.0,
            threshold: 1.0,
            leak: 0.08,
            refractory: 0,
            fired_count: 0,
        }
    }
}
