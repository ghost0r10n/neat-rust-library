#[derive(Clone)]
pub enum NeuronType {
    Input,
    Hidden,
    Output,
}

#[derive(Clone)]
pub struct Neuron {
    pub type_: NeuronType,
    pub value: f64,
    pub bias: f64,
    pub activation: fn(f64) -> f64,
}

impl Neuron {
    pub fn new(type_: NeuronType, value: f64, bias: f64, activation: fn(f64) -> f64) -> Self {
        Self {
            type_,
            value,
            bias,
            activation,
        }
    }
    // Activate the neuron
}
