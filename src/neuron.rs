use crate::composition::Composition;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Neuron {
    pub potential: f32,           // Potentiel électrique
    pub threshold: f32,           // Seuil d'activation
    pub refractory: u32,          // Période réfractaire
    pub composition: Composition, // Composition chimique sous forme de molécules
    pub v: f32,                   // Potentiel membranaire
    pub fired_count: u32,         // Compteur de spikes
    pub leak: f32,                // Taux de fuite
}

impl Neuron {
    pub fn new(threshold: f32, composition: Composition) -> Self {
        Self {
            potential: 0.0,
            threshold,
            refractory: 0,
            composition,
            v: 0.0,
            fired_count: 0,
            leak: 0.1,
        }
    }
}
