use crate::datatypes::Expr;

/// Converts an `Expr` into x86-64 assembly instructions
pub fn compile_expr(e: &Expr) -> String {
    match e {
        Expr::Num(n) => format!("mov rax, {}", *n),
        Expr::Add1(subexpr) => compile_expr(subexpr) + "\nadd rax, 1",
        Expr::Sub1(subexpr) => compile_expr(subexpr) + "\nsub rax, 1",
    }
}

/// Possible unit tests for compiler functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_num() {
        let expr = Expr::Num(5);
        let result = compile_expr(&expr);
        assert_eq!(result, "mov rax, 5");
    }

    #[test]
    fn test_compile_add1() {
        let expr = Expr::Add1(Box::new(Expr::Num(5))); // add1(5) → assembly
        let result = compile_expr(&expr);
        assert_eq!(result, "mov rax, 5\nadd rax, 1");
    }

    #[test]
    fn test_compile_sub1() {
        let expr = Expr::Sub1(Box::new(Expr::Num(10))); // sub1(10) → assembly
        let result = compile_expr(&expr);
        assert_eq!(result, "mov rax, 10\nsub rax, 1");
    }

    #[test]
    fn test_compile_nested_add1_sub1() {
        let expr = Expr::Add1(Box::new(Expr::Sub1(Box::new(Expr::Num(8))))); // add1(sub1(8))
        let result = compile_expr(&expr);
        assert_eq!(result, "mov rax, 8\nsub rax, 1\nadd rax, 1");
    }

    #[test]
    fn test_compile_multiple_add1() {
        let expr = Expr::Add1(Box::new(Expr::Add1(Box::new(Expr::Num(3))))); // add1(add1(3))
        let result = compile_expr(&expr);
        assert_eq!(result, "mov rax, 3\nadd rax, 1\nadd rax, 1");
    }

    #[test]
    fn test_compile_multiple_sub1() {
        let expr = Expr::Sub1(Box::new(Expr::Sub1(Box::new(Expr::Num(5))))); // sub1(sub1(5))
        let result = compile_expr(&expr);
        assert_eq!(result, "mov rax, 5\nsub rax, 1\nsub rax, 1");
    }

    #[test]
    fn test_compile_complex_nested() {
        let expr = Expr::Sub1(Box::new(Expr::Add1(Box::new(Expr::Add1(Box::new(Expr::Num(4))))))); // sub1(add1(add1(4)))
        let result = compile_expr(&expr);
        assert_eq!(result, "mov rax, 4\nadd rax, 1\nadd rax, 1\nsub rax, 1");
    }
}
