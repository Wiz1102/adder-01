// fn main() {
//     println!("Hello, world!");
// }


/// Compile a source program into a string of x86-64 assembly
fn compile(program: String) -> String {
    let num = program.trim().parse::<i32>().unwrap();
    return format!("mov rax, {}", num);
}

// fn main() {
//     let program = "37";
//     let compiled = compile(String::from(program));
//     println!("{}", compiled);
// }

use std::env;
use std::fs::File;
use std::io::prelude::*;

enum Expr {
    Num(i32),
    Add1(Box<Expr>),
    Sub1(Box<Expr>),
    Negate(Box<Expr>)
}

fn eval(e: &Expr) -> i32 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add1(e1) => eval(e1) + 1,
        Expr::Sub1(e1) => eval(e1) - 1,
        Expr::Negate(e1) => -eval(e1)
    }
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let in_name = &args[1];
    let out_name = &args[2];

    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    let result = compile(in_contents);

    let asm_program = format!("
section .text
global our_code_starts_here
our_code_starts_here:
  {}
  ret
", result);

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;  // Import everything from the outer module

    #[test]
    fn test_num() {
        assert_eq!(eval(&Expr::Num(10)), 10);
    }

    #[test]
    fn test_add1() {
        assert_eq!(eval(&Expr::Add1(Box::new(Expr::Num(4)))), 5);
    }

    #[test]
    fn test_sub1() {
        assert_eq!(eval(&Expr::Sub1(Box::new(Expr::Num(7)))), 6);
    }

    #[test]
    fn test_combination() {
        let expr = Expr::Add1(Box::new(Expr::Sub1(Box::new(Expr::Num(5)))));
        assert_eq!(eval(&expr), 5);
    }

    #[test]
    fn test_negate() {
        assert_eq!(eval(&Expr::Negate(Box::new(Expr::Num(5)))), -5);
    }

    #[test]
    fn test_negate_combination() {
        let expr = Expr::Negate(Box::new(Expr::Add1(Box::new(Expr::Num(5)))));
        assert_eq!(eval(&expr), -6);
    }
}


