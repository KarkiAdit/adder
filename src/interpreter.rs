use sexp::*;
use sexp::Atom::*;
use crate::datatypes::Expr;

/// Evaluates an `Expr` recursively
pub fn eval(e: &Expr) -> i32 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add1(e1) => eval(e1) + 1,
        Expr::Sub1(e1) => eval(e1) - 1,
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
                _ => panic!("parse error"), // Keeping this as per the assignment
            }
        },
        _ => panic!("parse error"),
    }
}

/// Possible unit tests for interpreter functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_num() {
        let expr = Expr::Num(10);
        assert_eq!(eval(&expr), 10);
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
