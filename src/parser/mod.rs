use crate::{
    ast::{Ast, Expresion, LetStatement, Statement},
    lexer::{
        Lexer,
        token::{Token, TokenKind},
    },
};
use std::iter::Peekable;

pub struct Parser<'a> {
    pub tokens: Peekable<Lexer<'a>>,
    pub ast: Ast,
}

impl<'a> Parser<'a>{
    pub fn new(lexer: Lexer) -> Parser{
        Parser{
            tokens : lexer.peekable(),
            ast : Ast{
                statements: Vec<Statement>::new()
            }
        }
    }

    pub fn parse(&self){
        todo!("implement parse method");
    }

    pub fn parseLetStatement(&mut self) -> Option<LetStatement>{

        let identifier : Token;
        let mut exepresion : Vec<Token> = Vec::new();
        let peek : &Token = self.tokens.peek()?;
        if peek.kind == TokenKind::LET {
            self.tokens.next();
        }
        else{
            panic!("wrong token");
        }

        let peek : &Token = self.tokens.peek()?;
        if peek.kind == TokenKind::IDENT {
            identifier = self.tokens.next()?;
        }
        else{
            panic!("wrong token");
        }

        let peek : &Token = self.tokens.peek()?;
        if peek.kind == TokenKind::Equal {
            self.tokens.next();
        }
        else{
            panic!("wrong token");
        }

        loop {

            let cur : Token = self.tokens.next()?;
            if cur.kind == TokenKind::SemiColon{
                break;
            }
            else{
                exepresion.push(cur);
            }
        }


        let statement = LetStatement::new(
                identifier,
                Expresion::new(exepresion)
        );
        return Some(statement);

    }

}
