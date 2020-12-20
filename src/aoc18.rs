pub fn run() {
    let input = std::fs::read_to_string("day18.txt").unwrap();
    println!("18:1: {}", run_1(&input));
    println!("18:2: {}", run_2(&input));
}

#[derive(Debug)]
enum Token {
    Num(usize),
    Add,
    Mul,
    OpenParen,
    CloseParen,
}

#[derive(Debug)]
enum Expr {
    Num(usize),
    Add(Box<Expr>, Option<Box<Expr>>),
    Mul(Box<Expr>, Option<Box<Expr>>),
}

fn parse_tokens(i: &str) -> nom::IResult<&str, Vec<Token>> {
    let add = nom::combinator::map(nom::bytes::complete::tag("+"), |_| Token::Add);
    let mul = nom::combinator::map(nom::bytes::complete::tag("*"), |_| Token::Mul);
    let open = nom::combinator::map(nom::bytes::complete::tag("("), |_| Token::OpenParen);
    let close = nom::combinator::map(nom::bytes::complete::tag(")"), |_| Token::CloseParen);
    let num = nom::combinator::map(crate::helper::uval, Token::Num);

    let token = nom::branch::alt((add, mul, open, close, num));
    let token = nom::sequence::preceded(nom::character::complete::space0, token);
    nom::multi::many1(token)(i)
}

fn eval_inner_1(mut idx: usize, tokens: &[Token]) -> (usize, usize) {
    let mut res = None;
    let mut op = None;
    while idx < tokens.len() {
        match tokens[idx] {
            Token::Add => {
                op = Some(Token::Add);
            }
            Token::Mul => {
                op = Some(Token::Mul);
            }
            Token::Num(n) => match op.take() {
                Some(Token::Add) => {
                    res.as_mut().map(|res| *res += n);
                }
                Some(Token::Mul) => {
                    res.as_mut().map(|res| *res *= n);
                }
                _ => {
                    res = Some(n);
                }
            },
            Token::OpenParen => {
                let (new_idx, n) = eval_inner_1(idx + 1, tokens);
                idx = new_idx;
                match op.take() {
                    Some(Token::Add) => {
                        res.as_mut().map(|res| *res += n);
                    }
                    Some(Token::Mul) => {
                        res.as_mut().map(|res| *res *= n);
                    }
                    _ => {
                        res = Some(n);
                    }
                }
            }
            Token::CloseParen => {
                break;
            }
        }
        idx += 1;
    }
    (idx, res.unwrap())
}

fn eval_1(input: &str) -> usize {
    let (_, tokens) = parse_tokens(input).unwrap();

    eval_inner_1(0, &tokens).1
}

// fn eval_inner_2(mut idx: usize, tokens: &[Token]) -> (usize, usize) {
//     let mut res = None;
//     let mut op = None;
//     println!("--> {:?}", &tokens[idx..]);
//     while idx < tokens.len() {
//         dbg! {(idx, &tokens[idx])};
//         match tokens[idx] {
//             Token::Add => {
//                 op = Some(Token::Add);
//             }
//             Token::Mul => {
//                 let (new_idx, right) = eval_inner_2(idx + 1, tokens);
//                 idx = new_idx;
//                 res.as_mut().map(|res| *res *= right);
//             }
//             Token::Num(n) => match op.take() {
//                 Some(Token::Add) => {
//                     res.as_mut().map(|res| *res += n);
//                 }
//                 Some(Token::Mul) => {
//                     unreachable!();
//                 }
//                 _ => {
//                     res = Some(n);
//                 }
//             },
//             Token::OpenParen => {
//                 let (new_idx, n) = eval_inner_2(idx + 1, tokens);
//                 idx = new_idx;
//                 match op.take() {
//                     Some(Token::Add) => {
//                         res.as_mut().map(|res| *res += n);
//                     }
//                     Some(Token::Mul) => {
//                         unreachable!();
//                     }
//                     _ => {
//                         res = Some(n);
//                     }
//                 }
//                 continue;
//             }
//             Token::CloseParen => {
//                 idx += 1;
//                 dbg! {res};
//                 break;
//             }
//         }
//         idx += 1;
//     }
//     println!("<--");
//     (idx, res.unwrap())
// }

fn eval_inner_2(tokens: &mut Vec<Token>) -> usize {
    let mut stack = Vec::new();
    let mut op_stack = Vec::new();

    while !tokens.is_empty() {
        let token = tokens.pop().unwrap();
        match token {
            Token::Num(v) => stack.push(v),
            Token::CloseParen => {
                break;
            }
            Token::OpenParen => {
                stack.push(eval_inner_2(tokens));
            }
            Token::Add => {
                op_stack.push(Token::Add);
            }
            Token::Mul => {
                op_stack.push(Token::Add);
            }
        }
    }

    assert_eq!(stack.len(), 1);
    return stack.pop().unwrap();
}
fn eval_2(input: &str) -> usize {
    // dbg! {input};
    let (_, mut tokens) = parse_tokens(input).unwrap();
    tokens.reverse();

    eval_inner_2(&mut tokens)
}

fn run_1(input: &str) -> usize {
    input.lines().map(eval_1).sum()
}

fn run_2(input: &str) -> usize {
    input.lines().map(eval_2).sum()
}
// 26441552469787634 too high

#[cfg(test)]
mod tests {
    #[test]
    fn aoc18_eval_1() {
        assert_eq!(super::eval_1("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(super::eval_1("2 * 3 + (4 * 5)"), 26);
        assert_eq!(super::eval_1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(
            super::eval_1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            12240
        );
        assert_eq!(
            super::eval_1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn aoc18_eval_2() {
        assert_eq!(super::eval_2("2 * 1 + 2"), 6);
        assert_eq!(super::eval_2("2 * 1 + 2 + 3"), 12);
        // assert_eq!(super::eval_2("2 * (1+2) + 2"), 10);
        // assert_eq!(super::eval_2("2 * (1+2)"), 6);
        // assert_eq!(
        //     super::eval_2(
        //         "2 * 9 + 5 + ((8 + 6 + 5) * (2 + 3 * 9 + 3) + 5) * (7 + 9 + 7 + 3 * 7) * 5"
        //     ),
        //     23340
        // );
        // assert_eq!(super::eval_2("1 + 2 * 3 + 4 * 5 + 6"), 231);
        // assert_eq!(super::eval_2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        // assert_eq!(super::eval_2("2 * 3 + (4 * 5)"), 46);
        // assert_eq!(super::eval_2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        // assert_eq!(
        //     super::eval_2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        //     669060
        // );
        assert_eq!(
            super::eval_2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}
