# Lucy

Lucy is an interpreted programming language based on Assembly x86.

> Note: Lucy is still in development and not every instruction is implemented yet.

# Examples

You can find some working examples [here]("https://github.com/Haato3o/Lucy-rs/tree/main/examples")

## Hello World

```asm
; It supports comments!
mov rax, "Hello, Grace :)"
dmp rax

; >>> "Hello, Grace :)"
```

## Loops

```asm
jmp main

dmp "This line will be skippped"

:main
    mov rdi, 0
    mov rbx, 1000

:_loop
    cmp rdi, rbx
    add rdi, 1
    dmp rdi
    jne _loop

; >>> 1
; >>> 2
; >>> [...]
; >>> 999
; >>> 1000
```

## Equality

```asm
:main
    mov rax, "Hello, world"
    mov rcx, 0xC0FFEE
    mov rdx, 0b110000001111111111101110

    cmp rax, "Goodbye, world"
    je string_equality_true
    jne string_equality_false
    :_ret_string_equality ; the ret operator isn't supported yet

    cmp rcx, rdx
    je int64_equality_true
    jne int64_equality_false

:string_equality_true
    dmp "Strings are equal!"
    jmp _ret_string_equality

:string_equality_false
    dmp "Strings are different!"
    jmp _ret_string_equality

:int64_equality_true
    dmp "Int64 are equal :)"
    jmp end

:int64_equality_false
    dmp "Int64 are different :("
    jmp end

:end
    dmp "Bye!"

; >>> Strings are different!
; >>> Int64 are equal :)
; >>> Bye!
```