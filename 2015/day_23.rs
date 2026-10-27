
#[derive(Clone, Copy)]
enum Register {
    A,
    B
}

#[derive(Clone, Copy)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i32),
    Jie(Register, i32),
    Jio(Register, i32),

}

struct Registers {
    a: i32,
    b: i32,
}

impl Registers {
    fn get(&mut self, reg: Register) -> i32 {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
        }
    }
    fn get_mut(&mut self, reg: Register) -> &mut i32 {
        match reg {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
        }
    }
}

fn solve(registers: &mut Registers, instructions: &[Instruction]) {
    let mut pc: i32 = 0;

    loop {
        if pc < 0 || pc >= instructions.len() as i32 { break }
        let inst = &instructions[pc as usize];
        match inst {
            Instruction::Hlf(reg) |
            Instruction::Tpl(reg) |
            Instruction::Inc(reg) => {
                let reg = registers.get_mut(*reg);
                match inst {
                    Instruction::Hlf(_) => *reg /= 2,
                    Instruction::Tpl(_) => *reg *= 3,
                    Instruction::Inc(_) => *reg += 1,
                    _ => unreachable!()
                }
            }
            Instruction::Jmp(off) => {
                pc += *off;
                continue
            }
            Instruction::Jie(reg, off) |
            Instruction::Jio(reg, off) => {
                let reg = registers.get(*reg);
                let cond = match inst {
                    Instruction::Jie(_, _) => reg % 2 == 0,
                    Instruction::Jio(_, _) => reg == 1,
                    _ => unreachable!()
                };
                if cond {
                    pc += *off;
                    continue;
                }
            }
        }
        pc += 1;
    }
}

fn parse(i: impl Iterator<Item: AsRef<str>>) -> Vec<Instruction> {
    i.map(|l| {
        let (inst, args) = l.as_ref().split_once(" ").unwrap();
        fn parse_reg(name: &str) -> Register {
            match name.trim() {
                "a" => Register::A,
                "b" => Register::B,
                _ => unreachable!()
            }
        }
        match inst {
            "hlf" => Instruction::Hlf(parse_reg(args)),
            "tpl" => Instruction::Tpl(parse_reg(args)),
            "inc" => Instruction::Inc(parse_reg(args)),
            "jmp" => Instruction::Jmp(args.trim().parse().unwrap()),
            "jie" | "jio" => {
                let (reg, off) = args.trim().split_once(", ").unwrap();
                let reg = parse_reg(reg);
                let off: i32 = off.parse().unwrap();
                match inst {
                    "jie" => Instruction::Jie(reg, off),
                    "jio" => Instruction::Jio(reg, off),
                    _ => unreachable!()
                }
            }
            _ => unreachable!()
        }
    }).collect()
}

fn main() {
    let input: Vec<_> = parse(aoc::get_input_file_lines());
    aoc::run(|| {
        let mut pt1 = Registers { a: 0, b: 0 };
        solve(&mut pt1, &input);
        let mut pt2 = Registers { a: 1, b: 0 };
        solve(&mut pt2, &input);
        (pt1.b, pt2.b)
    });
}

#[cfg(test)]
mod test {
    use crate::{Registers, parse, solve};

    #[test]
    fn example() {
        let insts = parse("inc a
jio a, +2
tpl a
inc a".lines());
        let mut registers = Registers { a: 0, b: 0 };
        solve(&mut registers, &insts);
        assert_eq!(registers.a, 2)
    }
}
