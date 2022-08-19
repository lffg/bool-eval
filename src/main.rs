use bool_eval::{eval_program, lex, parse_program, ErrorPrinter, ExprTreePrinter, PResult};

fn repl(prompt: &str) -> impl Iterator<Item = String> + '_ {
    use std::io::{stdin, stdout, Write};

    std::iter::from_fn(move || {
        print!("{prompt}");
        stdout().flush().unwrap();
        let mut buf = String::new();
        match stdin().read_line(&mut buf).unwrap() {
            0 => None,
            _ => Some(buf.trim().into()),
        }
    })
}

fn run(input: &str) -> PResult<bool> {
    let program = parse_program(input, lex(input))?;
    if cfg!(feature = "show-tree") {
        println!(
            "=== PARSE TREE ===\n{}======",
            ExprTreePrinter(&program.expr)
        );
    }
    let val = eval_program(&program)?;
    Ok(val)
}

fn main() {
    for input in repl(">>> ") {
        if input.trim().is_empty() {
            continue;
        }
        match run(&input) {
            Ok(val) => println!("{val}"),
            Err(error) => print!("{}", ErrorPrinter(&error, &input)),
        }
    }
}
