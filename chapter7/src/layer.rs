// layer.rs
// Adapted From Classic Computer Science Problems in Python/Java Chapter 7
// Copyright 2023 Markus Peter
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::neuron::Neuron;
use crate::utils::dot_product;
use rand::Rng;
use std::rc::Rc;

pub struct Layer {
    previous_layer: Box<Option<Layer>>, // Box due to recursive definition
    neurons: Vec<Neuron>,
    output_cache: Vec<f64>,
}

impl Layer {
    pub fn new(
        previous_layer: Option<Layer>,
        num_neurons: usize,
        learning_rate: f64,
        activation_function: Rc<dyn Fn(f64) -> f64>,
        activation_function_derivative: Rc<dyn Fn(f64) -> f64>,
    ) -> Self {
        let mut neurons = Vec::<Neuron>::new();
        for _ in 0..num_neurons {
            let randon_weights: Vec<f64> = previous_layer.as_ref().map_or_else(
                || vec![0.0; num_neurons],
                |layer| {
                    let mut rng = rand::thread_rng();
                    (0..layer.neurons.len()).map(|_| rng.gen::<f64>()).collect()
                },
            );
            neurons.push(Neuron::new(
                randon_weights,
                learning_rate,
                Rc::clone(&activation_function),
                Rc::clone(&activation_function_derivative),
            ));
        }
        Layer {
            previous_layer: Box::new(previous_layer),
            neurons,
            output_cache: vec![0.0; num_neurons],
        }
    }

    pub fn outputs(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        self.output_cache = if self.previous_layer.is_none() {
            inputs
        } else {
            self.neurons
                .iter_mut()
                .map(|neuron| neuron.output(&inputs))
                .collect()
        };
        self.output_cache.clone()
    }
    // should only be called on output layer
    pub fn calculate_deltas_for_output_layer(&mut self, expected: Vec<f64>){
        for n in 0..self.neurons.len() {
            self.neurons[n].delta = (self.neurons[n].activation_function_derivative)(self.neurons[n].output_cache) * (expected[n] - self.output_cache[n]);
        }
    }
    // should not be called on output layer
    pub fn calculate_deltas_for_hidden_layer(&mut self, next_layer: &Layer) {
        for (index, neuron) in self.neurons.iter_mut().enumerate() {
            let next_weights: Vec<f64> = next_layer.neurons.iter().map(|n| n.weights[index]).collect();
            let next_deltas: Vec<f64> = next_layer.neurons.iter().map(|n| n.delta).collect();
            let sum_weights_and_deltas: f64 = dot_product(&next_weights, &next_deltas);
            neuron.delta = (neuron.activation_function_derivative)(neuron.output_cache) * sum_weights_and_deltas;
        }
    }        
}
