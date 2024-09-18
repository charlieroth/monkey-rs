use crate::{
    ast::{self},
    lexer::Lexer,
    token::Token,
};

type ParseError = String;
type ParseErrors = Vec<ParseError>;

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

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program: ast::Program = vec![];
        while self.curr_token != Token::Eof {
            match self.parse_statement() {
                Ok(statement) => program.push(statement),
                Err(e) => self.errors.push(e),
            }
            self.next_token();
        }

        program
    }

    pub fn parse_statement(&mut self) -> Result<ast::Statement, ParseError> {
        match self.curr_token {
            Token::Let => self.parse_let_statement(),
            _ => Err(format!(
                "Unsupported statement token: {:?}",
                self.curr_token
            )),
        }
    }

    pub fn parse_let_statement(&mut self) -> Result<ast::Statement, ParseError> {
        self.next_token();

        let mut identifier_name = "".to_string();
        match &self.curr_token {
            Token::Ident(name) => {
                identifier_name = name.to_string();
            }
            _ => return Err(format!("Expected identifier, got: {:?}", self.curr_token)),
        };

        self.expect_peek_token(&Token::Assign)?;
        self.next_token();

        let expression = match self.parse_expression() {
            Ok(expression) => expression,
            Err(e) => return Err(e),
        };

        self.expect_peek_token(&Token::Semicolon)?;
        self.next_token();

        Ok(ast::Statement::Let(ast::Ident(identifier_name), expression))
    }

    pub fn parse_expression(&mut self) -> Result<ast::Expr, ParseError> {
        match self.curr_token {
            Token::Ident(_) => self.parse_ident_expression(),
            Token::Int(_) => self.parse_int_expression(),
            _ => Err(format!(
                "Unsupported token for expression parsing {:?}",
                self.curr_token
            )),
        }
    }

    fn parse_ident_expression(&mut self) -> Result<ast::Expr, ParseError> {
        self.parse_ident()
    }

    pub fn parse_ident(&mut self) -> Result<ast::Expr, ParseError> {
        match self.curr_token {
            Token::Ident(ref mut ident) => Ok(ast::Expr::Ident(ast::Ident(ident.clone()))),
            _ => Err(format!(
                "Expected identifier token, got: {:?}",
                self.curr_token
            )),
        }
    }

    fn parse_int_expression(&mut self) -> Result<ast::Expr, ParseError> {
        self.parse_int()
    }

    fn parse_int(&mut self) -> Result<ast::Expr, ParseError> {
        match self.curr_token {
            Token::Int(int) => Ok(ast::Expr::Literal(ast::Literal::Int(int))),
            _ => Err(format!("Expected int tokne, got: {:?}", self.curr_token)),
        }
    }

    pub fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next();
    }

    pub fn peek_token_is(&mut self, tok: &Token) -> bool {
        self.peek_token == *tok
    }

    pub fn expect_peek_token(&mut self, tok: &Token) -> Result<(), ParseError> {
        self.next_token();
        if self.curr_token == *tok {
            Ok(())
        } else {
            Err(format!(
                "Expected token: {:?}, got {:?}",
                tok, self.curr_token
            ))
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
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ";

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
    fn let_statement_with_error() {
        let input = "
        let = 10;
        ";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse_program();
        // println!("{:?}", program);
        // println!("{:?}", parser.get_errors());
        // assert_eq!(true, true);
        assert_eq!(2, parser.get_errors().len());
    }
}
