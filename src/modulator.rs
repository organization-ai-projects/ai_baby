use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Modulator {
    pub dopamine: f32, // renforce l'apprentissage positif
    pub stress: f32,   // renforce l'inhibition
}

impl Modulator {
    pub fn new() -> Self {
        Self {
            dopamine: 0.0,
            stress: 0.0,
        }
    }

    pub fn decay(&mut self) {
        self.dopamine *= 0.90;
        self.stress *= 0.90;
    }
}
