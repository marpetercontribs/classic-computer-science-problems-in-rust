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

// Option 1 - which avoids treating the letters as Strings unnecessarily,
//            but makes the cartesian_product function parameters asymmetric
fn cartesian_product(first: &[String], second: &[char]) -> Vec<String> {
    let mut product: Vec<String> = Vec::new();
    for in_first in first {
        for in_second in second {
            let mut combination = in_first.clone();
            combination.push(*in_second);
            product.push(combination);
        }
    }
    product
}

const fn digit_mapping_1() -> [(char, &'static [char]); 10] {
    [
        ('1', &['1']),
        ('2', &['A', 'B', 'C']),
        ('3', &['D', 'E', 'F']),
        ('4', &['G', 'H', 'I']),
        ('5', &['J', 'K', 'L']),
        ('6', &['M', 'N', 'P']),
        ('7', &['P', 'Q', 'R', 'S']),
        ('8', &['T', 'U', 'V']),
        ('9', &['W', 'X', 'Y', 'Z']),
        ('0', &['0']),
    ]
}
// Using LazyLock here to be able to declare DIGIT_MAPPING_1 as const
// and avoid the overhead of reconstructing the HashMap on every function call.
const DIGIT_MAPPING_1: std::sync::LazyLock<HashMap<char, &'static [char]>> =
    std::sync::LazyLock::new(|| HashMap::from(digit_mapping_1()));

pub fn possible_mnemonics(phone_number: &str) -> Vec<String> {
    let mut mnemonics: Vec<String> = vec!["".to_string()];
    for digit in phone_number.chars() {
        if let Some(combo) = DIGIT_MAPPING_1.get(&digit) {
            mnemonics = cartesian_product(&mnemonics, combo);
        }
    }
    mnemonics
}

// Option 2 - Treat the letters as String slices similar to what is done in Kopec's book,
//            leads to a more symmetric cartesian_product function
fn cartesian_product2(first: &[String], second: &[&str]) -> Vec<String> {
    let mut product: Vec<String> = Vec::new();
    for in_first in first {
        for in_second in second {
            let mut combination = in_first.to_string();
            combination.push_str(in_second);
            product.push(combination);
        }
    }
    product
}

const fn digit_mapping_2() -> [(char, &'static [&'static str]); 10] {
    [
        ('1', &["1"]),
        ('2', &["A", "B", "C"]),
        ('3', &["D", "E", "F"]),
        ('4', &["G", "H", "I"]),
        ('5', &["J", "K", "L"]),
        ('6', &["M", "N", "P"]),
        ('7', &["P", "Q", "R", "S"]),
        ('8', &["T", "U", "V"]),
        ('9', &["W", "X", "Y", "Z"]),
        ('0', &["0"]),
    ]
}
// Using LazyLock here to be able to declare DIGIT_MAPPING_2 as const
// and avoid the overhead of reconstructing the HashMap on every function call.
const DIGIT_MAPPING_2: std::sync::LazyLock<HashMap<char, &[&str]>> =
    std::sync::LazyLock::new(|| HashMap::from(digit_mapping_2()));

pub fn possible_mnemonics2(phone_number: &str) -> Vec<String> {
    let mut mnemonics: Vec<String> = vec!["".to_string()];
    for digit in phone_number.chars() {
        if let Some(combo) = DIGIT_MAPPING_2.get(&digit) {
            mnemonics = cartesian_product2(&mnemonics, *combo);
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
    println!("{:#?}", mnemonics);
}
