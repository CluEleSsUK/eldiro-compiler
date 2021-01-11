#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, digits) = extract_digits(s);
        (s, Self(digits.parse().unwrap()))
    }
}

#[derive(Debug, PartialEq)]
pub enum Op { Add, Sub, Mul, Div }

impl Op {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, op) = extract_operator(s);
        (s, match op {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            _ => panic!("Invalid operator!"),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Number,
    pub rhs: Number,
    pub operator: Op,
}

impl Expr {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, lhs) = Number::new(s);
        let (s, _) = extract_whitespace(s);

        let (s, operator) = Op::new(s);
        let (s, _) = extract_whitespace(s);

        let (s, rhs) = Number::new(s);
        let (s, _) = extract_whitespace(s);

        return (s, Self { lhs, rhs, operator });
    }
}

// returns a tuple of remaining string and number (as a string)
pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    take_while(|c: char| c.is_ascii_digit(), s)
}

pub(crate) fn extract_operator(s: &str) -> (&str, &str) {
    (&s[1..], &s[0..1])
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| c == ' ', s)
}

fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let end_of_match = s.char_indices()
        .find_map(|(index, char)| if accept(char) { None } else { Some(index) })
        .unwrap_or_else(|| s.len());

    return (&s[end_of_match..], &s[..end_of_match]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_parse_correctly() {
        assert_eq!(Number::new("123"), ("", Number(123)))
    }

    #[test]
    fn operators_parse_correctly() {
        assert_eq!(Op::new("+"), ("", Op::Add));
        assert_eq!(Op::new("-"), ("", Op::Sub));
        assert_eq!(Op::new("*"), ("", Op::Mul));
        assert_eq!(Op::new("/"), ("", Op::Div));
    }

    #[test]
    fn one_plus_two_equals_three() {
        assert_eq!(Expr::new("1+2"), ("", Expr { lhs: Number(1), rhs: Number(2), operator: Op::Add }))
    }

    #[test]
    fn extract_number_from_expr() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
        assert_eq!(extract_digits("10+2"), ("+2", "10"))
    }

    #[test]
    fn do_not_extract_from_empty_input() {
        assert_eq!(extract_digits(""), ("", ""));
    }

    #[test]
    fn extract_digits_without_remainder() {
        assert_eq!(extract_digits("100"), ("", "100"));
    }

    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespace("   1 1 11"), ("1 1 11", "   "))
    }
}

