// knapsack.rs
// Adapted From Classic Computer Science Problems in Python/Java Chapter 9
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

#[derive(Clone, Debug)]
pub struct Item {
    name: String,
    weight: usize,
    value: f64,
}

pub fn knapsack(items: &[Item], max_capacity: usize) -> Vec<Item> {
    // build up dynamic programming table
    let mut table: Vec<Vec<f64>> = vec![vec![0.0; max_capacity + 1]; items.len() + 1];
    for (index, item) in items.iter().enumerate() {
        for capacity in 1..=max_capacity {
            let previous_items_value = table[index][capacity];
            if capacity >= item.weight {
                // item fits in knapsack
                let value_freeing_weight_for_item = table[index][capacity - item.weight];
                table[index + 1][capacity] =
                    previous_items_value.max(value_freeing_weight_for_item + item.value);
            } else {
                // no room for this item
                table[index + 1][capacity] = previous_items_value;
            }
        }
    }
    let mut capacity = max_capacity;
    let mut solution: Vec<Item> = Vec::new();
    for index in (1..=items.len()).rev() {
        // work backwards
        // was this item used?
        if table[index - 1][capacity] != table[index][capacity] {
            solution.push(items[index - 1].clone());
            // if the item was used, remove its weight
            capacity -= items[index - 1].weight;
        }
    }
    solution
}

// Implement Display for Item to get rid of clippy's "field `name` is never read" warning
impl std::fmt::Display for Item {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            formatter,
            "Item {} (weight: {}, value: {})",
            self.name, self.weight, self.value
        )
    }
}
// to see the output when testing, use
// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let items = vec![
            Item {
                name: "television".to_string(),
                weight: 50,
                value: 500_f64,
            },
            Item {
                name: "candlesticks".to_string(),
                weight: 2,
                value: 300_f64,
            },
            Item {
                name: "stereo".to_string(),
                weight: 35,
                value: 400_f64,
            },
            Item {
                name: "laptop".to_string(),
                weight: 3,
                value: 1000_f64,
            },
            Item {
                name: "food".to_string(),
                weight: 15,
                value: 50_f64,
            },
            Item {
                name: "clothing".to_string(),
                weight: 20,
                value: 800_f64,
            },
            Item {
                name: "jewelry".to_string(),
                weight: 1,
                value: 4000_f64,
            },
            Item {
                name: "books".to_string(),
                weight: 100,
                value: 300_f64,
            },
            Item {
                name: "printer".to_string(),
                weight: 18,
                value: 30_f64,
            },
            Item {
                name: "refrigerator".to_string(),
                weight: 200,
                value: 700_f64,
            },
            Item {
                name: "painting".to_string(),
                weight: 10,
                value: 1000_f64,
            },
        ];
        println!("{:#?}", knapsack(&items, 75));
    }
}
