use std::fs;

pub fn run() {
    let input = fs::read_to_string("day8.txt").unwrap();
    println!("day8-1: {}", run_1(&input));
    println!("day8-2: {}", run_2(&input));
}

fn run_1(input: &str) -> i64 {
    let mut cpu = crate::intcode::CPU::new(input);
    let mut visited_pcs = std::collections::HashSet::new();
    while !visited_pcs.contains(&cpu.pc) {
        // dbg!(cpu.pc);
        visited_pcs.insert(cpu.pc);
        cpu.step();
        // dbg!(cpu.pc);
    }
    cpu.acc
}

fn run_2(input: &str) -> i64 {
    let orig_cpu = crate::intcode::CPU::new(input);

    for (pc, op) in orig_cpu.code.iter().enumerate() {
        let new_op = match op {
            crate::intcode::Op::Jmp(v) => Some(crate::intcode::Op::Nop(*v)),
            crate::intcode::Op::Nop(v) => Some(crate::intcode::Op::Jmp(*v)),
            _ => None,
        };
        if let Some(new_op) = new_op {
            let mut cpu = orig_cpu.clone();
            cpu.code[pc] = new_op;
            let mut visited_pcs = std::collections::HashSet::new();
            while !visited_pcs.contains(&cpu.pc) {
                // dbg!(cpu.pc);
                visited_pcs.insert(cpu.pc);
                if cpu.step().is_none() {
                    return cpu.acc;
                }
            }
        }
    }
    panic!("Didn't solve")
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn aoc8_run_1() {
        assert_eq!(super::run_1(INPUT), 5);
    }

    #[test]
    fn aoc8_run_2() {
        assert_eq!(super::run_2(INPUT), 8);
    }
}
