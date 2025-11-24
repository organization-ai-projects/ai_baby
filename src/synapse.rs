use crate::neurotransmitter::Neurotransmitter;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Synapse {
    pub strength: f32,                      // Force de la connexion
    pub neurotransmitter: Neurotransmitter, // Type de neurotransmetteur
}

impl Synapse {
    pub fn new(neurotransmitter: Neurotransmitter, strength: f32) -> Self {
        Self {
            strength,
            neurotransmitter,
        }
    }

    pub fn is_excitatory(&self) -> bool {
        matches!(self.neurotransmitter, Neurotransmitter::Glutamate)
    }

    pub fn is_inhibitory(&self) -> bool {
        matches!(self.neurotransmitter, Neurotransmitter::Gaba)
    }
}
