use bool_eval::{eval_program, lex, parse_program, ErrorPrinter, ExprTreePrinter};

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

fn run(input: &str) {
    let run = || {
        // How I wish `try` blocks were a thing. :-(
        let program = parse_program(input, lex(input))?;
        if cfg!(feature = "show-tree") {
            println!(
                "=== PARSE TREE ===\n{}======",
                ExprTreePrinter(&program.expr)
            );
        }
        eval_program(&program)
    };
    match run() {
        Ok(val) => println!("{val}"),
        Err(error) => print!("{}", ErrorPrinter(&error, &input)),
    }
}

fn main() {
    let mut args = std::env::args().skip(1);

    if let Some(path) = args.next() {
        let src = std::fs::read_to_string(path).unwrap();
        run(&src);
        return;
    }

    for input in repl(">>> ") {
        if input.trim().is_empty() {
            continue;
        }
        run(&input);
    }
}
