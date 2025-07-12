#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(String),
    StringLiteral(String),
    Symbol(char),
    Keyword(String),
    Comment(String),
    Whitespace,
    Eof,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    fn next_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn bump(&mut self) -> Option<char> {
        if let Some(c) = self.next_char() {
            self.position += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.input[self.position..].chars().nth(offset)
    }

    fn consume_while<F>(&mut self, cond: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while let Some(c) = self.next_char() {
            if cond(c) {
                result.push(c);
                self.bump();
            } else {
                break;
            }
        }
        result
    }

    fn consume_long_bracket_string(&mut self) -> String {
        // We're already at the first '[', consume it
        self.bump();

        // Count the number of '=' characters between the brackets
        let mut level = 0;
        while self.next_char() == Some('=') {
            level += 1;
            self.bump();
        }

        // Consume the second '['
        if self.next_char() == Some('[') {
            self.bump();
        }

        // Skip the first newline if present (Lua behavior)
        if self.next_char() == Some('\n') {
            self.bump();
        }

        let mut content = String::new();

        // Look for the closing bracket with the same level
        while let Some(c) = self.next_char() {
            if c == ']' {
                // Check if this is the closing bracket sequence
                let mut temp_pos = self.position;
                let mut closing_level = 0;

                // Skip the first ']'
                temp_pos += 1;

                // Count '=' characters
                while temp_pos < self.input.len() {
                    if let Some(ch) = self.input[temp_pos..].chars().next() {
                        if ch == '=' {
                            closing_level += 1;
                            temp_pos += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                // Check if we have the final ']' and the levels match
                if temp_pos < self.input.len()
                    && self.input[temp_pos..].chars().next() == Some(']')
                    && closing_level == level
                {
                    // Found the closing bracket, consume it
                    self.bump(); // consume first ']'
                    for _ in 0..level {
                        self.bump(); // consume '=' characters
                    }
                    self.bump(); // consume final ']'
                    break;
                }
            }

            content.push(c);
            self.bump();
        }

        content
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(c) = self.next_char() {
            return match c {
                // Whitespace
                c if c.is_whitespace() => {
                    self.consume_while(|ch| ch.is_whitespace());
                    return Token::Whitespace;
                }

                // Long bracket strings
                '[' => {
                    if self.peek(1) == Some('[')
                        || (self.peek(1) == Some('=')
                            && self.input[self.position + 1..].chars().any(|ch| ch == '['))
                    {
                        let content = self.consume_long_bracket_string();
                        return Token::StringLiteral(content);
                    } else {
                        self.bump();
                        return Token::Symbol('[');
                    }
                }

                // Comments and minus symbol
                '-' => {
                    if self.peek(1) == Some('-') {
                        self.bump();
                        self.bump();
                        if self.peek(0) == Some('[') && self.peek(1) == Some('[') {
                            self.bump();
                            self.bump();
                            let content = self.consume_while(|c| !c.to_string().ends_with("]]"));
                            self.consume_while(|c| c != ']'); // consume until ]]
                            self.bump();
                            self.bump();
                            Token::Comment(content)
                        } else {
                            let content = self.consume_while(|c| c != '\n');
                            Token::Comment(content)
                        }
                    } else {
                        self.bump();
                        Token::Symbol('-')
                    }
                }

                // Keywords and identifiers
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let ident = self.consume_while(|ch| ch.is_ascii_alphanumeric() || ch == '_');
                    return match ident.as_str() {
                        "if" | "then" | "else" | "elseif" | "while" | "for" | "function"
                        | "local" | "return" | "do" | "end" | "break" | "not" | "and" | "or"
                        | "in" => Token::Keyword(ident),
                        _ => Token::Identifier(ident),
                    };
                }

                // Numbers
                c if c.is_ascii_digit() => {
                    let num = self.consume_while(|ch| ch.is_ascii_digit() || ch == '.');
                    return Token::Number(num);
                }

                '"' | '\'' => {
                    let quote = self.bump().unwrap();
                    let mut string = String::new();
                    while let Some(ch) = self.bump() {
                        if ch == quote {
                            break;
                        } else if ch == '\\' {
                            if let Some(esc) = self.bump() {
                                string.push(esc); // basic escape handling
                            }
                        } else {
                            string.push(ch);
                        }
                    }
                    return Token::StringLiteral(string);
                }

                // Symbols (removed '[' since it's handled above)
                c if "{}]();:,+*/%^=#<>".contains(c) => {
                    self.bump();
                    return Token::Symbol(c);
                }

                _ => {
                    self.bump();
                    return Token::Symbol(c);
                }
            };
        }
        Token::Eof
    }
}
