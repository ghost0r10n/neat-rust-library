use layer::NeuronLayer;
use layer::NeuronLayerType;
use network::Network;
use neuron::Neuron;
use neuron::NeuronType;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};
mod layer;
mod network;
mod neuron;
mod synapse;

pub fn main() {}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Set up the canvas and context

    let activation = |x| if x > 0.0 { x } else { 0.0 };
    let mut network = Network::new();
    network.add_layer(NeuronLayer::new(
        NeuronLayerType::Input,
        vec![
            Neuron::new(NeuronType::Input, 0.0, 0.0, activation),
            Neuron::new(NeuronType::Input, 0.0, 0.0, activation),
        ],
    ));
    network.add_layer(NeuronLayer::new(
        NeuronLayerType::Output,
        vec![Neuron::new(NeuronType::Output, 0.0, 0.0, activation)],
    ));
    network.add_random_neuron();
    network.add_random_neuron();
    network.add_random_neuron();
    network.add_random_neuron();
    network.add_random_neuron();
    network.add_random_neuron();
    network.add_random_synapses();
    network.add_random_synapses();
    network.add_random_synapses();
    network.add_random_synapses();
    network.add_random_synapses();
    network.add_random_synapses();
    network.add_random_synapses();
    network.add_random_synapses();
    let window = window().unwrap();
    let document = window.document().unwrap();

    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Start the animation loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        // Clear the canvas
        context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
        network.draw_network(&context);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}
