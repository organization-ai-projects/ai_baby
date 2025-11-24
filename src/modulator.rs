use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Modulator {
    pub dopamine: f32,      // renforce l'apprentissage positif
    pub stress: f32,        // renforce l'inhibition
    pub serotonin: f32,     // stabilise l'humeur
    pub noradrenaline: f32, // augmente l'attention
    pub endorphins: f32,    // réduit le stress et augmente le plaisir
}

impl Modulator {
    pub fn new() -> Self {
        Self {
            dopamine: 0.1,       // Niveau basal de dopamine
            stress: 0.05,        // Niveau basal de stress
            serotonin: 0.1,      // Niveau basal de sérotonine
            noradrenaline: 0.05, // Niveau basal de noradrénaline
            endorphins: 0.1,     // Niveau basal d'endorphines
        }
    }

    pub fn decay(&mut self) {
        println!(
            "[Modulator] Avant décroissance: dopamine = {:.2}, stress = {:.2}, serotonin = {:.2}, noradrenaline = {:.2}, endorphins = {:.2}",
            self.dopamine, self.stress, self.serotonin, self.noradrenaline, self.endorphins
        );

        self.dopamine = (self.dopamine * 0.90).max(0.1); // Ne descend pas en dessous du niveau basal
        self.stress = (self.stress * 0.90).max(0.05);
        self.serotonin = (self.serotonin * 0.95).max(0.1);
        self.noradrenaline = (self.noradrenaline * 0.92).max(0.05);
        self.endorphins = (self.endorphins * 0.93).max(0.1);

        println!(
            "[Modulator] Après décroissance: dopamine = {:.2}, stress = {:.2}, serotonin = {:.2}, noradrenaline = {:.2}, endorphins = {:.2}",
            self.dopamine, self.stress, self.serotonin, self.noradrenaline, self.endorphins
        );
    }

    pub fn modulate_neurotransmitter(&self, neurotransmitter: &str) -> f32 {
        match neurotransmitter {
            "glutamate" => self.dopamine * 1.2, // Dopamine renforce glutamate
            "GABA" => self.stress * 1.5,        // Stress renforce GABA
            "acetylcholine" => self.serotonin * 1.1, // Sérotonine favorise l'acétylcholine
            _ => 1.0,                           // Par défaut, pas de modulation
        }
    }

    pub fn adjust_hormones_for_neurotransmitter(&mut self, neurotransmitter: &str) {
        match neurotransmitter {
            "glutamate" => self.dopamine += 0.05,
            "GABA" => self.stress += 0.05,
            "acetylcholine" => self.serotonin += 0.05,
            _ => (),
        }

        // Limiter les valeurs pour éviter les débordements
        self.dopamine = self.dopamine.clamp(0.0, 1.0);
        self.stress = self.stress.clamp(0.0, 1.0);
        self.serotonin = self.serotonin.clamp(0.0, 1.0);
    }

    pub fn detect_neurotransmitter(text: &str) -> Option<String> {
        let lowered = text.to_lowercase();
        if lowered.contains("glutamate") {
            Some("glutamate".to_string())
        } else if lowered.contains("gaba") {
            Some("GABA".to_string())
        } else if lowered.contains("acetylcholine") {
            Some("acetylcholine".to_string())
        } else {
            None
        }
    }
}
