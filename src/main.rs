mod lexer;
mod token;

fn main() {
    let mut lexer = lexer::Lexer::new("{ \"key\": 123 }".to_string());
    lexer.tokenize();
    for token in lexer.tokens {
        println!("{:?}", token);
    }
}
