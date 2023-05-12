// unbreakable_encryption.rs
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

use rand::RngCore;
use std::str;

fn random_key(length: usize) -> Vec<u8> {
    let mut dummy = vec![0; length];
    rand::thread_rng().fill_bytes(&mut dummy);
    dummy
}

fn encrypt(original: &str) -> (Vec<u8>, Vec<u8>) {
    let original_bytes = original.as_bytes().to_vec();
    let key = random_key(original_bytes.len());
    let mut encrypted = Vec::<u8>::new();
    for index in 0..original_bytes.len() {
        encrypted.push(key[index] ^ original_bytes[index]);
    }
    (encrypted, key)
}

fn decrypt(key1: &Vec<u8>, key2: &Vec<u8>) -> String {
    let mut decrypted = Vec::<u8>::new();
    for index in 0..key1.len() {
        decrypted.push(key1[index] ^ key2[index]);
    }
    let temp = str::from_utf8(&decrypted).unwrap();
    String::from(temp)
}

fn main() {
    let (key1, key2) = encrypt(&String::from("One Time Pad!"));
    let result = decrypt(&key1, &key2);
    println!("{}", result);
}
