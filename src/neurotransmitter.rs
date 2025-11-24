use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Neurotransmitter {
    Glutamate, // Excitateur
    Gaba,      // Inhibiteur
    Dopamine,  // Modulateur
}

impl std::fmt::Display for Neurotransmitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Neurotransmitter::Glutamate => "Glutamate",
            Neurotransmitter::Gaba => "GABA",
            Neurotransmitter::Dopamine => "Dopamine",
        };
        write!(f, "{}", name)
    }
}
