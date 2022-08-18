use bool_eval::lexer::lex;

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
        println!("Parsing `{input}`...");
        for token in lex(&input) {
            let text = &input[token.span.range()];
            let kind = token.kind;
            println!("`{text}` :: {kind:?}");
        }
    }
}
