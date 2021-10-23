use lucy::logger::log::{Log, Logger};
use lucy::parser::pretokenizer::PreTokenizer;
use lucy::parser::tokenizer::Tokenizer;
use lucy::compiler::compiler::LucyCompiler;

fn main() {
    let input = 
    "
        jmp main
        dmp \"Hello world!\"
        :main
        mov edi, 0
        :loop
        
    ";

    let mut pretokenizer = PreTokenizer::new(String::from(input));
    let _ = pretokenizer.parse();
    let mut tokenizer = Tokenizer::new(pretokenizer);
    let tokens = tokenizer.tokenize();

    // for tkn in tokens {
    //     Log::info(format!("{}", tkn));
    // }

    let compiled = LucyCompiler::compile(tokens);

    for c in compiled {
        Log::info(format!("{:?} {:?}", c.instruction, c.arguments[0].typ));
    }

}
