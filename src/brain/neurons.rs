use crate::composition::Composition;
use crate::neuron::Neuron;
use std::collections::HashMap;

pub type Word = String;

pub fn ensure_neuron(neurons: &mut HashMap<Word, Neuron>, w: &str, composition: Composition) {
    neurons
        .entry(w.to_string())
        .or_insert_with(|| Neuron::new(1.0, composition));
}

pub fn inject_input(
    neurons: &mut HashMap<Word, Neuron>,
    active_words: &[Word],
) -> std::collections::HashSet<Word> {
    let spikes = std::collections::HashSet::new();
    for w in active_words {
        ensure_neuron(neurons, w, Composition::new(vec![])); // Composition par défaut
        if let Some(n) = neurons.get_mut(w) {
            n.v += 0.6;
        }
    }
    spikes
}
