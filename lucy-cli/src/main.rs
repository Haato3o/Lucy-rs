use std::io::{self, BufRead, Read, Write};
use std::process::exit;

use lucy::logger::log::{Log, Logger};
use lucy::parser::pretokenizer::PreTokenizer;
use lucy::parser::tokenizer::Tokenizer;
use lucy::compiler::compiler::LucyCompiler;
use lucy::vm::machine::LucyMachine;

const VERSION: &str = "1.0.0";

fn main() {
    // let input = 
    // "
    //     jmp main
    //     dmp \"Hello, World!\"
    //     :main
    //     mov eax, 10
    //     mov ebx, 20
    //     add eax, ebx
    //     dmp eax
    //     dmp ebx
    // ";

    // let mut pretokenizer = PreTokenizer::new(String::from(input));
    // let _ = pretokenizer.parse();
    // let mut tokenizer = Tokenizer::new(pretokenizer);
    // let tokens = tokenizer.tokenize();

    // for tkn in tokens {
    //     Log::info(format!("{}", tkn));
    // }

    // let compiled = LucyCompiler::compile(tokens);

    // for c in &compiled {
    //     Log::info(format!("{:?} {:?}", c.op_code, c.arguments[0].data));
    // }

    // let mut vm = LucyMachine::new(compiled);
    // vm.run();
    
    println!("Lucy interpreter @ Version: {}", VERSION);

    let mut program_buffer = String::new();
    
    let mut vm = LucyMachine::new(Vec::new());
    let mut prefix = ">>> ";
    loop {
        let mut buffer = String::new();

        io::stdout().write(prefix.as_bytes())
            .expect("failed to write to stdout");
        io::stdout().flush()
            .expect("failed to flush stdout");

        io::stdin().read_line(&mut buffer)
            .expect("failed to read stdin");

        buffer = buffer[0..buffer.len() - 2].to_string();

        if buffer == "exit" {
            exit(0);
        }

        if buffer.len() == 0 {
            program_buffer.push('\n');

            let mut pretokens = PreTokenizer::new(String::from(&program_buffer));
            pretokens.parse();
            let mut tokens = Tokenizer::new(pretokens);
            let compiled = LucyCompiler::compile(tokens.tokenize());
            vm.run_next(compiled);

            program_buffer = String::new();
            prefix = ">>> ";
        } else {
            prefix = "... ";
            program_buffer.push('\n');
            program_buffer.push_str(&buffer.as_str())
        }
    }
}
