use std::iter::Peekable;
use std::str::Chars;

pub struct Solution;

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut chars = expression.chars().peekable();
        Self::parse_expression(&mut chars)
    }

    pub fn parse_expression(chars: &mut Peekable<Chars>) -> bool {
        match chars.next() {
            Some('t') => true,
            Some('f') => false,
            Some('!') => {
                Self::consume_char(chars, '(');
                let result = !Self::parse_expression(chars);
                Self::consume_char(chars, ')');
                result
            }
            Some('&') => Self::parse_operator(chars, true, |a, b| a && b),
            Some('|') => Self::parse_operator(chars, false, |a, b| a || b),
            _ => panic!("Invalid expression"),
        }
    }

    pub fn parse_operator<F>(chars: &mut Peekable<Chars>, initial: bool, op: F) -> bool
    where
        F: Fn(bool, bool) -> bool,
    {
        Self::consume_char(chars, '(');
        let mut result = initial;

        while chars.peek() != Some(&')') {
            result = op(result, Self::parse_expression(chars));
            if chars.peek() == Some(&',') {
                chars.next();
            }
        }

        Self::consume_char(chars, ')');
        result
    }

    pub fn consume_char(chars: &mut Peekable<Chars<'_>>, arg: char) {
        if chars.next() != Some(arg) {
            panic!("Expected '{}' ", arg);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn hard_it_works() {
        let expression = "&(|(f))".to_owned();
        let result = Solution::parse_bool_expr(expression);
        assert_eq!(false, result);
    }
}
