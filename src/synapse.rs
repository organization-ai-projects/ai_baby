use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Synapse {
    pub exc: f32, // excitateur
    pub inh: f32, // inhibiteur
}

impl Synapse {
    pub fn new() -> Self {
        Self { exc: 0.0, inh: 0.0 }
    }
}
