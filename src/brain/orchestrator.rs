// Fusion des fonctionnalités complètes de brain.rs

// Suppression des duplications et intégration des méthodes manquantes
// Uniformisation des commentaires et ajustements pour la modularité

use crate::brain::{inject_input, neurons, synapses};
use crate::composition::Composition;
use crate::neuron::Neuron;
use crate::neurotransmitter;
use crate::synapse::Synapse;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub type Word = String;

#[derive(Serialize, Deserialize)]
pub struct Brain {
    pub neurons: HashMap<Word, Neuron>,
    pub synapses: HashMap<(Word, Word), Synapse>,
    pub modulator: crate::Modulator,

    // hyperparams
    pub lr_exc: f32,
    pub lr_inh: f32,
    pub exc_max: f32,
    pub inh_max: f32,
    pub forget: f32,
}

impl Brain {
    pub fn new() -> Self {
        let mut brain = Self {
            neurons: HashMap::new(),
            synapses: HashMap::new(),
            modulator: crate::Modulator::new(),
            lr_exc: 0.05,
            lr_inh: 0.03,
            exc_max: 3.0,
            inh_max: 2.0,
            forget: 0.999,
        };

        // Stimulation initiale : ajouter des mots et connexions de base
        let initial_words = vec!["maman", "papa", "bébé", "amour", "calme", "joie"];
        for word in &initial_words {
            neurons::ensure_neuron(&mut brain.neurons, word, Composition::default()); // Composition par défaut
        }
        for i in 0..initial_words.len() {
            for j in i + 1..initial_words.len() {
                synapses::ensure_synapse(
                    &mut brain.synapses,
                    initial_words[i],
                    initial_words[j],
                    neurotransmitter::Neurotransmitter::Glutamate,
                );
            }
        }

        brain
    }

