use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, opt};
use nom::sequence::pair;
use nom::IResult;

fn signed_to_val<I>(s: (Option<&str>, &str)) -> Result<I, std::num::ParseIntError>
where
    I: std::str::FromStr<Err = std::num::ParseIntError> + std::ops::Neg<Output = I>,
{
    match s.0 {
        Some("-") => s.1.parse::<I>().map(|v| -v),
        _ => s.1.parse::<I>(),
    }
}

pub fn ival<I>(i: &str) -> IResult<&str, I>
where
    I: std::str::FromStr<Err = std::num::ParseIntError> + std::ops::Neg<Output = I>,
{
    let a = alt((tag("-"), tag("+")));
    map_res(pair(opt(a), digit1), signed_to_val)(i)
}

pub fn uval<U: std::str::FromStr>(i: &str) -> IResult<&str, U> {
    map_res(digit1, |s: &str| s.parse::<U>())(i)
}

#[cfg(test)]
mod tests {
    #[test]
    fn helper_parse_int32() {
        use super::*;
        assert_eq!(ival("123"), Ok(("", 123)));
        assert_eq!(ival("+123"), Ok(("", 123)));
        assert_eq!(ival("-123"), Ok(("", -123)));
    }

    #[test]
    fn helper_parse_uint32() {
        use super::*;
        assert_eq!(uval("123"), Ok(("", 123)));
    }
}
