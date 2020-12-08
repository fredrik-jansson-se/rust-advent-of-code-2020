use nom::character::complete::newline;

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
    // Add,
    // Mul,
    // Store,
    // Load,
    // JumpIfTrue,
    // JumpIfFalse,
    // LT,
    // Eq,
    // AdjRelBase,
    // End,
}

fn nop(i: &str) -> nom::IResult<&str, Op> {
    let (i, _) = nom::bytes::complete::tag("nop ")(i)?;
    let (i, v) = crate::helper::ival(i)?;
    Ok((i, Op::Nop(v)))
}
fn acc(i: &str) -> nom::IResult<&str, Op> {
    let (i, _) = nom::bytes::complete::tag("acc ")(i)?;
    let (i, v) = crate::helper::ival(i)?;
    Ok((i, Op::Acc(v)))
}
fn jmp(i: &str) -> nom::IResult<&str, Op> {
    let (i, _) = nom::bytes::complete::tag("jmp ")(i)?;
    let (i, v) = crate::helper::ival(i)?;
    Ok((i, Op::Jmp(v)))
}

fn operation(i: &str) -> nom::IResult<&str, Op> {
    let (i, op) = nom::branch::alt((nop, acc, jmp))(i)?;
    Ok((i, op))
}

fn parse_program(i: &str) -> nom::IResult<&str, Vec<Op>> {
    nom::multi::separated_list1(newline, operation)(i)
}

#[derive(Clone)]
pub struct CPU {
    pub pc: usize,
    pub code: Vec<Op>,
    pub acc: i64,
}

impl CPU {
    pub fn new(program: &str) -> Self {
        let (_, code) = parse_program(program).unwrap();
        CPU {
            pc: 0,
            code,
            acc: 0,
        }
    }

    pub fn step(&mut self) -> Option<()> {
        // use nom::lib::std::prelude::v1::v1::stringify;
        if let Some(op) = self.code.get(self.pc) {
            let res = match op {
                Op::Nop(_) => {
                    self.pc += 1;
                }
                Op::Acc(a) => {
                    self.acc += *a;
                    self.pc += 1;
                }
                Op::Jmp(a) => {
                    self.pc = (self.pc as i64 + *a) as usize;
                }
            };
            // dbg! {(self.pc, &op)};
            Some(res)
        } else {
            None
        }
    }

    // pub fn run(&mut self) {
    //     while self.step().is_some() {}
    // }
}

#[cfg(test)]
mod tests {

    #[test]
    fn intcode_parse() {
        let (_, op) = super::operation("nop +0").unwrap();
        assert_eq!(op, super::Op::Nop(0));

        let (_, op) = super::operation("acc 1").unwrap();
        assert_eq!(op, super::Op::Acc(1));
        let (_, op) = super::operation("acc +1").unwrap();
        assert_eq!(op, super::Op::Acc(1));
        let (_, op) = super::operation("acc -1").unwrap();
        assert_eq!(op, super::Op::Acc(-1));

        let (_, op) = super::operation("jmp 1").unwrap();
        assert_eq!(op, super::Op::Jmp(1));
        let (_, op) = super::operation("jmp +1").unwrap();
        assert_eq!(op, super::Op::Jmp(1));
        let (_, op) = super::operation("jmp -1").unwrap();
        assert_eq!(op, super::Op::Jmp(-1));
    }

    #[test]
    fn intcode_parse_program() {
        const INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        // Assert we can parse line by line
        for line in INPUT.lines() {
            let (i, _) = super::operation(line).unwrap();
            assert_eq!(i, "");
        }
        let (i, code) = super::parse_program(INPUT).unwrap();
        assert_eq!(i, "");
    }
}
