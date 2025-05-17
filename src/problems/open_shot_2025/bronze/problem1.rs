#![allow(unused)]

use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Outcome {
    Draw,
    Win,
    Loss,
}

impl Outcome {
    fn rev(&self) -> Self {
        match self {
            Self::Draw => Self::Draw,
            Self::Win => Self::Loss,
            Self::Loss => Self::Win,
        }
    }
}

impl From<&u8> for Outcome {
    fn from(value: &u8) -> Self {
        match value {
            b'D' => Self::Draw,
            b'W' => Self::Win,
            b'L' => Self::Loss,
            _ => panic!(),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
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
    fn compare(&self, counter: &Self, rules: HashMap<Input, Outcome>) -> Self {
        let mut output = vec![];

        // for e.g. the counter and the possibilities in that order
        // 1 2,                                        2 3, 1 1
        // 1 1, 2 1, 1 2, 2 2, 3 1, 1 3, 3 2, 2 3, 3 3
        //
        // 1 1, 2 2, 1 2, 2 1

        for i in counter {
            for j in self {
                let mut checker = vec![];
                let result = j.combine(i);

                for r in result {
                    checker.push(*rules.get(&r).unwrap());
                }

                checker.dedup();

                if checker.iter().all(|v| *v != Outcome::Loss)
                    && checker.iter().any(|v| *v == Outcome::Win)
                {
                    output.push(*j);
                }
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

    let mut map: HashMap<Input, Outcome> = HashMap::with_capacity(symbols);
    let mut possibilities = vec![];
    let mut counter = Vec::with_capacity(games);

    for (i, line) in relations.trim().lines().enumerate() {
        for (j, c) in line.as_bytes().iter().enumerate() {
            let dummy: Outcome = c.into();

            map.insert(Input::new(i + 1, j + 1), dummy);
            possibilities.push(Input::new(i + 1, j + 1));

            // for e.g. 2 1, 2 wins but 1 loses, so 1 2 should have 1 losing and 2 winning
            if i != j {
                map.insert(Input::new(j + 1, i + 1), dummy.rev());
                possibilities.push(Input::new(j + 1, i + 1));
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
