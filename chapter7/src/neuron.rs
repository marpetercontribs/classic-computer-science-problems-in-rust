// neuron.rs
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

use crate::utils;
use std::rc::Rc;

pub struct Neuron {
    pub weights: Vec<f64>,
    pub learning_rate: f64,
    pub output_cache: f64,
    pub delta: f64,
    pub activation_function: Rc<dyn Fn(f64) -> f64>,
    pub activation_function_derivative: Rc<dyn Fn(f64) -> f64>,
}

impl Neuron {
    pub fn new(
        weights: Vec<f64>,
        learning_rate: f64,
        activation_function: Rc<dyn Fn(f64) -> f64>,
        activation_function_derivative: Rc<dyn Fn(f64) -> f64>,
    ) -> Self {
        Neuron {
            weights,
            learning_rate,
            output_cache: 0.0,
            delta: 0.0,
            activation_function: Rc::clone(&activation_function),
            activation_function_derivative: Rc::clone(&activation_function_derivative),
        }
    }
    pub fn output(&mut self, input: &[f64]) -> f64 {
        self.output_cache = utils::dot_product(input, &self.weights);
        (self.activation_function)(self.output_cache)
    }
}
