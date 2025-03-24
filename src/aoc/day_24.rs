use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Op {
    And,
    Or,
    Xor
}

#[derive(Debug)]
struct Wire {
    name: u32,
    state: bool
}
#[derive(Debug)]
struct Gate {
    op: Op,
    inputs: (u32, u32),
    output: u32
}

struct Puzzle {
    wires: Vec<Wire>,
    gates: Vec<Gate>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let data = data.trim().replace("\r", "");
        let parse_wire = |s: &str| {
            let (name, state) = s.split_once(": ")?;
            if name.len() != 3 {
                return None;
            }
            if (state != "0") && (state != "1") {
                return None;
            }
            let name = Self::encode_name(name);
            let state = state == "1";
            Some(Wire {name, state})
        };
        let parse_gate = |s: &str| {
            let (aob, output) = s.split_once(" -> ")?;
            let output = Self::encode_name(output);
            let mut aob = aob.split(" ");
            let a = aob.next()?;
            let op = aob.next()?;
            let b = aob.next()?;
            if aob.next().is_some() {return None;}
            if (a.len() != 3) || (a.len() != 3) {return None;}
            let a = Self::encode_name(a);
            let b = Self::encode_name(b);
            let op = match op {
                "AND" => Op::And,
                "OR"  => Op::Or,
                "XOR" => Op::Xor,
                _ => return None
            };
            let inputs = (a, b);
            Some(Gate {op, inputs, output})
        };
        let (wires, gates) = data.split_once("\n\n")?;
        let wires: Option<_> = wires.trim().lines().map(parse_wire).collect();
        let wires = wires?;
        let gates: Option<_> = gates.trim().lines().map(parse_gate).collect();
        let gates = gates?;
        Some(Puzzle {wires, gates})
    }
 
    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }

    fn encode_name(name: &str) -> u32 {
        let mut name = name.bytes();
        let (a, b, c) = (
            name.next().unwrap() as u32,
            name.next().unwrap() as u32,
            name.next().unwrap() as u32);
        (a << 16) | (b << 8) | c
    }

    fn decode_name(name: u32) -> String {
        let (a, b, c) = (((name >> 16) & 0xff), ((name >> 8) & 0xff), (name & 0xff));
        let (a, b, c) = (a as u8, b as u8, c as u8);
        unsafe {String::from_utf8_unchecked(vec![a, b, c])}
    }
}

fn part_1(puzzle: &Puzzle) -> Option<u64> {
    let mut wires = HashMap::new();
    for w in &puzzle.wires {
        wires.insert(w.name, w.state);
    }
    let mut gates = HashMap::new();
    for g in &puzzle.gates {
        gates.insert(g.output, g);
    }
    let mut compute: Vec<_> = gates.iter()
        .filter(|(n, _)| ((*n >> 16) as u8) == b'z')
        .map(|(n, g)| (*n, g)).collect();
    while !compute.is_empty() {
        let (n, g) = compute.pop().unwrap();
        let i0 = wires.contains_key(&g.inputs.0);
        let i1 = wires.contains_key(&g.inputs.1);
        if i0 && i1 {
            let i0 = wires.get(&g.inputs.0).unwrap();
            let i1 = wires.get(&g.inputs.1).unwrap();
            let s = match g.op {
                Op::And => i0 & i1,
                Op::Or => i0 | i1,
                Op::Xor => i0 ^ i1
            };
            wires.insert(n, s);
        } else {
            compute.push((n, g));
            if !i0 {
                let g = gates.get(&g.inputs.0)?;
                compute.push((g.output, g));
            }
            if !i1 {
                let g = gates.get(&g.inputs.1)?;
                compute.push((g.output, g));
            }
        }
    }
    let mut zs: Vec<_> = wires.into_iter().filter(|(n, _)| ((n >> 16) as u8) == b'z').collect();
    zs.sort();
    zs.reverse();
    Some(zs.into_iter().fold(0, |a, (_, b)| (a << 1) | b as u64))
}

fn solve_part_1(puzzle: &Puzzle) -> u64 {
    part_1(puzzle).expect("solvable puzzle")
}

fn solve_part_2(puzzle: &Puzzle) -> u32 {
    todo!("part 2")
}

pub(crate) fn solve() {
    let puzzle = include_str!("../../data/day_24/input.txt");
    let puzzle = Puzzle::load(puzzle);
    println!("part 1: {}", solve_part_1(&puzzle));
    println!("part 2: {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let puzzle = include_str!("../../data/day_24/test.txt");
        let puzzle = Puzzle::load(puzzle);
        assert!(solve_part_1(&puzzle) == 2024);
    }
}
