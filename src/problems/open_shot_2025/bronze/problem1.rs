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
    fn compare(&self, counter: &Self, rules: HashMap<Input, Outcome>) -> Vec<u32>;
}

impl Comparator for Vec<Input> {
    fn compare(&self, counter: &Self, rules: HashMap<Input, Outcome>) -> Vec<u32> {
        let mut output = vec![0; counter.len()];

        // for each counter from elsie, analyze each individual number/hoof
        // the analysis should check if the possibility can win against it for at least one of them
        // so e.g. if elsie comes up with 2 3, one possibility is to come up with 3 3 as a counter
        // let's analyze it this way
        // for 2 in 2 3 against 3 3 => can 3 win against 2? yes, can 3 win against 2, yes
        // for 3 in 2 3 against 3 3 => can 3 win against 3? no, can 3 win against 3? no
        // in each one of the previous assertions, there ought to be at least one yes for 3 3 to be
        // a winning combination

        for (idx, i) in counter.iter().enumerate() {
            let x = i.0;
            let y = i.1;

            for j in self {
                let mut check1 = false;
                let mut check2 = false;

                if *rules.get(&Input::new(j.0, x)).unwrap() == Outcome::Win
                    || *rules.get(&Input::new(j.1, x)).unwrap() == Outcome::Win
                {
                    check1 = true;
                }

                if *rules.get(&Input::new(j.0, y)).unwrap() == Outcome::Win
                    || *rules.get(&Input::new(j.1, y)).unwrap() == Outcome::Win
                {
                    check2 = true;
                }

                if check1 && check2 {
                    *output.get_mut(idx).unwrap() += 1;
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
