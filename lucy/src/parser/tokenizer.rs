use crate::core::data::{AnyData, DataType};
use crate::core::instructions::Operations;
use crate::core::register::Register;
use crate::logger::log::{Log, Logger};

use super::converter::Converter;
use super::pretokenizer::PreTokenizer;
use super::token::TokenType;

pub struct Tokenizer {
    pre: PreTokenizer,
    cursor: usize,
    tokens: Vec<TokenType>,
    checkpoints: Vec<String>,
    last_operation: Option<Operations>
}

impl Tokenizer {
    pub fn new(pre_tokenizer: PreTokenizer) -> Self {
        Tokenizer { pre: pre_tokenizer, cursor: 0, tokens: Vec::new(), checkpoints: Vec::new(), last_operation: None }
    }

    pub fn tokenize(&mut self) -> &Vec<TokenType> {
        self.get_checkpoints();

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
            TokenType::REGISTER(Register::from_string(tkn).unwrap())
        } else if Operations::is_operation(tkn) {
            self.last_operation = Operations::from_string(tkn);
            TokenType::INSTRUCTION(Operations::from_string(tkn).unwrap())
        } else if Tokenizer::is_checkpoint(tkn) {
            TokenType::CHECKPOINT(String::from(tkn))
        } else if Tokenizer::is_comment(tkn) {
            TokenType::COMMENT(String::from(tkn))
        } else if self.is_goto( tkn) {
            TokenType::GOTO(String::from(tkn))
        } else {

            match Converter::to_int(tkn) {
                Some(value) => return TokenType::DATA(DataType::Int64, AnyData::from(value)),
                None => {},
            }
            
            TokenType::DATA(DataType::String, AnyData::from(tkn))
        }
    }

    fn get_checkpoints(&mut self) {
        for tkn in &self.pre.tokens {
            if Tokenizer::is_checkpoint(tkn) {
                self.checkpoints.push(String::from(tkn));
            }
        }
    }

    fn is_goto(&self, token: &String) -> bool {
        let checkpoint_name = format!(":{}", token);
        
        let jmp_ops = vec![Operations::JMP, Operations::JNE, Operations::JE];

        match self.last_operation {
            Some(op) => self.checkpoints.contains(&checkpoint_name) && jmp_ops.contains(&op),
            None => false,
        }
    } 

    fn is_comment(token: &String) -> bool {
        token.starts_with(";")
    }

    fn is_checkpoint(token: &String) -> bool {
        token.starts_with(":")
    }
}