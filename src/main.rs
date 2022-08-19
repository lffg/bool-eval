use bool_eval::{
    evaluator::eval_program,
    lexer::lex,
    parser::parse_program,
    util::{Error, ErrorPrinter},
};

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

fn main() {
    for input in repl(">>> ") {
        let program = match parse_program(&input, lex(&input)) {
            Ok(program) => program,
            Err(error) => {
                print_error(&error);
                continue;
            }
        };
        let val = match eval_program(&program) {
            Ok(val) => val,
            Err(error) => {
                print_error(&error);
                continue;
            }
        };
        println!("{val}");
    }
}

fn print_error(error: &Error) {
    println!("Error:\n  {}", ErrorPrinter(&error))
}
