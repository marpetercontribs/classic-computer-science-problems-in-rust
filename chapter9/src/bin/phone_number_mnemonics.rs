// phone_number_mnemonics.rs
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
use std::collections::HashMap;
use std::io;

// TODO: remove asymmetry in parameters?
fn cartesian_product(first: &[String], second: &[char]) -> Vec<String> {
    let mut product: Vec<String> = Vec::<>::new();
    for in_first in first {
        for in_second in second {
            let mut combination = in_first.clone();
            combination.push( *in_second );
            product.push(combination);
        }
    }
    product
} 

pub fn possible_mnemonics(phone_number: &str) -> Vec<String> {
    // We define DIGIT_MAPPING here, instead of as a const/static outside the function
    // because it is only used here anyway, and declaring it as a const outside would
    // require using once_cell::sync::Lazy::new( || ... ) because vec! cannot be used
    // directly in const expressions.
    let DIGIT_MAPPING: HashMap<char, Vec<char>> =
        HashMap::from([
            ('1', vec!['1']),
            ('2', vec!['A', 'B', 'C']),
            ('3', vec!['D', 'E', 'F']),
            ('4', vec!['G', 'H', 'I']),
            ('5', vec!['J', 'K', 'L']),
            ('6', vec!['M', 'N', 'P']),
            ('7', vec!['P', 'Q', 'R', 'S']),
            ('8', vec!['T', 'U', 'V']),
            ('9', vec!['W', 'X', 'Y', 'Z']),
            ('0', vec!['0']),
        ]);
    let mut mnemonics: Vec<String> = vec!["".to_string()];
    for digit in phone_number.chars() {
        if let Some(combo) = DIGIT_MAPPING.get(&digit) {
            mnemonics = cartesian_product(&mnemonics, combo);
        }
    }
    mnemonics
}

fn main() {
    println!("Enter a phone number:");

    let mut phone_number = String::new();
    io::stdin()
        .read_line(&mut phone_number)
        .expect("Error reading your input.");

    let mnemonics = possible_mnemonics(&phone_number);

    println!("Here are the potential mnemonics:");
    println!("{:?}", mnemonics);
}
