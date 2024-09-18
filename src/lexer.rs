use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();
        lexer
    }

    pub fn next(&mut self) -> Token {
        self.skip_whitespace();

        match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    Token::Eq
                } else {
                    self.read_char();
                    Token::Assign
                }
            }
            b';' => {
                self.read_char();
                Token::Semicolon
            }
            b'(' => {
                self.read_char();
                Token::Lparen
            }
            b')' => {
                self.read_char();
                Token::Rparen
            }
            b',' => {
                self.read_char();
                Token::Comma
            }
            b'+' => {
                self.read_char();
                Token::Plus
            }
            b'-' => {
                self.read_char();
                Token::Minus
            }
            b'/' => {
                self.read_char();
                Token::Slash
            }
            b'*' => {
                self.read_char();
                Token::Asterisk
            }
            b'<' => {
                self.read_char();
                Token::Lt
            }
            b'>' => {
                self.read_char();
                Token::Gt
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    Token::NotEq
                } else {
                    self.read_char();
                    Token::Bang
                }
            }
            b'{' => {
                self.read_char();
                Token::Lbrace
            }
            b'}' => {
                self.read_char();
                Token::Rbrace
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.read_identifier(),
            b'0'..=b'9' => self.read_number(),
            0 => {
                self.read_char();
                Token::Eof
            }
            _ => {
                self.read_char();
                Token::Illegal
            }
        }
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while let b'a'..=b'z' | b'A'..=b'Z' | b'_' = self.ch {
            self.read_char();
        }

        let literal = &self.input[start..self.position];
        match literal {
            "fn" => Token::Func,
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "true" => Token::True,
            "false" => Token::False,
            "return" => Token::Return,
            _ => Token::Ident(String::from(literal)),
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        while let b'0'..=b'9' = self.ch {
            self.read_char();
        }

        let literal = &self.input[start..self.position];
        Token::Int(literal.parse().unwrap())
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while let b' ' | b'\n' | b'\t' | b'\r' = self.ch {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_symbols() {
        let input = "
        =+(){},;
        ";
        let mut lexer = Lexer::new(input);
        let expected_tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,
        ];

        for expected in expected_tokens {
            let actual = lexer.next();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn variables() {
        let input = "
        let five = 5;
        let ten = 10;
        ";
        let mut lexer = Lexer::new(input);
        let expected_tokens = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Eof,
        ];

        for expected in expected_tokens {
            let actual = lexer.next();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn functions() {
        let input = "
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        ";
        let mut lexer = Lexer::new(input);
        let expected_tokens = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Func,
            Token::Lparen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::Lparen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::Rparen,
            Token::Semicolon,
            Token::Eof,
        ];

        for expected in expected_tokens {
            let actual = lexer.next();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn more_symbols() {
        let input = "
        !-/*5;
        5 < 10 > 5;
        ";
        let mut lexer = Lexer::new(input);
        let expected_tokens = vec![
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Gt,
            Token::Int(5),
            Token::Semicolon,
            Token::Eof,
        ];

        for expected in expected_tokens {
            let actual = lexer.next();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn if_else_statements() {
        let input = "
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        ";
        let mut lexer = Lexer::new(input);
        let expected_tokens = vec![
            Token::If,
            Token::Lparen,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::Rbrace,
            Token::Eof,
        ];

        for expected in expected_tokens {
            let actual = lexer.next();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn equal_not_equal() {
        let input = "
        10 == 10;
        10 != 9;
        ";
        let mut lexer = Lexer::new(input);
        let expected_tokens = vec![
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEq,
            Token::Int(9),
            Token::Semicolon,
            Token::Eof,
        ];

        for expected in expected_tokens {
            let actual = lexer.next();
            assert_eq!(expected, actual);
        }
    }
}
