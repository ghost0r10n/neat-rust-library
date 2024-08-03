use core::f64;

use rand::{thread_rng, Rng};
use wasm_bindgen::JsValue;
use web_sys::{console, js_sys::Math::random, CanvasRenderingContext2d};

use crate::{
    layer::{NeuronLayer, NeuronLayerType},
    neuron::{self, Neuron, NeuronType},
    synapse::Synapse,
};

pub struct Network {
    pub layers: Vec<NeuronLayer>,
    pub synapses: Vec<Synapse>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            synapses: Vec::new(),
        }
    }

    pub fn add_layer(&mut self, layer: NeuronLayer) {
        self.layers.push(layer);
    }
    pub fn add_synapses(&mut self, synapse: Synapse) {
        self.synapses.push(synapse);
    }
    pub fn add_random_synapses(&mut self) {
        let layer_from = rand::thread_rng().gen_range(0..self.layers.len() - 1) as usize;
        let layer_to = rand::thread_rng().gen_range(1..self.layers.len()) as usize;
        console::log_1(&format!("Layer from: {}, Layer to: {}", layer_from as u8, layer_to).into());
        let from = rand::thread_rng().gen_range(0..self.layers[layer_from].neurons.len()) as usize;
        let to = rand::thread_rng().gen_range(0..self.layers[layer_to].neurons.len()) as usize;
        let weight = rand::thread_rng().gen_range(-1.0..1.0);
        console::log_1(
            &format!(
                "Layer from: {}, Layer to: {}, From: {}, To: {}, Weight: {}",
                layer_from as u8, layer_to as u8, from as u8, to as u8, weight
            )
            .into(),
        );
        self.add_synapses(Synapse::new(
            weight,
            layer_from as u8,
            from as f64,
            layer_to as u8,
            to as f64,
        ));
    }

    pub fn add_random_neuron(&mut self) {
        let decision_maker = thread_rng().gen_range(0.0..1.0) as f64;
        console::log_1(&format!("Decision maker: {}", decision_maker).into());

        let neuron = Neuron::new(NeuronType::Hidden, 0.0, random(), |x| {
            if x > 0.0 {
                x
            } else {
                0.0
            }
        });
        console::log_1(&format!("Neuron created").into());
        let hidden_layer_selection = thread_rng().gen_range(1..self.layers.len());
        console::log_1(
            &format!(
                "Decision maker: {}, Hidden layer selection: {}",
                decision_maker, hidden_layer_selection
            )
            .into(),
        );
        if decision_maker < 0.5 || self.layers.len() == 2 {
            self.layers.insert(
                hidden_layer_selection,
                NeuronLayer::new(NeuronLayerType::Hidden, vec![neuron.clone()]),
            );
        } else {
            self.layers[hidden_layer_selection - if hidden_layer_selection == 1 { 0 } else { 1 }]
                .neurons
                .push(neuron.clone());
        }
    }

    pub fn draw_network(&self, context: &CanvasRenderingContext2d) {
        context.set_line_width(2.0);
        context.set_stroke_style(&JsValue::from_str("black"));
        console::log_1(&format!("Current time: {}", self.synapses.len()).into());
        for c in 0..self.synapses.len() {
            context.begin_path();
            context.move_to(
                self.synapses[c].layer_from as f64 * (70.0) + 10.0,
                self.synapses[c].index_from * 70.0 + 10.0,
            );
            context.line_to(
                (70.0 * (self.synapses[c].layer_to as f64)) + 10.0,
                ((self.synapses[c].index_to) * (70.0)) + 10.0,
            );
            context.stroke();
        }
        context.set_fill_style(&JsValue::from_str("black"));
        let mut increasexd: f64 = 0.0;
        for c in 0..self.layers.len() {
            let mut increaseyd: f64 = 0.0;
            match self.layers[c].type_ {
                NeuronLayerType::Input | NeuronLayerType::Hidden | NeuronLayerType::Output => {
                    for i in 0..self.layers[c].neurons.len() {
                        context.set_fill_style(&JsValue::from_str(
                            match self.layers[c].neurons[i].type_ {
                                neuron::NeuronType::Input => "green",
                                neuron::NeuronType::Hidden => "blue",
                                neuron::NeuronType::Output => "red",
                            },
                        ));
                        context.fill_rect(increasexd, increaseyd, 20.0, 20.0);
                        context.set_fill_style(&JsValue::from_str("black"));
                        let _ = context.fill_text(
                            &self.layers[c].neurons[i].value.to_string(),
                            increasexd + 7.0,
                            increaseyd + 15.0,
                        );
                        increaseyd += 70.0;
                    }
                }
            }

            increasexd += 70.0;
        }
    }
}
