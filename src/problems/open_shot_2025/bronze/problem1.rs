use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Input(usize, usize);

impl Input {
    fn new(i: usize, j: usize) -> Self {
        Self(i, j)
    }
}

trait Comparator {
    fn compare(&self, counter: &Self, rules: HashMap<Input, Outcome>) -> Vec<u32>;
}

impl Comparator for Vec<Input> {
    fn compare(&self, counter: &Self, rules: HashMap<Input, Outcome>) -> Vec<u32> {
        let mut output = vec![0; counter.len()];

        for (idx, i) in counter.iter().enumerate() {
            let hoof1 = i.0;
            let hoof2 = i.1;

            for j in self {
                let mut check1 = false;
                let mut check2 = false;

                if *rules.get(&Input::new(j.0, hoof1)).unwrap() == Outcome::Win
                    && *rules.get(&Input::new(j.0, hoof2)).unwrap() == Outcome::Win
                {
                    check1 = true;
                }

                if *rules.get(&Input::new(j.1, hoof1)).unwrap() == Outcome::Win
                    && *rules.get(&Input::new(j.1, hoof2)).unwrap() == Outcome::Win
                {
                    check2 = true;
                }

                if check1 || check2 {
                    *output.get_mut(idx).unwrap() += 1;
                    dbg!(j.0);
                    dbg!(j.1);
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
    let games = games.trim().parse().unwrap();
    let (relations, input) = input.split_at(input.find(|v: char| v.is_numeric()).unwrap());

    let mut map: HashMap<Input, Outcome> = HashMap::with_capacity(symbols);
    let mut possibilities = vec![];
    let mut counter = Vec::with_capacity(games);

    for (i, line) in relations.trim().lines().enumerate() {
        for (j, c) in line.as_bytes().iter().enumerate() {
            let dummy: Outcome = c.into();

            map.insert(Input::new(i + 1, j + 1), dummy);
            possibilities.push(Input::new(i + 1, j + 1));

            if i != j {
                map.insert(Input::new(j + 1, i + 1), dummy.rev());
                possibilities.push(Input::new(j + 1, i + 1));
            }
        }
    }

    for line in input.lines() {
        let (i, line) = line.split_at(line.find(' ').unwrap());
        let i: usize = i.parse().unwrap();
        let j: usize = line.trim().parse().unwrap();

        counter.push(Input::new(i, j));
    }

    (map, possibilities, counter)
}

pub fn process(input: &str) -> Vec<u32> {
    let (rules, besie, elsie) = process_input(input);

    besie.compare(&elsie, rules)
}
