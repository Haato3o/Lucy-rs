use lucy::parser::pretokenizer::PreTokenizer;
use lucy::parser::tokenizer::{Tokenizer};

fn main() {
    let input = 
    "
        mov eax, 10
        mov ebx, 20
        mov edx, 0b0010101
        mov ecx, 0x10 ;should be 16
        add eax, ebx
        inc ebx
        add eax, ebx
        dmp eax
        dmp 'c'
        ret
        mov ebx, \"Hello World\" ; This is a comment!
    ";

    let mut pretokenizer = PreTokenizer::new(String::from(input));
    let _ = pretokenizer.parse();
    let mut tokenizer = Tokenizer::new(pretokenizer);

    for string in tokenizer.tokenize() {
        println!("{}", string);
    }
}
