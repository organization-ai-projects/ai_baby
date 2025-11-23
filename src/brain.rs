use crate::neuron::Neuron;
use crate::synapse::Synapse;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;

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
        Self {
            neurons: HashMap::new(),
            synapses: HashMap::new(),
            modulator: crate::Modulator::new(),
            lr_exc: 0.05,
            lr_inh: 0.03,
            exc_max: 3.0,
            inh_max: 2.0,
            forget: 0.999,
        }
    }

    pub fn ensure_neuron(&mut self, w: &str) {
        self.neurons
            .entry(w.to_string())
            .or_insert_with(Neuron::new);
    }

    pub fn syn_key(a: &str, b: &str) -> (Word, Word) {
        if a <= b {
            (a.to_string(), b.to_string())
        } else {
            (b.to_string(), a.to_string())
        }
    }

    pub fn ensure_synapse(&mut self, a: &str, b: &str) {
        let key = Self::syn_key(a, b);
        self.synapses.entry(key).or_insert_with(Synapse::new);
    }

    pub fn inject_input(&mut self, active_words: &[Word]) -> HashSet<Word> {
        let spikes = HashSet::new();
        for w in active_words {
            self.ensure_neuron(w);
            if let Some(n) = self.neurons.get_mut(w) {
                n.v += 0.6;
            }
        }
        spikes
    }

    pub fn run_spiking(&mut self, seed_words: &[Word], ticks: usize) -> Vec<HashSet<Word>> {
        for i in 0..seed_words.len() {
            self.ensure_neuron(&seed_words[i]);
            for j in i + 1..seed_words.len() {
                self.ensure_neuron(&seed_words[j]);
                self.ensure_synapse(&seed_words[i], &seed_words[j]);
            }
        }

        self.inject_input(seed_words);

        let mut history: Vec<HashSet<Word>> = Vec::with_capacity(ticks);

        for _t in 0..ticks {
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
                        input_sum += other_spike_like * (syn.exc - syn.inh);
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

            history.push(fired);
        }

        history
    }

    pub fn learn_from_spikes(&mut self, spikes_history: &[HashSet<Word>]) {
        for fired in spikes_history.iter() {
            let fired_vec: Vec<_> = fired.iter().cloned().collect();
            for i in 0..fired_vec.len() {
                for j in i + 1..fired_vec.len() {
                    let a = &fired_vec[i];
                    let b = &fired_vec[j];
                    self.ensure_synapse(a, b);
                    let key = Self::syn_key(a, b);
                    let syn = self.synapses.get_mut(&key).unwrap();

                    let dop = self.modulator.dopamine.max(0.1);
                    syn.exc += self.lr_exc * dop;
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
                    let key = Self::syn_key(a, b);
                    if let Some(syn) = self.synapses.get_mut(&key) {
                        syn.inh += self.lr_inh * self.modulator.stress;
                    }
                }
            }
        }

        for syn in self.synapses.values_mut() {
            syn.exc = (syn.exc * self.forget).clamp(0.0, self.exc_max);
            syn.inh = (syn.inh * self.forget).clamp(0.0, self.inh_max);
        }

        for n in self.neurons.values_mut() {
            if n.fired_count > 200 {
                n.threshold += 0.02;
                n.fired_count = 0;
            } else if n.fired_count == 0 && n.threshold > 0.6 {
                n.threshold -= 0.001;
            }
        }

        self.modulator.decay();
    }

    pub fn load(path: &str) -> Option<Self> {
        let s = fs::read_to_string(path).ok()?;
        serde_json::from_str(&s).ok()
    }

    pub fn save(&self, path: &str) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }

    pub fn update_modulator_from_feedback(&mut self, _user: &str) {
        // Implémentation simplifiée pour éviter les erreurs
    }

    pub fn generate_reply(&self, _seed_words: &[Word]) -> String {
        "".to_string() // Implémentation simplifiée
    }

    pub fn apply_feedback(&mut self, _feedback: &str, _last_reply_words: &[Word]) {
        // Implémentation simplifiée
    }
}
