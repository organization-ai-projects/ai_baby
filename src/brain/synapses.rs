use crate::neurotransmitter::Neurotransmitter;
use crate::synapse::Synapse;
use std::collections::HashMap;

pub type Word = String;

fn syn_key(a: &str, b: &str) -> (Word, Word) {
    if a <= b {
        (a.to_string(), b.to_string())
    } else {
        (b.to_string(), a.to_string())
    }
}

pub fn ensure_synapse(
    synapses: &mut HashMap<(Word, Word), Synapse>,
    a: &str,
    b: &str,
    neurotransmitter: Neurotransmitter,
) {
    let key = syn_key(a, b);
    synapses
        .entry(key)
        .or_insert_with(|| Synapse::new(neurotransmitter, 0.5)); // Ajout d'une force par défaut
}
