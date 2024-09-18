#[derive(Debug, Clone, PartialEq)]
pub struct Ident(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Ident(Ident),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(Ident, Expr),
}

pub type Block = Vec<Statement>;

pub type Program = Block;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {}
}
