pub fn run() {
    let input = std::fs::read_to_string("day18.txt").unwrap();
    println!("18:1: {}", run_1(&input));
    println!("18:2: {}", run_2(&input));
}

#[derive(Clone, Debug, PartialEq)]
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
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Group(Box<Expr>),
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

struct RecursiveDescent {
    current: usize,
    tokens: Vec<Token>,
}

impl RecursiveDescent {
    fn new(tokens: Vec<Token>) -> Self {
        RecursiveDescent { current: 0, tokens }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn check(&self, t: &Token) -> bool {
        self.peek().map(|t1| t1 == t).unwrap_or(false)
    }

    fn match_tokens(&mut self, t: &Token) -> bool {
        if self.check(t) {
            self.advance();
            return true;
        }
        return false;
    }

    fn primary(&mut self) -> Box<Expr> {
        if let Some(Token::Num(v)) = self.peek().map(|t| (*t).clone()) {
            self.advance();
            return Box::new(Expr::Num(v));
        }

        if self.match_tokens(&Token::OpenParen) {
            let expr = self.factor();
            self.advance();
            return Box::new(Expr::Group(expr));
        }
        unreachable!()
    }

    fn term(&mut self) -> Box<Expr> {
        let mut expr = self.primary();

        while self.match_tokens(&Token::Add) {
            let right = self.primary();
            expr = Box::new(Expr::Add(expr, right));
        }
        expr
    }

    fn factor(&mut self) -> Box<Expr> {
        let mut expr = self.term();

        while self.match_tokens(&Token::Mul) {
            let right = self.term();
            expr = Box::new(Expr::Mul(expr, right));
        }
        expr
    }
}

fn eval_expr(expr: &Expr) -> usize {
    match expr {
        Expr::Num(v) => *v,
        Expr::Add(a, b) => eval_expr(a) + eval_expr(b),
        Expr::Mul(a, b) => eval_expr(a) * eval_expr(b),
        Expr::Group(a) => eval_expr(a),
    }
}

fn eval_2(input: &str) -> usize {
    let (rest, tokens) = parse_tokens(input).unwrap();
    assert_eq!(rest, "");

    let mut rd = RecursiveDescent::new(tokens);
    let expr = rd.factor();
    eval_expr(&expr)
}

fn run_1(input: &str) -> usize {
    input.lines().map(eval_1).sum()
}

fn run_2(input: &str) -> usize {
    input.lines().map(eval_2).sum()
}

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
        assert_eq!(
            super::eval_1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn aoc18_eval_2() {
        assert_eq!(super::eval_2("1+2*3"), 9);
        assert_eq!(super::eval_2("1*2+3"), 5);
        assert_eq!(super::eval_2("1*2*3"), 6);
        assert_eq!(super::eval_2("1+(2*3)+2"), 9);
        assert_eq!(super::eval_2("((2+4*9) * (6+9*8+6) +6)"), 11664);
        assert_eq!(super::eval_2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(super::eval_2("2 * 3 + (4 * 5)"), 46);
        assert_eq!(super::eval_2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(
            super::eval_2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            669060
        );
        assert_eq!(
            super::eval_2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}
