// dna_search.rs
// Adapted From Classic Computer Science Problems in Python/Java Chapter 2
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
use std::cmp::Ordering;
#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum Nucleotide {
    A,
    C,
    G,
    T,
}

type Codon = (Nucleotide, Nucleotide, Nucleotide);
type Gene = Vec<Codon>;

// Is there a more elegant / idiomatic way for the following?
fn byte_to_nucleotide(byte: &u8) -> Nucleotide {
    match byte {
        b'A' => Nucleotide::A,
        b'C' => Nucleotide::C,
        b'G' => Nucleotide::G,
        b'T' => Nucleotide::T,
        _ => panic!("Unknown nucleotide!"),
    }
}

fn string_to_gene(gene_string: &str) -> Gene {
    let mut gene = Gene::new();
    let gene_bytes = gene_string.as_bytes();
    for index in (0..gene_bytes.len()).step_by(3) {
        if index + 2 < gene_bytes.len() {
            let codon = (
                byte_to_nucleotide(&gene_bytes[index]),
                byte_to_nucleotide(&gene_bytes[index + 1]),
                byte_to_nucleotide(&gene_bytes[index + 2]),
            );
            gene.push(codon);
        }
    }
    gene
}

fn linear_contains(gene: &Gene, key_codon: &Codon) -> bool {
    // The following using the built-in Iterator method any would work as well, of course:
    // gene.iter().any( |codon| codon == key_codon )
    for codon in gene {
        if codon == key_codon {
            return true;
        }
    }
    false
}

fn binary_contains(gene: &Gene, key_codon: &Codon) -> bool {
    let mut low: usize = 0;
    let mut high: usize = gene.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        match gene[mid].cmp(key_codon) {
            Ordering::Greater => high = mid - 1,
            Ordering::Less => low = mid + 1,
            Ordering::Equal => return true,
        }
    }
    false
    // The following using the built-in vec method binary_search would work as well, of course:
    // match gene.binary_search(key_codon) {
    //     Ok(_) => true,
    //     Err(_) => false
    // }
}

fn main() {
    let gene_str = "ACGTGGCTCTCTAACGTACGTACGTACGGGGTTTATATATACCCTAGGACTCCCTTT";
    let mut gene = string_to_gene(gene_str);
    let acg = (Nucleotide::A, Nucleotide::C, Nucleotide::G);
    let gat = (Nucleotide::G, Nucleotide::A, Nucleotide::T);
    println!("{}", linear_contains(&gene, &acg));
    println!("{}", linear_contains(&gene, &gat));
    // The following using the built-in vec method contains would work as well, of course:
    // println!("{}", gene.contains(&acg));
    // println!("{}", gene.contains(&gat));

    gene.sort();
    println!("{}", binary_contains(&gene, &acg));
    println!("{}", binary_contains(&gene, &gat));
}