    pub fn run_spiking(&mut self, seed_words: &[Word], max_ticks: usize) -> Vec<HashSet<Word>> {
        println!(
            "[Brain] Début de run_spiking avec seed_words: {:?}, max_ticks: {}",
            seed_words, max_ticks
        );

        for i in 0..seed_words.len() {
            neurons::ensure_neuron(&mut self.neurons, &seed_words[i], Composition::default()); // Composition par défaut
            for j in i + 1..seed_words.len() {
                neurons::ensure_neuron(&mut self.neurons, &seed_words[j], Composition::default()); // Composition par défaut
                synapses::ensure_synapse(
                    &mut self.synapses,
                    &seed_words[i],
                    &seed_words[j],
                    neurotransmitter::Neurotransmitter::Glutamate,
                );
            }
        }

        inject_input(&mut self.neurons, seed_words);

        let mut history: Vec<HashSet<Word>> = Vec::with_capacity(max_ticks);
        let mut last_fired_count = 0;

        for t in 0..max_ticks {
            let mut fired: HashSet<Word> = HashSet::new();

            let keys: Vec<Word> = self.neurons.keys().cloned().collect();
            for w in keys.iter() {
                let refractory = self.neurons[w].refractory;
                if refractory > 0 {
                    continue;
                }

                let mut input_sum = 0.0;

                for ((a, b), syn) in self.synapses.iter() {
                    if a == w || b == w {
                        let other = if a == w { b } else { a };
                        let other_v = self.neurons.get(other).map(|n| n.v).unwrap_or(0.0);
                        let other_spike_like = if other_v > 0.9 { 1.0 } else { 0.0 };
                        if syn.is_excitatory() {
                            input_sum += other_spike_like * syn.strength;
                        } else if syn.is_inhibitory() {
                            input_sum -= other_spike_like * syn.strength;
                        }
                    }
                }

                let n = self.neurons.get_mut(w).unwrap();
                n.v = n.v * (1.0 - n.leak) + input_sum;
            }

            for w in self.neurons.keys().cloned().collect::<Vec<_>>() {
                let (th, refra, v_now) = {
                    let n = &self.neurons[&w];
                    (n.threshold, n.refractory, n.v)
                };

                if refra == 0 && v_now > th {
                    fired.insert(w.clone());
                    let n = self.neurons.get_mut(&w).unwrap();
                    n.v = 0.0;
                    n.refractory = 2;
                    n.fired_count += 1;
                }
            }

            for n in self.neurons.values_mut() {
                if n.refractory > 0 {
                    n.refractory -= 1;
                }
            }

            // Limiter les activations excessives
            for n in self.neurons.values_mut() {
                if n.v > 1.5 {
                    // Seuil pour limiter les activations excessives
                    n.v = 1.5;
                }
            }

            // Renforcer les connexions pertinentes
            for ((a, b), syn) in self.synapses.iter_mut() {
                if fired.contains(a) && fired.contains(b) {
                    syn.strength += self.lr_exc * self.modulator.dopamine.max(0.1);
                }
            }

            // Simulation des oscillations neuronales
            let oscillation_factor = (t as f32 * 0.1).sin();
            for n in self.neurons.values_mut() {
                n.v += oscillation_factor * 0.05; // Ajout d'une oscillation légère
            }

            // Ajustement dynamique des seuils des neurones
            for n in self.neurons.values_mut() {
                n.threshold += oscillation_factor * 0.01; // Ajustement dynamique
            }

            // Simulation des rythmes circadiens
            let circadian_factor = ((t as f32 / 100.0).sin() + 1.0) / 2.0; // Oscillation lente
            for n in self.neurons.values_mut() {
                n.v *= circadian_factor; // Modulation lente
            }

            // Ajustement des hormones en fonction des rythmes circadiens
            self.modulator.dopamine *= circadian_factor;
            self.modulator.serotonin *= circadian_factor;
            self.modulator.stress *= 1.0 - circadian_factor;

            println!(
                "[Brain] Modulateur après rythmes circadiens: dopamine = {:.2}, stress = {:.2}, serotonin = {:.2}, noradrenaline = {:.2}, endorphins = {:.2}",
                self.modulator.dopamine,
                self.modulator.stress,
                self.modulator.serotonin,
                self.modulator.noradrenaline,
                self.modulator.endorphins
            );

            println!("[Brain] Tick {}: fired neurons = {:?}", t, fired);
            history.push(fired.clone());

            // Arrêter si aucune activité n'est détectée
            if fired.is_empty() && last_fired_count == 0 {
                println!("[Brain] Aucun spike détecté, arrêt anticipé au tick {}", t);
                break;
            }

            last_fired_count = fired.len();
        }

        println!("[Brain] Fin de run_spiking");
        history
    }

