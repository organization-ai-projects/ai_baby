// main.rs
// ------------------------------------------------------------            // Compagnon "bébé neuronal" spiking + chimie jouet.
// - Zéro pré-entraînement
// - Apprentissage en ligne
// - Fun / émergent / instable mais stabilisé
// ------------------------------------------------------------

mod brain;
mod modulator;
mod neuron;
mod synapse;
mod tokenizer;

use brain::Brain;
use modulator::Modulator;
use tokenizer::tokenize;

type Word = String;

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
