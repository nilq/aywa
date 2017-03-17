pub mod tokens;
pub mod lexer;
pub mod ast;

#[cfg(test)]
mod tests {
    #[test]
    fn lexer_tokenize() {
        use syntax::lexer::Lexer;

        let mut lexer = Lexer::new();

        lexer.tokenize(String::from("
            define fib_sum(a -> int, b -> int) -> int
                if a + b <= 2
                    return c
                end

                fib_sum(a, b - 1) + fib_sum(a, b - 2)
            end
        "));

        assert!(lexer.get_tokens().len() != 0)
    }

    #[test]
    fn parser_tokenize() {
        use syntax::lexer::Lexer;
        use syntax::ast::Parser;

        let mut lexer = Lexer::new();

        lexer.tokenize(String::from("
            foo = \"hey aywa\"
            foo = 22.2

            define bar(a, b) do
                foo = 2
            end

            sum = 1337 - 2 - 10.2 * 100
        "));

        let mut parser = Parser::from(lexer);

        let stack = parser.parse_full();
    }
}