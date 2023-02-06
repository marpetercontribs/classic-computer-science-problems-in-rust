//generic_search.rs
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

pub fn linear_contains<'a, T: 'a + PartialEq>( iterable: impl IntoIterator<Item = &'a T>, key: &T) -> bool {
    for item in iterable.into_iter() {
        if item == key {
            return true;
        }
    }
    false
}

pub fn binary_contains<'a, T: 'a + PartialOrd + ?Sized>( list: &Vec<&'a T>, key: &T) -> bool {
    let mut low: usize = 0;
    let mut high: usize = list.len() - 1;
    while low <= high {
        let middle = (low + high)/2;
        if list[middle] < key {
            low = middle+1;
        } else if list[middle] > key {
            high = middle-1;
        } else {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_contains_works() {
        assert_eq!( linear_contains(&vec!(1, 5, 15, 15, 15, 15, 20),&15), true );
        assert_eq!( linear_contains(&vec!(1, 5, 15, 15, 15, 15, 20),&6), false );
    }

    #[test]
    fn binary_contains_works() {
        assert_eq!( binary_contains(&vec!("a", "d", "e", "f", "z"),"f"), true);
        assert_eq!( binary_contains(&vec!("john", "mark", "ronald", "sarah"),"sheila"), false);
    }
}
