use bool_eval::{
    ast::Program,
    lexer::lex,
    parser::parse,
    util::{ErrorPrinter, ExprTreePrinter},
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
        println!("------");
        println!("Parsing `{input}`...\n");
        let tokens = lex(&input);
        match parse(&input, tokens) {
            Ok(Program { expr, args }) => {
                println!("{}\n", ExprTreePrinter(&expr));
                println!("=== ARGS ===\n{args:#?}");
            }
            Err(error) => println!("Error:\n  {}", ErrorPrinter(&error)),
        }
        println!("------");
    }
}
