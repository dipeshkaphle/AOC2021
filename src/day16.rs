use std::io::BufRead;

use crate::get_buffer;

// struct Packet;

#[derive(Debug)]
enum PacketInternal {
    Literal(u128),
    NonLiteral(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: usize,
    pkt_type: usize,
    internal: PacketInternal,
}

impl Packet {
    pub fn version_sum_dfs(&self) -> usize {
        match &self.internal {
            PacketInternal::Literal(_) => self.version,
            PacketInternal::NonLiteral(vec) => vec
                .iter()
                .fold(self.version, |acc, x| acc + x.version_sum_dfs()),
        }
    }
    pub fn get_literal_from_internal(&self) -> u128 {
        match &self.internal {
            PacketInternal::Literal(x) => *x,
            PacketInternal::NonLiteral(_) => unimplemented!(),
        }
    }

    pub fn get_subpackets_from_internal(&self) -> &Vec<Packet> {
        match &self.internal {
            PacketInternal::Literal(_) => unimplemented!(),
            PacketInternal::NonLiteral(v) => v,
        }
    }

    pub fn eval(&self) -> u128 {
        match self.pkt_type {
            0 => self
                .get_subpackets_from_internal()
                .iter()
                .fold(0, |acc, f| acc + f.eval()),
            1 => self
                .get_subpackets_from_internal()
                .iter()
                .fold(1, |acc, f| acc * f.eval()),
            2 => self
                .get_subpackets_from_internal()
                .iter()
                .map(|x| x.eval())
                .min()
                .unwrap(),
            3 => self
                .get_subpackets_from_internal()
                .iter()
                .map(|x| x.eval())
                .max()
                .unwrap(),
            4 => self.get_literal_from_internal(),
            5 => {
                let packets = self
                    .get_subpackets_from_internal()
                    .iter()
                    .map(|x| x.eval())
                    .collect::<Vec<u128>>();
                if packets[0] > packets[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                let packets = self
                    .get_subpackets_from_internal()
                    .iter()
                    .map(|x| x.eval())
                    .collect::<Vec<u128>>();
                if packets[0] < packets[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                let packets = self
                    .get_subpackets_from_internal()
                    .iter()
                    .map(|x| x.eval())
                    .collect::<Vec<u128>>();
                if packets[0] == packets[1] {
                    1
                } else {
                    0
                }
            }
            _ => unimplemented!(),
        }
    }
}

struct Parser<'a> {
    inp: &'a mut [u8],
    it: usize,
}

impl<'a> Parser<'a> {
    pub fn new(inp: &'a mut [u8]) -> Self {
        Parser { inp, it: 0 }
    }

    pub fn is_empty(&self) -> bool {
        return self.peek() == None;
    }

    pub fn peek(&self) -> Option<u8> {
        if self.it >= self.inp.len() {
            None
        } else {
            Some(self.inp[self.it])
        }
    }
    pub fn consume(&mut self) -> Option<u8> {
        if self.it >= self.inp.len() {
            None
        } else {
            self.it += 1;
            Some(self.inp[self.it - 1])
        }
    }

    pub fn parse_literal(&mut self) -> Option<Vec<u8>> {
        let mut bits = vec![];
        loop {
            let id = self.consume()?;
            for _ in 0..4 {
                bits.push(self.consume()?);
            }
            if id == 0 {
                break;
            }
        }
        Some(bits)
    }

    pub fn parse(&mut self) -> Option<Packet> {
        let mut version_num = 0;
        for _ in 0..3 {
            version_num = (version_num * 2) + self.consume()?;
        }
        let mut pkt_type = 0;
        for _ in 0..3 {
            pkt_type = (pkt_type * 2) + self.consume()?;
        }
        if pkt_type == 4 {
            let mut p = Parser::new(&mut self.inp[self.it..]);
            let literals = p.parse_literal()?;
            let ans = literals.into_iter().fold(0 as u128, |acc, x| {
                return (acc << 1) + (x as u128);
            });
            self.it += p.it;
            return Some(Packet {
                version: version_num as usize,
                pkt_type: pkt_type as usize,
                internal: PacketInternal::Literal(ans),
            });
        } else {
            let len_type_id = self.consume()?;
            if len_type_id == 0 {
                let mut total_len_in_bits = 0;
                for _ in 0..15 {
                    total_len_in_bits *= 2;
                    total_len_in_bits += self.consume()? as usize;
                }
                let mut p = Parser::new(&mut self.inp[self.it..(self.it + total_len_in_bits)]);
                self.it += total_len_in_bits;
                let mut packets = vec![];
                loop {
                    let packet = p.parse();
                    if packet.is_none() {
                        break;
                    }
                    packets.push(packet.unwrap());
                }
                return Some(Packet {
                    version: version_num as usize,
                    pkt_type: pkt_type as usize,
                    internal: PacketInternal::NonLiteral(packets),
                });
            } else {
                let mut L = 0;
                for _ in 0..11 {
                    L *= 2;
                    L += self.consume()? as usize;
                }
                let mut packets = vec![];
                for _ in 0..L {
                    let packet = self.parse().unwrap();
                    packets.push(packet);
                }
                return Some(Packet {
                    version: version_num as usize,
                    pkt_type: pkt_type as usize,
                    internal: PacketInternal::NonLiteral(packets),
                });
            }
        }
    }
}

pub struct Day16 {}
impl Day16 {
    fn read() -> Vec<u8> {
        let mut rd = get_buffer("input/day16.txt").lines().map(|x| x.unwrap());
        let ln = rd.next().unwrap();
        ln.chars()
            .map(|x| {
                let num = u8::from_str_radix(&x.to_string(), 16).unwrap();
                vec![(num & 8) / 8, (num & 4) / 4, (num & (2)) / 2, num & 1]
            })
            .flatten()
            .collect()
    }
    pub fn part_1() -> usize {
        let mut inp = Self::read();
        let mut p = Parser::new(&mut inp);
        let x = p.parse().unwrap();
        x.version_sum_dfs()
    }
    pub fn part_2() -> u128 {
        let mut inp = Self::read();
        let mut p = Parser::new(&mut inp);
        let x = p.parse().unwrap();
        x.eval()
    }
}

#[cfg(test)]
mod tests {

    use super::Day16;

    #[test]
    fn test() {
        println!("{}", Day16::part_1());
        println!("{}", Day16::part_2());
    }
}
