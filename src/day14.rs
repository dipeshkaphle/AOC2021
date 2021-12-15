use std::{collections::HashMap, io::BufRead};

use crate::get_buffer;

pub struct Day14 {}
impl Day14 {
    fn read() -> (String, HashMap<String, String>) {
        let mut lines = get_buffer("input/day14.txt")
            .lines()
            .map(|x| x.unwrap())
            .into_iter();
        let template = lines.next().unwrap();
        lines.next();
        let mut rules = HashMap::new();
        loop {
            let next_ln = lines.next();
            match next_ln {
                Some(x) => {
                    let split = x.split(" -> ").map(|y| y.trim()).collect::<Vec<&str>>();
                    rules.insert(split[0].to_owned(), split[1].to_owned());
                }
                None => break,
            }
        }
        (template, rules)
    }

    fn split_to_pairs(s: &str) -> HashMap<String, u128> {
        let t = s.as_bytes();
        let mut h = HashMap::new();
        t.windows(2).for_each(|x| {
            *h.entry(x.iter().map(|y| *y as char).collect::<String>())
                .or_insert(0) += 1;
        });
        h
    }

    fn handle_match(
        s: &String,
        occ: &u128,
        pairs: &mut HashMap<String, u128>,
        rules: &HashMap<String, String>,
    ) {
        let s_as_bytes = s.as_bytes();
        let rhs = &rules[s].as_bytes()[0];
        let fst = vec![s_as_bytes[0], *rhs]
            .into_iter()
            .map(|x| x as char)
            .collect::<String>();
        let snd = vec![*rhs, s_as_bytes[1]]
            .into_iter()
            .map(|x| x as char)
            .collect::<String>();
        *pairs.entry(fst).or_insert(0) += *occ;
        *pairs.entry(snd).or_insert(0) += *occ;
    }

    fn get_individual_counts(
        pairs: &HashMap<String, u128>,
        last_char: char,
    ) -> HashMap<char, u128> {
        let mut h = HashMap::new();
        for (k, v) in pairs {
            let mut chars = k.chars();
            *h.entry(chars.next().unwrap()).or_insert(0) += v;
        }
        *h.entry(last_char).or_insert(0) += 1;
        h
    }
    fn common(lim: usize) -> u128 {
        let (template, rules) = Self::read();
        let last_char = template.as_bytes()[template.len() - 1] as char;
        let mut pairs = Self::split_to_pairs(&template);
        for _ in 0..lim {
            //
            let old = pairs.clone();
            let mut new_pairs = HashMap::new();
            for (s, occ) in old.iter() {
                if rules.contains_key(s) {
                    Self::handle_match(s, occ, &mut new_pairs, &rules);
                } else {
                    *new_pairs.entry(s.to_owned()).or_insert(0) += *occ;
                }
            }
            pairs = new_pairs;
        }
        let h = Self::get_individual_counts(&pairs, last_char);
        let mx = h.values().into_iter().max().unwrap().clone();
        let mn = h.values().into_iter().min().unwrap().clone();
        return mx - mn;
    }

    pub fn part_1() -> u128 {
        Self::common(10)
    }
    pub fn part_2() -> u128 {
        Self::common(40)
    }
}

#[cfg(test)]
mod tests {

    use super::Day14;

    #[test]
    fn test() {
        println!("{}", Day14::part_1());
        println!("{}", Day14::part_2());
    }
}
