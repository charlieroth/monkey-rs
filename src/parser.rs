use crate::{
    ast::{self},
    lexer::Lexer,
    token::Token,
};

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub curr_token: Token,
    pub peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            curr_token: Token::Eof,
            peek_token: Token::Eof,
        };

        parser.next_token();
        parser.next_token();

        return parser;
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program: ast::Program = vec![];
        while self.curr_token != Token::Eof {
            match self.parse_statement() {
                Some(statement) => program.push(statement),
                None => {}
            }
            self.next_token();
        }

        return program;
    }

    pub fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.curr_token {
            Token::Let => self.parse_let_statement(),
            _ => return None,
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        // next token should be Token::Ident
        match &self.peek_token {
            Token::Ident(_) => self.next_token(),
            _ => return None,
        };

        // parse ast::Expr::Ident
        let ident = match self.parse_ident() {
            Some(ident) => ident,
            None => return None,
        };

        // next token should be Token::Assign, no need to parse anything
        if self.expect_peek_token(Token::Assign) == false {
            return None;
        }

        // skip Token::Assign
        self.next_token();

        // parse ast::Expr within ast::Statement::Let
        let expression = match self.parse_expression() {
            Some(expr) => expr,
            None => return None,
        };

        // next token should be Token::Semicolon
        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        // all conditions for a ast::Statement::Let are met
        // return parsed ast::Statement::Let
        Some(ast::Statement::Let(ident, expression))
    }

    pub fn parse_expression(&mut self) -> Option<ast::Expr> {
        match self.curr_token {
            Token::Ident(_) => self.parse_ident_expression(),
            Token::Int(_) => self.parse_int_expression(),
            _ => return None,
        }
    }

    fn parse_ident_expression(&mut self) -> Option<ast::Expr> {
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
        match self.parse_int() {
            Some(int) => Some(ast::Expr::Literal(int)),
            None => None,
        }
    }

    fn parse_int(&mut self) -> Option<ast::Literal> {
        match self.curr_token {
            Token::Int(int) => Some(ast::Literal::Int(int)),
            _ => None,
        }
    }

    pub fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next();
    }

    pub fn peek_token_is(&mut self, tok: &Token) -> bool {
        return self.peek_token == *tok;
    }

    pub fn expect_peek_token(&mut self, tok: Token) -> bool {
        if self.peek_token_is(&tok) {
            self.next_token();
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast;

    use super::*;

    #[test]
    fn initialization() {
        let input = "
        let x = 5;
        ";
        let parser = Parser::new(Lexer::new(input));
        assert_eq!(parser.curr_token, Token::Let);
        assert_eq!(parser.peek_token, Token::Ident(String::from("x")));
    }

    #[test]
    fn parse_let_statement() {
        let input = "
        let x = 5;
        ";
        let mut parser = Parser::new(Lexer::new(input));
        let statement = parser.parse_let_statement();
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
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse_program();

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
}
