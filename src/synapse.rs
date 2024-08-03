pub struct Synapse {
    pub weight: f64,
    pub layer_from: u8,
    pub index_from: f64,
    pub layer_to: u8,
    pub index_to: f64,
}

impl Synapse {
    pub fn new(weight: f64, layer_from: u8, index_from: f64, layer_to: u8, index_to: f64) -> Self {
        Self {
            weight,
            layer_from,
            index_from,
            layer_to,
            index_to,
        }
    }
}
