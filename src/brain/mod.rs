pub mod neurons;
pub mod orchestrator;
pub mod synapses;

pub use neurons::{ensure_neuron, inject_input};
pub use orchestrator::Brain;
pub use synapses::ensure_synapse;
