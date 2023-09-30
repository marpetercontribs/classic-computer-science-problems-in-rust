// network.rs
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

use crate::layer::Layer;
use std::cell::RefCell;
use std::iter::zip;
use std::rc::Rc;

pub struct Network {
    layers: Vec<Rc<RefCell<Layer>>>, // "ownership" is shared with the layers (linked list), and the network must be able to mutate the layers
}

impl Network {
    pub fn new(
        layer_structure: Vec<usize>,
        learning_rate: f64,
        activation_function: Rc<dyn Fn(f64) -> f64>,
        activation_function_derivative: Rc<dyn Fn(f64) -> f64>,
    ) -> Self {
        if layer_structure.len() < 3 {
            panic!("Error: There should be at least 3 layers (1 input, 1 hidden, 1 output)");
        }
        let mut layers = Vec::<Rc<RefCell<Layer>>>::new();
        let input_layer = Layer::new(
            None,
            layer_structure[0],
            learning_rate,
            Rc::clone(&activation_function),
            Rc::clone(&activation_function_derivative),
        );
        layers.push(Rc::new(RefCell::new(input_layer)));
        for (index, num_neurons) in layer_structure[1..].iter().enumerate() {
            let next_layer = Layer::new(
                Some(Rc::clone(&layers[index])),
                *num_neurons,
                learning_rate,
                Rc::clone(&activation_function),
                Rc::clone(&activation_function_derivative),
            );
            layers.push(Rc::new(RefCell::new(next_layer)));
        }
        Network { layers }
    }
    pub fn outputs(&self, input: &[f64]) -> Vec<f64> {
        let mut inputs: Vec<f64> = input.to_vec();
        for layer in self.layers.iter() {
            // using self.layers.iter().fold( ... ) did not work because the intermediate "inputs" don't seem to live long enough
            inputs = layer.borrow_mut().outputs(&inputs);
        }
        inputs
    }
    pub fn backpropagate(&self, expected: &[f64]) {
        let last_layer_no = self.layers.len() - 1;
        self.layers[last_layer_no]
            .borrow_mut()
            .calculate_deltas_for_output_layer(expected);
        for layer_no in (0..(last_layer_no - 1)).rev() {
            self.layers[last_layer_no]
                .borrow_mut()
                .calculate_deltas_for_hidden_layer(&self.layers[layer_no + 1].borrow());
        }
    }
    pub fn update_weights(&self) {
        for layer in self.layers[1..].iter() {
            for neuron in layer.borrow_mut().neurons.iter_mut() {
                for weight in 0..neuron.weights.len() {
                    neuron.weights[weight] += neuron.learning_rate
                        * (layer
                            .borrow() // deref Rc and borrow content of RefCell
                            .previous_layer
                            .as_ref() // to avoid move
                            .unwrap() // the Option!
                            .borrow() // deref Rc and borrow content of RefCell
                            .output_cache[weight])
                        * neuron.delta
                }
            }
        }
    }
    pub fn train(&self, inputs: Vec<Vec<f64>>, expecteds: Vec<Vec<f64>>) {
        for (run, xs) in inputs.iter().enumerate() {
            let ys = &expecteds[run];
            let _ = self.outputs(xs);
            self.backpropagate(ys);
            self.update_weights();
        }
    }
    pub fn validate<T: std::cmp::PartialEq>(
        &self,
        inputs: Vec<Vec<f64>>,
        expecteds: Vec<T>,
        interpret_output: &dyn Fn(&[f64]) -> T,
    ) -> (usize, usize, f64) {
        let mut correct: usize = 0;
        let len = inputs.len();
        for (input, expected) in zip(inputs, expecteds) {
            let result: T = interpret_output(&self.outputs(&input));
            if result == expected {
                correct += 1;
            }
        }
        let percentage: f64 = (correct as f64) / (len as f64);
        (correct, len, percentage)
    }
}
