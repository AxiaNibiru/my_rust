#![allow(unused)]
#![allow(dead_code)]

use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

#[test]
pub fn test1() {
    fn is_valid(input: String) -> bool {
        let mut pair: HashMap<char, char> = HashMap::from([
            (')', '('), 
            ('}', '{'), 
            (']', '[')
        ]);

        let mut stack: VecDeque<char> = VecDeque::new();

        for c in input.chars() {
            match c {
                c if pair.values().any(|&value| value == c) => stack.push_back(c),
                c if stack.len() == 0 || !pair.contains_key(&c) => return false,
                _ => {
                    stack.pop_back();
                }
            }
        }

        stack.len() == 0
    }

    let input: String = "{{}}()[{}]".to_owned();

    let result: bool = is_valid(input);
    println!("{}", result);
}


