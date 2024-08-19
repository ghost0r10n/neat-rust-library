use rand::Rng;

#[derive(Debug, Clone)]
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
    pub fn mutate(&mut self) {
        let mutation: f64 = rand::thread_rng().gen_range(-0.5..0.5);
        self.weight = self.weight + mutation;
    }
}
