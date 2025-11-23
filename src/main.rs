// main.rs
//
// Dépendances Cargo.toml :
//
// [dependencies]
// rand = "0.8"
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
//
// ------------------------------------------------------------
// Compagnon "bébé neuronal" spiking + chimie jouet.
// - Zéro pré-entraînement
// - Apprentissage en ligne
// - Fun / émergent / instable mais stabilisé
// ------------------------------------------------------------

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;

type Word = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Neuron {
    v: f32,           // potentiel
    threshold: f32,   // seuil
    leak: f32,        // fuite
    refractory: u8,   // ticks restants de repos
    fired_count: u32, // pour stats
}

impl Neuron {
    fn new() -> Self {
        Self {
            v: 0.0,
            threshold: 1.0,
            leak: 0.08,
            refractory: 0,
            fired_count: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Synapse {
    exc: f32, // excitateur
    inh: f32, // inhibiteur
}

impl Synapse {
    fn new() -> Self {
        Self { exc: 0.0, inh: 0.0 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Modulator {
    dopamine: f32, // renforce l'apprentissage positif
    stress: f32,   // renforce l'inhibition
}

impl Modulator {
    fn new() -> Self {
        Self {
            dopamine: 0.0,
            stress: 0.0,
        }
    }

    fn decay(&mut self) {
        self.dopamine *= 0.90;
        self.stress *= 0.90;
    }
}

#[derive(Serialize, Deserialize)]
struct Brain {
    neurons: HashMap<Word, Neuron>,
    synapses: HashMap<(Word, Word), Synapse>,
    modulator: Modulator,

    // hyperparams
    lr_exc: f32,
    lr_inh: f32,
    exc_max: f32,
    inh_max: f32,
    forget: f32,
}

impl Brain {
    fn new() -> Self {
        Self {
            neurons: HashMap::new(),
            synapses: HashMap::new(),
            modulator: Modulator::new(),
            lr_exc: 0.05,
            lr_inh: 0.03,
            exc_max: 3.0,
            inh_max: 2.0,
            forget: 0.999,
        }
    }

    fn ensure_neuron(&mut self, w: &str) {
        self.neurons
            .entry(w.to_string())
            .or_insert_with(Neuron::new);
    }

    fn syn_key(a: &str, b: &str) -> (Word, Word) {
        if a <= b {
            (a.to_string(), b.to_string())
        } else {
            (b.to_string(), a.to_string())
        }
    }

    fn ensure_synapse(&mut self, a: &str, b: &str) {
        let key = Self::syn_key(a, b);
        self.synapses.entry(key).or_insert_with(Synapse::new);
    }

    /// Injecte des spikes initiaux sur les mots d'entrée.
    fn inject_input(&mut self, active_words: &[Word]) -> HashSet<Word> {
        let spikes = HashSet::new();
        for w in active_words {
            self.ensure_neuron(w);
            // petite excitation initiale
            if let Some(n) = self.neurons.get_mut(w) {
                n.v += 0.6;
            }
        }
        spikes
    }

    /// Tourne le réseau spiking quelques ticks.
    /// Retourne un historique de spikes (par tick).
    fn run_spiking(&mut self, seed_words: &[Word], ticks: usize) -> Vec<HashSet<Word>> {
        // On s'assure d'avoir neurones + synapses entre mots vus ensemble
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

            // 1) intégration (LIF)
            let keys: Vec<Word> = self.neurons.keys().cloned().collect();
            for w in keys.iter() {
                let refractory = self.neurons[w].refractory;
                if refractory > 0 {
                    continue;
                }

                let mut input_sum = 0.0;

                // somme des spikes des voisins (ici: on approxime via poids globaux
                // en prenant les synapses incidentes au neurone)
                for ((a, b), syn) in self.synapses.iter() {
                    if a == w || b == w {
                        let other = if a == w { b } else { a };
                        // Donc on "devine" si l'autre a un potentiel déjà élevé
                        // (ça simule une activité interne)
                        let other_v = self.neurons.get(other).map(|n| n.v).unwrap_or(0.0);
                        let other_spike_like = if other_v > 0.9 { 1.0 } else { 0.0 };
                        input_sum += other_spike_like * (syn.exc - syn.inh);
                    }
                }

                let n = self.neurons.get_mut(w).unwrap();
                n.v = n.v * (1.0 - n.leak) + input_sum;
            }

            // 2) déclenchement spike + reset
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

            // 3) refractory countdown
            for n in self.neurons.values_mut() {
                if n.refractory > 0 {
                    n.refractory -= 1;
                }
            }

            history.push(fired);
        }

        history
    }

    /// Apprentissage hebbien spiking + neuromodulation.
    fn learn_from_spikes(&mut self, spikes_history: &[HashSet<Word>]) {
        // renforce les co-spikes
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

        // inhibition via stress (anti-association)
        if self.modulator.stress > 0.2 {
            // on inhibe légèrement les synapses les plus actives récentes
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

        // stabilisation : clamp + oubli lent
        for syn in self.synapses.values_mut() {
            syn.exc = (syn.exc * self.forget).clamp(0.0, self.exc_max);
            syn.inh = (syn.inh * self.forget).clamp(0.0, self.inh_max);
        }

        // seuil adaptatif léger (auto-stabilité)
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

    /// Détecte feedback implicite dans le texte utilisateur.
    fn update_modulator_from_feedback(&mut self, user: &str) {
        let s = user.to_lowercase();
        let positive = [
            "mdr", "haha", "lol", "bien", "bravo", "cool", "yes", "ouai", "oui",
        ];
        let negative = ["non", "nul", "mauvais", "stop", "ta gueule", "bof", "nope"];

        let mut pos = 0;
        let mut neg = 0;

        for p in positive {
            if s.contains(p) {
                pos += 1;
            }
        }
        for n in negative {
            if s.contains(n) {
                neg += 1;
            }
        }

        if pos > neg {
            self.modulator.dopamine += 0.7;
            self.modulator.stress *= 0.6;
        } else if neg > pos {
            self.modulator.stress += 0.7;
            self.modulator.dopamine *= 0.6;
        } else {
            // neutre → légère baisse
            self.modulator.dopamine *= 0.95;
            self.modulator.stress *= 0.95;
        }
    }

    /// Met à jour les synapses en fonction du feedback explicite.
    fn apply_feedback(&mut self, feedback: &str, last_reply_words: &[Word]) {
        let positive = ["oui", "bravo", "bien", "super", "cool", "mdr", "haha"];
        let negative = ["non", "faux", "nul", "stop", "mauvais", "bof"];

        if positive.iter().any(|p| feedback.contains(p)) {
            // Renforce les connexions des mots de la dernière réponse
            for i in 0..last_reply_words.len() {
                for j in i + 1..last_reply_words.len() {
                    let a = &last_reply_words[i];
                    let b = &last_reply_words[j];
                    self.ensure_synapse(a, b);
                    let key = Self::syn_key(a, b);
                    if let Some(syn) = self.synapses.get_mut(&key) {
                        syn.exc += self.lr_exc * 2.0; // Double renforcement
                    }
                }
            }
            self.modulator.dopamine += 1.0; // Boost de dopamine
        } else if negative.iter().any(|n| feedback.contains(n)) {
            // Inhibe les connexions des mots de la dernière réponse
            for i in 0..last_reply_words.len() {
                for j in i + 1..last_reply_words.len() {
                    let a = &last_reply_words[i];
                    let b = &last_reply_words[j];
                    let key = Self::syn_key(a, b);
                    if let Some(syn) = self.synapses.get_mut(&key) {
                        syn.inh += self.lr_inh * 2.0; // Double inhibition
                    }
                }
            }
            self.modulator.stress += 1.0; // Boost de stress
        }
    }

    /// Choisit des concepts dominants à partir des synapses associées aux seed words.
    fn pick_concepts(&self, seed_words: &[Word], k: usize) -> Vec<Word> {
        let mut scores: HashMap<Word, f32> = HashMap::new();

        for seed in seed_words {
            for ((a, b), syn) in self.synapses.iter() {
                if a == seed {
                    *scores.entry(b.clone()).or_insert(0.0) += syn.exc - syn.inh;
                }
                if b == seed {
                    *scores.entry(a.clone()).or_insert(0.0) += syn.exc - syn.inh;
                }
            }
        }

        // enlever les seeds elles-mêmes
        for s in seed_words {
            scores.remove(s);
        }

        let mut scored: Vec<_> = scores.into_iter().collect();
        scored.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap_or(std::cmp::Ordering::Equal));

        scored.into_iter().take(k).map(|x| x.0).collect()
    }

    /// Génère une réponse basée uniquement sur les concepts activés et les mots entendus.
    fn generate_reply(&self, seed_words: &[Word]) -> String {
        let mut rng = thread_rng();
        let concepts = self.pick_concepts(seed_words, 3);

        // Si pas assez de concepts, le "bébé" babille en répétant ou en mélangeant les mots entendus.
        if concepts.is_empty() {
            if seed_words.is_empty() {
                return "ba... ba...".to_string();
            }
            let babble = seed_words.to_vec();
            return babble.choose(&mut rng).unwrap_or(&"ba".to_string()).clone();
        }

        // Réponse émergente basée sur les concepts activés.
        let mut response = String::new();
        for concept in concepts {
            response.push_str(&format!("{} ", concept));
        }

        response.trim().to_string()
    }

    fn save(&self, path: &str) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }

    fn load(path: &str) -> Option<Self> {
        let s = fs::read_to_string(path).ok()?;
        serde_json::from_str(&s).ok()
    }
}

// Tokenisation volontairement simple et marrante.
fn tokenize(text: &str) -> Vec<Word> {
    let lowered = text.to_lowercase();
    lowered
        .split(|c: char| !c.is_alphanumeric() && c != '\'' && c != '_')
        .filter(|w| !w.trim().is_empty())
        .map(|w| w.to_string())
        .collect()
}

fn main() {
    let brain_path = "baby_brain.json";

    let mut brain = Brain::load(brain_path).unwrap_or_else(Brain::new);

    println!("🍼 Bébé neuronal réveillé.");
    println!("Parle-lui. Ctrl+C pour arrêter.");
    println!("(il apprend tout, même la merde 😈)\n");

    let mut last_reply_words: Vec<Word> = vec![];

    loop {
        // -------- input user --------
        use std::io::{self, Write};
        print!("Toi > ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }
        let input = input.trim().to_string();
        if input.is_empty() {
            continue;
        }

        if input == "/reset" {
            brain = Brain::new();
            println!("Lui > … (reset total)");
            continue;
        }
        if input == "/save" {
            brain.save(brain_path);
            println!("Lui > (je me suis sauvegardé.)");
            continue;
        }

        // -------- feedback implicite sur ce que tu dis --------
        brain.update_modulator_from_feedback(&input);

        // -------- spiking / apprentissage --------
        let words = tokenize(&input);

        // On inclut les mots de sa dernière réponse dans l'apprentissage
        // pour renforcer ce qui a été "utilisé" en conversation.
        let mut seed_words = words.clone();
        seed_words.extend(last_reply_words.clone());

        let spikes_history = brain.run_spiking(&seed_words, 30);
        brain.learn_from_spikes(&spikes_history);

        // -------- génération réponse --------
        let reply = brain.generate_reply(&words);
        last_reply_words = tokenize(&reply);

        println!("Lui > {}", reply);

        // -------- feedback explicite --------
        brain.apply_feedback(&input, &last_reply_words);

        // -------- autosave léger --------
        brain.save(brain_path);
    }
}
