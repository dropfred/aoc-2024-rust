#[derive(Clone, Debug)]
struct Computer {
    ip: usize,
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u8>,
    out: String
}

impl Computer {
    fn new(a: u64, b: u64, c: u64, program: Vec<u8>) -> Self {
        Self {a, b, c, program, ip: 0, out: String::new()}
    }

    fn parse(data: &str) -> Option<Self> {
        let mut data = data.trim().lines();
        let a = data.next()?.trim().trim_start_matches("Register A: ").parse().ok()?;
        let b = data.next()?.trim().trim_start_matches("Register B: ").parse().ok()?;
        let c = data.next()?.trim().trim_start_matches("Register C: ").parse().ok()?;
        data.next();
        let program: Vec<_> = data.next()?.trim().trim_start_matches("Program: ").split(",").map(|s| s.parse()).collect();
        if program.iter().any(|b| b.is_err()) {
            return None;
        }
        let program: Vec<_> = program.into_iter().map(|b| b.unwrap()).collect();
        if program.iter().any(|b| *b > 7) {
            return None;
        }
        Some(Self::new(a, b, c, program))
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("vaalid input")
    }

    // fn reset(&mut self, a: u64, b: u64, c: u64) {
    //     self.a = a;
    //     self.b = b;
    //     self.c = c;
    //     self.ip = 0;
    //     self.out = String::new();
    // }

    fn combo(&self, op: u8) -> u64 {
        let op = op as u64;
        match op {
            0..=3 => op,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid combo")
        }
    }

    fn adv(&mut self, operand: u8) {
        self.a >>= self.combo(operand);
    }

    fn bxl(&mut self, operand: u8) {
        self.b ^= operand as u64;
    }

    fn bst(&mut self, operand: u8) {
        self.b = self.combo(operand) & 7;
    }
    
    fn jnz(&mut self, operand: u8) {
        if self.a != 0 {
            self.ip = operand as usize;
        }
    }

    fn bxc(&mut self) {
        self.b ^= self.c;
    }

    fn out(&mut self, operand: u8) {
        if !self.out.is_empty() {
            self.out.push(',');
        }
        self.out.push(((self.combo(operand) & 7) as u8 + b'0') as char);
    }

    fn bdv(&mut self, operand: u8) {
        self.b = self.a >> self.combo(operand);
    }

    fn cdv(&mut self, operand: u8) {
        self.c = self.a >> self.combo(operand);
    }
    
    fn run(&mut self) -> &str {
        while self.ip != self.program.len() {
            let (opcode, operand) = (self.program[self.ip], self.program[self.ip + 1]);
            self.ip += 2;
            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => panic!("invalid opcode")
            }
        }

        self.out.as_str()
    }
}

impl std::fmt::Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "A: {:X}", self.a)?;
        writeln!(f, "B: {:X}", self.b)?;
        writeln!(f, "C: {:X}", self.c)?;
        writeln!(f, "IP: {:X}", self.ip)?;
        writeln!(f, "")?;
        for ip in 0..(self.program.len() / 2) {
            let (opcode, operand) = (self.program[ip * 2], self.program[ip * 2 + 1]);
            let combo = if ((opcode != 1) && (opcode != 3) && (opcode != 4)) && (operand > 3) && (operand < 7) {
                ["A", "B", "C"][(operand - 4) as usize]
            } else {
                ""
            };
            let opcode =  match opcode {
                0 => "adv",
                1 => "bxl",
                2 => "bst",
                3 => "jnz",
                4 => "bxc",
                5 => "out",
                6 => "bdv",
                7 => "cdv",
                _ => panic!("invalid opcode")
            };
            write!(f, "{:02}: {} {}", ip * 2, opcode, operand)?;
            if !combo.is_empty() {
                write!(f, " ({})", combo)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "")?;
        write!(f, "OUT: '{}'", self.out)?;
        Ok(())
    }
}

fn part_1() -> String {
    let computer = include_str!("../../data/day_17/input.txt");
    let mut computer = Computer::parse(computer).expect("valid input");
    computer.run().to_string()
}

fn run_loop(a: u64) -> u8 {
    let b = (a & 7) ^ 4;
    let c = a >> b;
    (((b ^ c) ^ 4) & 7) as u8
}

fn solve_part_2(computer: &Computer) -> Option<u64> {
    let mut ps = vec![0];
    for b in computer.program.iter().rev() {
        let mut nps = Vec::new();
        for a in &ps {
            let na = a << 3;
            for na in na..(na + 8) {
                if run_loop(na) == *b {
                    if na > 0 {nps.push(na);}
                }
            }
        }
        if nps.is_empty() {
            return None;
        }
        ps = nps;
    }
    if ps.is_empty() {
        None
    } else {
        Some(ps[0])
    }
}

fn part_2() -> u64 {
    let computer = include_str!("../../data/day_17/input.txt");
    let computer = Computer::load(computer);
    solve_part_2(&computer).expect("solvable puzzle")
}

pub(crate) fn solve() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let computer = include_str!("../../data/day_17/test_1.txt");
        let computer = Computer::parse(computer).unwrap();
        assert_eq!(computer.a, 729);
        assert_eq!(computer.b, 0);
        assert_eq!(computer.c, 0);
        assert_eq!(computer.program.len(), 6);
    }

    #[test]
    fn test_run() {
        let mut computer = Computer::new(0, 0, 0, vec![5, 0]);
        assert_eq!(computer.run(), "0");

        let mut computer = Computer::new(10, 0, 0, vec![5, 4]);
        assert_eq!(computer.run(), "2");

        let mut computer = Computer::new(0, 0, 9, vec![2, 6]);
        assert_eq!(computer.run(), "");
        assert_eq!(computer.b, 1);

        let mut computer = Computer::new(0, 29, 0, vec![1, 7]);
        assert_eq!(computer.run(), "");
        assert_eq!(computer.b, 26);

        let mut computer = Computer::new(0, 2024, 43690, vec![4, 0]);
        assert_eq!(computer.run(), "");
        assert_eq!(computer.b, 44354);

        let mut computer = Computer::new(10, 0, 0, vec![5,0,5,1,5,4]);
        assert_eq!(computer.run(), "0,1,2");

        let mut computer = Computer::new(2024, 0, 0, vec![0,1,5,4,3,0]);
        assert_eq!(computer.run(), "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(computer.a, 0);

        let computer = include_str!("../../data/day_17/test_1.txt");
        let mut computer = Computer::parse(computer).unwrap();
        assert_eq!(computer.run(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(), "7,0,7,3,4,1,3,0,1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(), 156985331222018);
    }
}
