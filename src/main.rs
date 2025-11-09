use std::io::{self, Read};

mod lexer;
mod token;

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let mut lexer = lexer::Lexer::new(buf);
    lexer.tokenize();
    for token in lexer.tokens {
        println!("{:?}", token);
    }

    Ok(())
}
