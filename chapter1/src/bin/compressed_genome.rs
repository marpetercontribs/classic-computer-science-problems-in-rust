// compressed_genome.rs
// Adapted From Classic Computer Science Problems in Python/Java Chapter 1
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
use std::mem;

struct CompressedGenome {
    bits: Vec<bool>,
}

impl CompressedGenome {
    fn new(genome: &str) -> Self {
        CompressedGenome {
            bits: Self::compress(genome),
        }
    }
    fn compress(genome: &str) -> Vec<bool> {
        let upper_genome = genome.to_ascii_uppercase();
        // imperative version
        let mut result = Vec::new();
        for nucleotide in upper_genome.chars() {
            match nucleotide {
                'A' => {
                    result.push(false);
                    result.push(false);
                },
                'C' => {
                    result.push(false);
                    result.push(true);
                },
                'G' => {
                    result.push(true);
                    result.push(false);
                },
                'T' => {
                    result.push(true);
                    result.push(true);
                },
                _ => panic!("Invalid nucleotide:{}", nucleotide),
            }
        }
        result
        // more functional version
        // upper_genome
        //     .chars()
        //     .fold(Vec::new(), |mut result, nucleotide| {
        //         match nucleotide {
        //             'A' => {
        //                 result.push(false);
        //                 result.push(false);
        //             }
        //             'C' => {
        //                 result.push(false);
        //                 result.push(true);
        //             }
        //             'G' => {
        //                 result.push(true);
        //                 result.push(false);
        //             }
        //             'T' => {
        //                 result.push(true);
        //                 result.push(true);
        //             }
        //             _ => panic!("Invalid nucleotide:{}", nucleotide),
        //         }
        //         result
        //     })
    }
    fn decompress(&self) -> String {
        // imperative version
        let mut genome = String::new();
        for index in 0..(self.bits.len() / 2) {
            let nucleotide: u8 = (if self.bits[2 * index] { 1 } else { 0 }) * 2
                + (if self.bits[2 * index + 1] { 1 } else { 0 });
            match nucleotide {
                0b00 => genome.push('A'),
                0b01 => genome.push('C'),
                0b10 => genome.push('G'),
                0b11 => genome.push('T'),
                _ => panic!("Invalid nucleotide:{}", nucleotide),
            }
        }
        genome
        // more functional version
        // (0..(self.bits.len() / 2))
        //     .map(|index| {
        //         (if self.bits[2 * index] { 1 } else { 0 }) * 2
        //             + (if self.bits[2 * index + 1] { 1 } else { 0 })
        //     })
        //     .fold(String::new(), |mut genome, nucleotide| {
        //         match nucleotide {
        //             0b00 => genome.push('A'),
        //             0b01 => genome.push('C'),
        //             0b10 => genome.push('G'),
        //             0b11 => genome.push('T'),
        //             _ => panic!("Invalid nucleotide:{}", nucleotide),
        //         }
        //         genome
        //     })
    }
}

fn main() {
    let original = String::from(
        "TAGGGATTAACCGTTATATATATATAGCCATGGATCGATTATATAGGGATTAACCGTTATATATATATAGCCATGGATCGATTATA",
    );
    println!(
        "original is {} bytes",
        mem::size_of_val(&'A') * original.len()
    );
    let compressed = CompressedGenome::new(&original);
    println!("compressed is {} bytes", mem::size_of_val(&compressed.bits));
    println!("{}", compressed.decompress());
    println!(
        "original and decompressed are the same: {}",
        original.eq(&compressed.decompress())
    );
}
