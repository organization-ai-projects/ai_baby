// main.rs
// ------------------------------------------------------------            // Compagnon "bébé neuronal" spiking + chimie jouet.
// - Zéro pré-entraînement
// - Apprentissage en ligne
// - Fun / émergent / instable mais stabilisé
// ------------------------------------------------------------

mod brain;
mod composition;
mod modulator;
mod neuron;
mod neurotransmitter;
mod persist;
mod synapse;
mod tokenizer;
use crate::composition::Composition;
use persist::{load, save};

use brain::Brain;
use brain::{ensure_neuron, ensure_synapse};
use modulator::Modulator;
use tokenizer::tokenize;

type Word = String;

fn main() {
    let brain_path = "baby_brain.json";

    let mut brain = load(brain_path).unwrap_or_else(|| {
        let mut b = Brain::new();
        let initial_words = vec!["maman", "papa", "bébé", "amour", "calme", "joie"];
        for word in &initial_words {
            ensure_neuron(&mut b.neurons, word, Composition::default()); // Composition par défaut
        }
        for i in 0..initial_words.len() {
            for j in i + 1..initial_words.len() {
                ensure_synapse(
                    &mut b.synapses,
                    initial_words[i],
                    initial_words[j],
                    neurotransmitter::Neurotransmitter::Glutamate,
                );
            }
        }
        b
    });

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
            save(&brain, brain_path);
            println!("Lui > (je me suis sauvegardé.)");
            continue;
        }

        // -------- feedback implicite sur ce que tu dis --------
        brain.update_modulator_from_feedback(&input);

        println!("[Main] Modulateur après feedback: {:?}", brain.modulator);

        // -------- spiking / apprentissage --------
        let words = tokenize(&input);

        // On inclut les mots de sa dernière réponse dans l'apprentissage
        // pour renforcer ce qui a été "utilisé" en conversation.
        let mut seed_words = words.clone();
        seed_words.extend(last_reply_words.clone());

        let spikes_history = brain.run_spiking(&seed_words, 1); // Un seul tick par interaction
        brain.learn_from_spikes(&spikes_history);

        println!(
            "[Main] État du cerveau après apprentissage: {} neurones, {} synapses",
            brain.neurons.len(),
            brain.synapses.len()
        );

        // -------- génération réponse --------
        let reply = brain.generate_reply(&words);
        println!("Lui > {}", reply);

        // Inclure la réponse dans l'apprentissage
        let reply_words = tokenize(&reply);
        let spikes_history = brain.run_spiking(&reply_words, 1); // Tick pour la réponse
        brain.learn_from_spikes(&spikes_history);

        last_reply_words = reply_words;

        println!(
            "[Main] État du cerveau après intégration de la réponse: {} neurones, {} synapses",
            brain.neurons.len(),
            brain.synapses.len()
        );

        // -------- feedback explicite --------
        brain.apply_feedback(&input, &last_reply_words);

        // -------- autosave léger --------
        save(&brain, brain_path);

        let neurotransmitter = Modulator::detect_neurotransmitter(&input);
        if let Some(nt) = neurotransmitter {
            println!("[Main] Neurotransmetteur détecté : {}", nt);
            brain.modulator.adjust_hormones_for_neurotransmitter(&nt);
        }
    }
}
