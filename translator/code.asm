; reg5 - next symbol pointer
; reg3 - last symbol from input
section .code
    di
    la reg5, user_name
    la reg1, greeting
    call print
    call print_endl
    ei 
wait:
    test reg3, reg3
    bne wait

    di
    la reg1, hello
    call print
    la reg1, user_name
    call print
    halt

int_handler0:
    di
    in reg3, 0
    stbi reg5, reg3
    inc reg5
    ei
    ret

print:
    push reg3
    push reg4
    mov reg3, reg1
loop:
    lbi reg4, reg3

    test reg4, reg4
    be end
    out 0, reg4
    inc reg3
    jmp loop
end:
    pop reg4
    pop reg3
    ret

print_endl:
    push reg3
    movn reg3, 10
    out 0, reg3
    pop reg3
    ret

section .data
    vec int_handler0

greeting:
    str "Enter your name"
hello:
    str "Hello, " 
user_name: