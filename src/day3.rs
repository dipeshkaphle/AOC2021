use std::{collections::HashMap, io::BufRead};

use crate::get_buffer;

struct Bits {
    counter: HashMap<usize, [usize; 2]>,
}
impl Bits {
    fn new() -> Self {
        Bits {
            counter: HashMap::new(),
        }
    }
    fn visit(&mut self, s: &str) {
        //
        s.as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, x)| match *x as char {
                '0' => self.counter.entry(i).or_insert([0; 2])[0] += 1,
                '1' => self.counter.entry(i).or_insert([0; 2])[1] += 1,
                _ => {}
            });
    }
    fn get_gamma_rate_and_epsilon_rate(&self) -> (usize, usize) {
        let mx = self.counter.keys().max().unwrap();
        let mut gamma_rate = String::new();
        let mut epsilon_rate = String::new();
        for i in 0..=(*mx) {
            if self.counter[&i][1] > self.counter[&i][0] {
                gamma_rate.push('1');
                epsilon_rate.push('0');
            } else {
                gamma_rate.push('0');
                epsilon_rate.push('1');
            }
        }
        (
            usize::from_str_radix(&gamma_rate, 2).unwrap(),
            usize::from_str_radix(&epsilon_rate, 2).unwrap(),
        )
        // unimplemented!()
    }
}

#[derive(Clone, Debug)]
struct Node {
    is_end: bool,
    count: usize,
    mp: HashMap<usize, usize>,
}

impl Node {
    fn new(is_end_: bool) -> Self {
        Node {
            is_end: is_end_,
            count: 0,
            mp: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct Trie {
    nodes: Vec<Node>,
    cur: usize,
}

impl Trie {
    fn new() -> Self {
        Trie {
            nodes: vec![Node::new(false); 2],
            cur: 2,
        }
    }
    fn insert(&mut self, s: &str) {
        let mut ptr = 1;
        for c in s.as_bytes() {
            let x = c - '0' as u8;
            self.nodes[ptr].count += 1;
            if !self.nodes[ptr].mp.contains_key(&(x as usize)) {
                self.nodes.push(Node::new(false));
                self.nodes[ptr].mp.insert(x as usize, self.cur);
                self.cur += 1;
            }
            ptr = self.nodes[ptr].mp[&(x as usize)];
        }
        self.nodes[ptr].is_end = true;
        self.nodes[ptr].count += 1;
    }
    fn dfs(&self, selector_fn: &dyn Fn(&Node) -> (usize, usize)) -> usize {
        let mut ptr = 1;
        let mut s = String::new();
        loop {
            let (index, val_chosen) = selector_fn(&self.nodes[ptr]);
            s.push(val_chosen.to_string().as_bytes()[0] as char);
            if self.nodes[index].count == 1 {
                ptr = self.nodes[ptr].mp[&val_chosen];
                while !self.nodes[ptr].is_end {
                    if self.nodes[ptr].mp.contains_key(&0) {
                        s.push('0');
                        ptr = self.nodes[ptr].mp[&0];
                    } else {
                        s.push('1');
                        ptr = self.nodes[ptr].mp[&1];
                    }
                }
                break;
                //
            } else {
                ptr = index;
            }
        }
        return usize::from_str_radix(&s, 2).unwrap();
    }
}

pub struct Day3 {}

impl Day3 {
    pub fn part_1() -> usize {
        let reader = get_buffer("input/day3.txt");
        let mut counter = Bits::new();
        reader
            .lines()
            .into_iter()
            .for_each(|x| counter.visit(&x.unwrap()));
        let (gamma_rate, epsilon_rate) = counter.get_gamma_rate_and_epsilon_rate();
        gamma_rate * epsilon_rate
    }

    pub fn calculate(mut inp: Vec<String>, f: fn(usize, usize) -> char) -> usize {
        let mut i = 0;
        loop {
            let ones = inp
                .iter()
                .filter(|x| x.as_bytes()[i] as char == '1')
                .count();
            let zeros = inp
                .iter()
                .filter(|x| x.as_bytes()[i] as char == '0')
                .count();
            let c = f(ones, zeros);
            inp = inp
                .into_iter()
                .filter(|x| x.as_bytes()[i] as char == c)
                .collect::<Vec<String>>();
            if inp.len() == 1 {
                break;
            }
            i += 1;
        }
        usize::from_str_radix(inp.first().unwrap(), 2).unwrap()
    }

    pub fn part_2_without_trie() -> usize {
        let lines = get_buffer("input/day3.txt")
            .lines()
            .into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<String>>();
        let o2 = Self::calculate(lines.clone(), |ones, zeros| {
            if zeros > ones {
                return '0';
            } else {
                return '1';
            }
        });
        let co2 = Self::calculate(lines.clone(), |ones, zeros| {
            if zeros <= ones {
                return '0';
            } else {
                return '1';
            }
        });
        o2 * co2
    }

    pub fn part_2() -> usize {
        let reader = get_buffer("input/day3.txt");
        let mut tr = Trie::new();
        reader.lines().into_iter().for_each(|x| {
            tr.insert(&x.unwrap());
        });
        let o2 = tr.dfs(&|nd: &Node| -> (usize, usize) {
            if nd.mp.contains_key(&0) && nd.mp.contains_key(&1) {
                let c0 = tr.nodes[nd.mp[&0]].count;
                let c1 = tr.nodes[nd.mp[&1]].count;
                if c0 > c1 {
                    return (nd.mp[&0], 0);
                } else {
                    return (nd.mp[&1], 1);
                }
            } else if nd.mp.contains_key(&0) {
                return (nd.mp[&0], 0);
            } else {
                return (nd.mp[&1], 1);
            }
        });
        let co2 = tr.dfs(&|nd: &Node| -> (usize, usize) {
            if nd.mp.contains_key(&0) && nd.mp.contains_key(&1) {
                let c0 = tr.nodes[nd.mp[&0]].count;
                let c1 = tr.nodes[nd.mp[&1]].count;
                if c0 <= c1 {
                    return (nd.mp[&0], 0);
                } else {
                    return (nd.mp[&1], 1);
                }
            } else if nd.mp.contains_key(&0) {
                return (nd.mp[&0], 0);
            } else {
                return (nd.mp[&1], 1);
            }
        });
        return o2 * co2;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use super::Day3;
        println!("{}", Day3::part_1());
        println!("{}", Day3::part_2());
        println!("{}", Day3::part_2_without_trie());
    }
}
