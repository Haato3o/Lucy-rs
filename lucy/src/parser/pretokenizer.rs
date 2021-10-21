use std::string::String;

pub struct PreTokenizer {
    stream: Vec<char>,
    pub tokens: Vec<String>
}
#[derive(Debug)]
pub enum PreTokenizerError {
    EndOfFile,
    NotAChar
}

impl PreTokenizer {

    pub fn new(string: String) -> Self {
        let vec: Vec<char> = string
                            .chars()
                            .rev()
                            .take(string.len())
                            .collect();
        
        PreTokenizer {
            stream: vec,
            tokens: Vec::new(),
        }

    }

    pub fn parse(&mut self) -> &Vec<String> { 

        while self.stream.len() > 0 {
            match self.next() {
                Some(token) => self.tokens.push(token),
                None => continue,
            }
        }

        &self.tokens
    }

    fn next(&mut self) -> Option<String> {

        let chr = self.read().unwrap();
        
        match chr {
            '"' => {
                Some(self.read_string())
            },
            ';' => {
                Some(self.read_comment())
            },
            '\'' => {
                Some(self.read_char())
            },
            _ => {
                let raw = self.read_raw();

                if raw.is_empty() {
                    return None;
                }

                Some(raw)
            }
        }

    }

    fn read(&self) -> Result<char, PreTokenizerError> {
        
        if self.stream.len() <= 0 {
            return Err(PreTokenizerError::EndOfFile);
        }

        let ch = self.stream[self.stream.len() - 1];
                
        Ok(ch)
    }

    // fn peek(&self) -> Result<char, PreTokenizerError> {
    //     if self.stream.len() - 1 <= 0 {
    //         return Err(PreTokenizerError::EndOfFile);
    //     }

    //     let ch = self.stream[self.stream.len() - 2];

    //     Ok(ch)
    // }

    fn consume_char(&mut self) -> Result<char, PreTokenizerError> {
        
        if self.stream.len() <= 0 {
            return Err(PreTokenizerError::EndOfFile);
        }
        
        let chr = self.stream.pop().unwrap();
        
        Ok(chr)
    }

    // fn consume_chars(&mut self, chars: Vec<char>) {

    //     while self.stream.len() > 0 {
    //         match self.consume_char() {
    //             Ok(char) => {
    //                 if chars.contains(&char) {
    //                     break;
    //                 }
    //             },
    //             Err(_) => break,
    //         }
    //     }

    // }

    fn read_raw(&mut self) -> String {
        let mut raw = String::new();
        let discard = [' ', ',', '\n', '\r', '\t'];

        let mut char = self.consume_char().unwrap();
        while self.stream.len() > 0 && !discard.contains(&char) {
            raw.push(char);
            char = self.consume_char().unwrap();
        }
        
        raw
    }

    fn read_comment(&mut self) -> String {
        let mut comment = String::new();
        let discard = ['\n', '\r'];
        let mut chr = self.consume_char().unwrap();

        while self.stream.len() > 0 && !discard.contains(&chr) {
            comment.push(chr);
            chr = self.consume_char().unwrap();
        }

        comment
    }

    fn read_string(&mut self) -> String {
        let mut string = String::new();

        let mut stack = vec![self.consume_char().unwrap()];
        
        while stack.len() > 0 {
            let chr = self.consume_char().unwrap();

            if chr == '"' {
                stack.pop();
                break;
            }
            string.push(chr);
        }

        string
    }

    fn read_char(&mut self) -> String {
        // TODO: Fix this to only read a single character
        let mut string = String::new();

        let mut stack = vec![self.consume_char().unwrap()];
        
        while stack.len() > 0 {
            let chr = self.consume_char().unwrap();

            if chr == '\'' {
                stack.pop();
                continue;
            }

            string.push(chr);
        }

        string
    }
}