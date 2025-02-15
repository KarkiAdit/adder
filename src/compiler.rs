use crate::datatypes::Expr;

/// Converts an `Expr` into x86-64 assembly instructions
pub fn compile_expr(e: &Expr) -> String {
    match e {
        Expr::Num(n) => format!("mov rax, {}", *n),
        Expr::Add1(subexpr) => compile_expr(subexpr) + "\nadd rax, 1",
        Expr::Sub1(subexpr) => compile_expr(subexpr) + "\nsub rax, 1",
        Expr::Negate(subexpr) => compile_expr(subexpr) + "\nneg rax",
    }
}

/// Possible unit tests for compiler functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_zero() {
        let expr = Expr::Num(0); // Zero case
        let result = compile_expr(&expr);
        assert_eq!(result, "mov rax, 0");
    }

    #[test]
    fn test_compile_negative_num() {
        let expr = Expr::Num(-7); // Directly storing a negative number
        let result = compile_expr(&expr);
        assert_eq!(result, "mov rax, -7");
    }

    #[test]
    fn test_compile_large_num() {
        let expr = Expr::Num(1_000_000);
        let result = compile_expr(&expr);
        assert_eq!(result, "mov rax, 1000000");
    }

    #[test]
    fn test_compile_negate_zero() {
        let expr = Expr::Negate(Box::new(Expr::Num(0))); // negate(0) => 0
        let result = compile_expr(&expr);
        assert_eq!(result, "mov rax, 0\nneg rax"); // `neg rax` is redundant but should be tested
    }

    #[test]
    fn test_compile_add1_sub1() {
        let expr = Expr::Add1(Box::new(Expr::Sub1(Box::new(Expr::Num(15))))); // add1(sub1(15)) => 15
        let result = compile_expr(&expr);
        assert_eq!(result, "mov rax, 15\nsub rax, 1\nadd rax, 1"); // Should cancel out
    }

    #[test]
    fn test_compile_nested_negate() {
        let expr = Expr::Negate(Box::new(Expr::Negate(Box::new(Expr::Num(9))))); // negate(negate(9)) => 9
        let result = compile_expr(&expr);
        assert_eq!(result, "mov rax, 9\nneg rax\nneg rax");
    }

    #[test]
    fn test_compile_negate_add1() {
        let expr = Expr::Negate(Box::new(Expr::Add1(Box::new(Expr::Num(3))))); // negate(add1(3)) => -4
        let result = compile_expr(&expr);
        assert_eq!(result, "mov rax, 3\nadd rax, 1\nneg rax");
    }

    #[test]
    fn test_compile_add1_negate() {
        let expr = Expr::Add1(Box::new(Expr::Negate(Box::new(Expr::Num(2))))); // add1(negate(2)) => -1
        let result = compile_expr(&expr);
        assert_eq!(result, "mov rax, 2\nneg rax\nadd rax, 1");
    }

    #[test]
    fn test_compile_deeply_nested() {
        let expr = Expr::Negate(Box::new(Expr::Sub1(Box::new(Expr::Add1(Box::new(Expr::Num(100))))))); // negate(sub1(add1(100)))
        let result = compile_expr(&expr);
        assert_eq!(result, "mov rax, 100\nadd rax, 1\nsub rax, 1\nneg rax");
    }
}