    pub fn learn_from_spikes(&mut self, spikes_history: &[HashSet<Word>]) {
        println!(
            "[Brain] Début de learn_from_spikes avec spikes_history de longueur: {}",
            spikes_history.len()
        );

        for fired in spikes_history.iter() {
            let fired_vec: Vec<_> = fired.iter().cloned().collect();
            for i in 0..fired_vec.len() {
                for j in i + 1..fired_vec.len() {
                    let a = &fired_vec[i];
                    let b = &fired_vec[j];
                    synapses::ensure_synapse(
                        &mut self.synapses,
                        a,
                        b,
                        neurotransmitter::Neurotransmitter::Glutamate,
                    );
                    let key = (a.clone(), b.clone());
                    let syn = self.synapses.get_mut(&key).unwrap();

                    let dop = self.modulator.dopamine.max(0.1);
                    syn.strength += self.lr_exc * dop;
                }
            }
        }

        if self.modulator.stress > 0.2 {
            let last_fired = spikes_history.last().cloned().unwrap_or_default();
            let fired_vec: Vec<_> = last_fired.iter().cloned().collect();
            for i in 0..fired_vec.len() {
                for j in i + 1..fired_vec.len() {
                    let a = &fired_vec[i];
                    let b = &fired_vec[j];
                    let key = (a.clone(), b.clone());
                    if let Some(syn) = self.synapses.get_mut(&key) {
                        syn.strength += self.lr_inh * self.modulator.stress;
                    }
                }
            }
        }

        // Limiter les activations excessives
        for n in self.neurons.values_mut() {
            if n.v > 1.5 {
                // Seuil pour limiter les activations excessives
                n.v = 1.5;
            }
        }

        // Renforcer les connexions pertinentes
        for fired in spikes_history.iter() {
            let fired_vec: Vec<_> = fired.iter().cloned().collect();
            for i in 0..fired_vec.len() {
                for j in i + 1..fired_vec.len() {
                    let a = &fired_vec[i];
                    let b = &fired_vec[j];
                    let key = (a.clone(), b.clone());
                    if let Some(syn) = self.synapses.get_mut(&key) {
                        let modulation = self
                            .modulator
                            .modulate_neurotransmitter(syn.neurotransmitter.to_string().as_str());
                        syn.strength += self.lr_exc * modulation;
                        syn.strength = (syn.strength * self.forget).clamp(0.0, self.inh_max); // Décroissance de l'inhibition
                    }
                }
            }
        }

        for syn in self.synapses.values_mut() {
            syn.strength = (syn.strength * self.forget).clamp(0.0, self.exc_max);
            syn.strength = (syn.strength * self.forget).clamp(0.0, self.inh_max);
        }

        for n in self.neurons.values_mut() {
            if n.fired_count > 200 {
                n.threshold += 0.02;
                n.fired_count = 0;
            } else if n.fired_count == 0 && n.threshold > 0.6 {
                n.threshold -= 0.001;
            }
        }

        // Ajuster les seuils des neurones en fonction de leur type
        for n in self.neurons.values_mut() {
            let is_excitatory = n
                .composition
                .iter()
                .any(|m| m.role.to_lowercase() == "excitatory");
            if is_excitatory {
                n.threshold = (n.threshold * 0.98).max(0.8); // Réduction progressive
            } else {
                n.threshold = (n.threshold * 1.02).min(1.2); // Augmentation progressive
            }
        }

        println!("[Brain] Fin de learn_from_spikes");
        self.modulator.decay();
    }

    pub fn update_modulator_from_feedback(&mut self, user_input: &str) {
        println!("[Brain] Feedback utilisateur reçu: {}", user_input);

        if user_input.contains("bien") || user_input.contains("super") {
            self.modulator.dopamine += 0.1;
            self.modulator.endorphins += 0.05;
        } else if user_input.contains("stress") || user_input.contains("peur") {
            self.modulator.stress += 0.1;
            self.modulator.noradrenaline += 0.08;
        } else if user_input.contains("calme") || user_input.contains("zen") {
            self.modulator.serotonin += 0.1;
        }

        self.modulator.dopamine = self.modulator.dopamine.clamp(0.0, 1.0);
        self.modulator.stress = self.modulator.stress.clamp(0.0, 1.0);
        self.modulator.serotonin = self.modulator.serotonin.clamp(0.0, 1.0);
        self.modulator.noradrenaline = self.modulator.noradrenaline.clamp(0.0, 1.0);
        self.modulator.endorphins = self.modulator.endorphins.clamp(0.0, 1.0);

        println!("[Brain] Modulateur après mise à jour: {:?}", self.modulator);
    }

    pub fn generate_reply(&self, seed_words: &[Word]) -> String {
        println!(
            "[Brain] Génération de réponse à partir de seed_words: {:?}",
            seed_words
        );

        let mut reply_words = HashSet::new();

        for word in seed_words {
            if let Some(neuron) = self.neurons.get(word)
                && neuron.v > 0.5
            {
                for ((a, b), syn) in &self.synapses {
                    if a == word && syn.strength > 0.1 {
                        reply_words.insert(b.clone());
                    } else if b == word && syn.strength > 0.1 {
                        reply_words.insert(a.clone());
                    }
                }
            }
        }

        let mut reply: Vec<String> = reply_words.into_iter().collect();
        reply.sort();

        let response = reply.join(" ");
        println!("[Brain] Réponse générée: {}", response);
        response
    }

    pub fn apply_feedback(&mut self, _feedback: &str, _last_reply_words: &[Word]) {
        // Implémentation simplifiée
    }
}
