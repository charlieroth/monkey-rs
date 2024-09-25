use core::fmt;

use crate::{
    ast::{self},
    lexer::Lexer,
    token::Token,
};

#[derive(Debug, Clone)]
pub enum ParseErrorKind {
    UnexpectedToken,
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseErrorKind::UnexpectedToken => write!(f, "Unexpected Token"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    kind: ParseErrorKind,
    msg: String,
}

impl ParseError {
    fn new(kind: ParseErrorKind, msg: String) -> Self {
        ParseError { kind, msg }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

pub type ParseErrors = Vec<ParseError>;

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub curr_token: Token,
    pub peek_token: Token,
    pub errors: ParseErrors,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            curr_token: Token::Eof,
            peek_token: Token::Eof,
            errors: vec![],
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn get_errors(&mut self) -> ParseErrors {
        self.errors.clone()
    }

    pub fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next();
    }

    pub fn curr_token_is(&mut self, tok: Token) -> bool {
        self.curr_token == tok
    }

    pub fn peek_token_is(&mut self, tok: &Token) -> bool {
        self.peek_token == *tok
    }

    pub fn expect_peek_token(&mut self, tok: Token) -> bool {
        if self.peek_token_is(&tok) {
            self.next_token();
            return true;
        } else {
            self.error_next_token(tok);
            return false;
        }
    }

    fn error_next_token(&mut self, tok: Token) {
        self.errors.push(ParseError::new(
            ParseErrorKind::UnexpectedToken,
            format!(
                "expected next token to be {:?}, got {:?} instead",
                tok, self.peek_token
            ),
        ));
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program: ast::Program = vec![];

        while !self.curr_token_is(Token::Eof) {
            match self.parse_statement() {
                Some(statement) => program.push(statement),
                None => {}
            }
            self.next_token();
        }

        program
    }

    pub fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.curr_token {
            Token::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        match &self.peek_token {
            Token::Ident(_) => self.next_token(),
            _ => {
                self.error_next_token(self.peek_token.clone());
                return None;
            }
        };

        let name = match self.parse_ident() {
            Some(name) => name,
            None => return None,
        };

        if !self.expect_peek_token(Token::Assign) {
            return None;
        }

        self.next_token();

        let expr = match self.parse_expression() {
            Some(expr) => expr,
            None => return None,
        };

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Some(ast::Statement::Let(name, expr))
    }

    pub fn parse_expression(&mut self) -> Option<ast::Expr> {
        match self.curr_token {
            Token::Ident(_) => self.parse_ident_expression(),
            Token::Int(_) => self.parse_int_expression(),
            _ => None,
        }
    }

    pub fn parse_ident_expression(&mut self) -> Option<ast::Expr> {
        match self.parse_ident() {
            Some(ident) => Some(ast::Expr::Ident(ident)),
            None => None,
        }
    }

    pub fn parse_ident(&mut self) -> Option<ast::Ident> {
        match self.curr_token {
            Token::Ident(ref mut ident) => Some(ast::Ident(ident.clone())),
            _ => None,
        }
    }

    fn parse_int_expression(&mut self) -> Option<ast::Expr> {
        self.parse_int()
    }

    fn parse_int(&mut self) -> Option<ast::Expr> {
        match self.curr_token {
            Token::Int(int) => Some(ast::Expr::Literal(ast::Literal::Int(int))),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast;

    use super::*;

    #[test]
    fn initialization() {
        let input = r#"
let x = 5;
        "#;

        let parser = Parser::new(Lexer::new(input));

        assert_eq!(parser.curr_token, Token::Let);
        assert_eq!(parser.peek_token, Token::Ident(String::from("x")));
    }

    #[test]
    fn parse_let_statement() {
        let input = "let x = 5;";

        let mut parser = Parser::new(Lexer::new(input));
        let statement = parser.parse_let_statement();

        assert_eq!(0, parser.get_errors().len());
        assert_eq!(
            statement,
            Some(ast::Statement::Let(
                ast::Ident(String::from("x")),
                ast::Expr::Literal(ast::Literal::Int(5))
            ))
        )
    }

    #[test]
    fn let_statements() {
        let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;"#;

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse_program();

        assert_eq!(0, parser.get_errors().len());
        assert_eq!(
            vec![
                ast::Statement::Let(
                    ast::Ident(String::from("x")),
                    ast::Expr::Literal(ast::Literal::Int(5)),
                ),
                ast::Statement::Let(
                    ast::Ident(String::from("y")),
                    ast::Expr::Literal(ast::Literal::Int(10)),
                ),
                ast::Statement::Let(
                    ast::Ident(String::from("foobar")),
                    ast::Expr::Literal(ast::Literal::Int(838383)),
                ),
            ],
            program
        );
    }

    #[test]
    fn let_statement_with_errors() {
        let input = r#"
let x 5;
let = 10;
let 838383;"#;

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse_program();
        assert_eq!(3, parser.get_errors().len());
    }
}
