use crate::token::Token;

// AST
// ----------------------------------------------------
#[derive(Clone, Debug)]
pub struct AST {
    pub token           : Token,
    pub children        : Vec<AST>,
}

impl AST {
    // new
    // return : AST
    pub fn new(token: Token, children: Vec<AST>) -> AST {
        AST {
            token       : token,
            children    : children
        }
    }
}
// ----------------------------------------------------