use crate::compiler::lexer::{Lexer, Token};

pub fn minify(source_bundle: &String) -> String {
    let mut lexer = Lexer::new(source_bundle);
    let mut result = String::new();
    let mut last_token: Option<Token> = None;

    loop {
        let token = lexer.next_token();

        if token == Token::Eof {
            break;
        }

        match &token {
            // Skip comments and whitespaces
            Token::Comment(_) | Token::Whitespace => {}

            Token::Identifier(cur) | Token::Keyword(cur) | Token::Number(cur) => {
                // Add a space if the last token was also a word/number/string
                match &last_token {
                    Some(Token::Identifier(_))
                    | Some(Token::Keyword(_))
                    | Some(Token::Number(_))
                    | Some(Token::StringLiteral(_)) => result.push(' '),
                    _ => {}
                }

                result.push_str(cur);
                last_token = Some(token);
            }

            Token::StringLiteral(content) => {
                // Add a space if the last token was also a word/number/string
                match &last_token {
                    Some(Token::Identifier(_))
                    | Some(Token::Keyword(_))
                    | Some(Token::Number(_))
                    | Some(Token::StringLiteral(_)) => result.push(' '),
                    _ => {}
                }

                // Format the string literal properly
                if content.contains('\n') || content.contains('"') || content.contains('\'') {
                    // Use long bracket format for multi-line strings or strings with quotes
                    result.push_str("[[");
                    result.push_str(content);
                    result.push_str("]]");
                } else {
                    // Use double quotes for simple strings
                    result.push('"');
                    result.push_str(content);
                    result.push('"');
                }
                last_token = Some(token);
            }

            Token::Symbol(ch) => {
                // No space around most symbols
                result.push(*ch);
                last_token = Some(token);
            }

            _ => {}
        }
    }

    result
}
