use crate::core::data::{AnyData, DataType};
use crate::core::instructions::Operations;
use crate::core::register::Register;

use super::converter::Converter;
use super::pretokenizer::PreTokenizer;
use super::token::TokenType;

pub struct Tokenizer {
    pre: PreTokenizer,
    cursor: usize,
    tokens: Vec<TokenType>
}

impl Tokenizer {
    pub fn new(pre_tokenizer: PreTokenizer) -> Self {
        Tokenizer { pre: pre_tokenizer, cursor: 0, tokens: Vec::new() }
    }

    pub fn tokenize(&mut self) -> &Vec<TokenType> {
        
        while self.cursor < self.pre.tokens.len() {
            let token = self.next();
            
            self.tokens.push(token);
        }
        
        &self.tokens
    }

    fn next(&mut self) -> TokenType {
        let tkn = &self.pre.tokens[self.cursor];
        
        self.cursor += 1;

        if Register::is_register(tkn) {
            TokenType::REGISTER(Register::RAX)
        } else if Operations::is_operation(tkn) {
            TokenType::INSTRUCTION(Operations::MOV)
        } else if Tokenizer::is_comment(tkn) {
            TokenType::COMMENT(String::from(tkn))
        } else {

            match Converter::to_int(tkn) {
                Some(value) => return TokenType::DATA(DataType::Int64, AnyData::from(value)),
                None => {},
            }
            
            TokenType::DATA(DataType::String, AnyData::from(tkn))
        }
    }

    fn is_comment(token: &String) -> bool {
        token.starts_with(";")
    }
}