use std::fmt::format;

pub struct Network {
    layers: Vec<Layer>,
}

struct Layer {
    neurons: Vec<Neuron>,
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}
pub struct LayerTopology {
    pub input_neurons: usize,
    pub output_neurons: usize,
}

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs: Vec<f32>, layer| layer.propagate(inputs))
    }

    pub fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn random(neurons_per_layer: Vec<usize>) -> Self {
        todo!()
    }
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    pub fn random(input_neurons: usize, output_neurons: usize) -> Self {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(input_neurons))
            .collect();
        Self { neurons }
    }
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        (self.bias + output).max(0.0)
    }

    pub fn random(output_size: usize) -> Self {
        let bias = todo!();

        let weights = (0..output_size).map(|_| todo!()).collect();
        Self { bias, weights }
    }
}
