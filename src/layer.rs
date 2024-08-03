use crate::{neuron::Neuron, synapse::Synapse};

pub enum NeuronLayerType {
    Input,
    Hidden,
    Output,
}

pub struct NeuronLayer {
    pub type_: NeuronLayerType,
    pub neurons: Vec<Neuron>,
}

impl NeuronLayer {
    pub fn new(type_: NeuronLayerType, neurons: Vec<Neuron>) -> Self {
        Self { type_, neurons }
    }
}

pub struct LayerSynapse {
    pub synapses: Vec<Synapse>,
}
