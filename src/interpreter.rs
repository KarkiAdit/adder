use sexp::*;
use sexp::Atom::*;
use crate::datatypes::Expr;

/// Evaluates an `Expr` recursively
#[allow(dead_code)]
pub fn eval(e: &Expr) -> i32 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add1(e1) => eval(e1) + 1,
        Expr::Sub1(e1) => eval(e1) - 1,
        Expr::Negate(e1) => -eval(e1),
    }
}

/// Parses an S-expression into an `Expr` (as given in the assignment)
pub fn parse_expr(s: &Sexp) -> Expr {
    match s {
        Sexp::Atom(I(n)) => Expr::Num(i32::try_from(*n).unwrap()),
        Sexp::List(vec) => {
            match &vec[..] {
                [Sexp::Atom(S(op)), e] if op == "add1" => Expr::Add1(Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e] if op == "sub1" => Expr::Sub1(Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e] if op == "negate" => Expr::Negate(Box::new(parse_expr(e))), 
                _ => panic!("parse error"),
            }
        },
        _ => panic!("parse error"),
    }
}

/// Possible unit tests for interpreter functions
#[cfg(test)]
mod tests {
    use super::*;
    use sexp::{Sexp, Atom::*};

    #[test]
    fn test_eval_num() {
        let expr = Expr::Num(10);
        assert_eq!(eval(&expr), 10);
    }

    #[test]
    fn test_eval_zero() {
        let expr = Expr::Num(0);
        assert_eq!(eval(&expr), 0);
    }

    #[test]
    fn test_eval_negative_num() {
        let expr = Expr::Num(-5);
        assert_eq!(eval(&expr), -5);
    }

    #[test]
    fn test_eval_add1() {
        let expr = Expr::Add1(Box::new(Expr::Num(5))); // add1(5) => 6
        assert_eq!(eval(&expr), 6);
    }

    #[test]
    fn test_eval_sub1() {
        let expr = Expr::Sub1(Box::new(Expr::Num(7))); // sub1(7) => 6
        assert_eq!(eval(&expr), 6);
    }

    #[test]
    fn test_eval_nested() {
        let expr = Expr::Sub1(Box::new(Expr::Add1(Box::new(Expr::Num(8))))); // sub1(add1(8)) => 8
        assert_eq!(eval(&expr), 8);
    }

    #[test]
    fn test_eval_negate() {
        let expr = Expr::Negate(Box::new(Expr::Num(5))); // negate(5) => -5
        assert_eq!(eval(&expr), -5);
    }

    #[test]
    fn test_eval_double_negate() {
        let expr = Expr::Negate(Box::new(Expr::Negate(Box::new(Expr::Num(4))))); // negate(negate(4)) => 4
        assert_eq!(eval(&expr), 4);
    }

    #[test]
    fn test_eval_negate_add1() {
        let expr = Expr::Negate(Box::new(Expr::Add1(Box::new(Expr::Num(3))))); // negate(add1(3)) => -4
        assert_eq!(eval(&expr), -4);
    }

    #[test]
    fn test_eval_add1_negate() {
        let expr = Expr::Add1(Box::new(Expr::Negate(Box::new(Expr::Num(2))))); // add1(negate(2)) => -1
        assert_eq!(eval(&expr), -1);
    }

    #[test]
    fn test_parse_add1() {
        let sexpr = Sexp::List(vec![Sexp::Atom(S("add1".to_string())), Sexp::Atom(I(5))]);
        let parsed = parse_expr(&sexpr);
        assert_eq!(parsed, Expr::Add1(Box::new(Expr::Num(5))));
    }

    #[test]
    fn test_parse_sub1() {
        let sexpr = Sexp::List(vec![Sexp::Atom(S("sub1".to_string())), Sexp::Atom(I(7))]);
        let parsed = parse_expr(&sexpr);
        assert_eq!(parsed, Expr::Sub1(Box::new(Expr::Num(7))));
    }

    #[test]
    fn test_parse_negate() {
        let sexpr = Sexp::List(vec![Sexp::Atom(S("negate".to_string())), Sexp::Atom(I(9))]);
        let parsed = parse_expr(&sexpr);
        assert_eq!(parsed, Expr::Negate(Box::new(Expr::Num(9))));
    }

    #[test]
    fn test_parse_negate_nested() {
        let sexpr = Sexp::List(vec![Sexp::Atom(S("negate".to_string())), Sexp::List(vec![Sexp::Atom(S("add1".to_string())), Sexp::Atom(I(2))])]);
        let parsed = parse_expr(&sexpr);
        assert_eq!(parsed, Expr::Negate(Box::new(Expr::Add1(Box::new(Expr::Num(2))))));
    }

    #[test]
    fn test_parse_add1_negate() {
        let sexpr = Sexp::List(vec![Sexp::Atom(S("add1".to_string())), Sexp::List(vec![Sexp::Atom(S("negate".to_string())), Sexp::Atom(I(2))])]);
        let parsed = parse_expr(&sexpr);
        assert_eq!(parsed, Expr::Add1(Box::new(Expr::Negate(Box::new(Expr::Num(2))))));
    }

    #[test]
    #[should_panic(expected = "parse error")]
    fn test_parse_invalid_list() {
        let sexpr = Sexp::List(vec![Sexp::Atom(S("add1".to_string()))]); // Missing second argument
        parse_expr(&sexpr); // Should panic
    }

    #[test]
    #[should_panic(expected = "parse error")]
    fn test_parse_unexpected_expression() {
        let sexpr = Sexp::List(vec![Sexp::Atom(S("unknown".to_string())), Sexp::Atom(I(3))]);
        parse_expr(&sexpr); // Should panic
    }
}
