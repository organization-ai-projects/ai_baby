use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Molecule {
    pub name: String,       // Nom de la molécule (ex: Glucose, ATP)
    pub concentration: f32, // Concentration en moles/L
    pub role: String,       // Rôle biologique (ex: Énergie, Signalisation)
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Composition {
    pub molecules: Vec<Molecule>, // Liste des molécules présentes
}

impl Composition {
    pub fn new(molecules: Vec<Molecule>) -> Self {
        Self { molecules }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, Molecule> {
        self.molecules.iter()
    }
}
