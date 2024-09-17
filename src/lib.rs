#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,
    Ident(String),
    Int(String),
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Func,
    Let,
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();
        return lexer;
    }

    pub fn next(&mut self) -> Token {
        self.skip_whitespace();

        match self.ch {
            b'=' => {
                self.read_char();
                return Token::Assign;
            }
            b';' => {
                self.read_char();
                return Token::Semicolon;
            }
            b'(' => {
                self.read_char();
                return Token::Lparen;
            }
            b')' => {
                self.read_char();
                return Token::Rparen;
            }
            b',' => {
                self.read_char();
                return Token::Comma;
            }
            b'+' => {
                self.read_char();
                return Token::Plus;
            }
            b'{' => {
                self.read_char();
                return Token::Lbrace;
            }
            b'}' => {
                self.read_char();
                return Token::Rbrace;
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                return self.read_identifier();
            }
            b'0'..=b'9' => {
                return self.read_number();
            }
            0 => {
                self.read_char();
                return Token::Eof;
            }
            _ => {
                self.read_char();
                return Token::Illegal;
            }
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        loop {
            match self.ch {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }

        let literal = &self.input[start..self.position];
        match literal {
            "fn" => Token::Func,
            "let" => Token::Let,
            _ => Token::Ident(String::from(literal))
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        loop {
            match self.ch {
                b'0'..=b'9' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }

        let literal = &self.input[start..self.position];
        return Token::Int(String::from(literal));
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
        loop {
            match self.ch {
                b' ' | b'\n' | b'\t' | b'\r' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_symbols() {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input.to_string());
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
        let mut lexer = Lexer::new(input.to_string());
        let expected_tokens = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int("10".to_string()),
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
        let mut lexer = Lexer::new(input.to_string());
        let expected_tokens = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int("10".to_string()),
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
}
