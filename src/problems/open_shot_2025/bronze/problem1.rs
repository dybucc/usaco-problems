// understanding input through example
// 3 3 => number of possible inputs (>=1) | number of games to process
//
// number of possible inputs:
// D   => posed with 1 1, draw
// WD  => posed with 2 1, 2 wins; posed with 2 2, draw
// LWD => posed with 3 1, 1 wins; posed with 3 2, 3 wins; posed with 3 3; draw
//
// sample input:
// 1 2
// 2 3
// 1 1

#![allow(unused)]

use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Outcome {
    Draw,
    Win,
    Loss,
}

impl Outcome {
    fn convert(input: &u8) -> Self {
        match input {
            b'D' => Self::Draw,
            b'W' => Self::Win,
            b'L' => Self::Loss,
            _ => panic!(),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Input(usize, usize);

impl Input {
    fn new(i: usize, j: usize) -> Self {
        Self(i, j)
    }

    fn combine(&self, other: &Self) -> [Input; 4] {
        [
            Self(self.0, other.0),
            Self(self.1, other.1),
            Self(self.0, other.1),
            Self(self.1, other.0),
        ]
    }
}

trait Comparator {
    fn compare(&self, other: &Self, rules: HashMap<Input, Outcome>) -> Self;
}

impl Comparator for Vec<Input> {
    fn compare(&self, other: &Self, rules: HashMap<Input, Outcome>) -> Self {
        let mut output = vec![];
        let mut checker = vec![];

        for i in self {
            for j in other {
                let result = i.combine(j);

                for r in result {
                    if rules.contains_key(&r) && *rules.get(&r).unwrap() == Outcome::Draw
                        || *rules.get(&r).unwrap() == Outcome::Win
                    {
                        checker.push((r.clone(), rules.get(&r).unwrap().clone()));
                    }
                }

                if checker.iter().any(|v| v.1 == Outcome::Win) {
                    for (elem, _) in &checker {
                        output.push(elem.clone());
                    }
                }

                // for e.g. the possibilities and the counter in that order
                // 1 1, 2 1, 2 2, 3 1, 3 2, 3 3
                // 1 2, 2 3, 1 1
            }
        }

        output
    }
}

fn process_input(input: &str) -> (HashMap<Input, Outcome>, Vec<Input>, Vec<Input>) {
    let (symbols, input) = input.split_at(input.find(' ').unwrap());
    let symbols: usize = symbols.parse().unwrap();
    let (games, input) = input.split_at(input.find('\n').unwrap());
    let games = games.parse().unwrap();
    let (relations, input) = input.split_at(input.find(|v: char| v.is_numeric()).unwrap());

    let mut possibilities = vec![];
    let mut counter = Vec::with_capacity(games);
    let mut map: HashMap<Input, Outcome> = HashMap::with_capacity(symbols);

    for (i, line) in relations.trim().lines().enumerate() {
        for (j, c) in line.as_bytes().iter().enumerate() {
            if c.is_ascii_alphabetic() {
                map.insert(Input::new(i + 1, j + 1), Outcome::convert(c));
                possibilities.push(Input::new(i + 1, j + 1));
            }
        }
    }

    for line in input.lines() {
        let (i, line) = line.split_at(line.find(' ').unwrap());
        let i: usize = i.parse().unwrap();
        let j: usize = line.parse().unwrap();

        counter.push(Input::new(i, j));
    }

    (map, possibilities, counter)
}
