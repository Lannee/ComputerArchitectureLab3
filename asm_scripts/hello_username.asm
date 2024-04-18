; reg5 - next symbol pointer
; reg3 - last symbol from input
section .code
    di
    la reg1, greeting
    call print
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
    stbi reg4, reg3
    inc reg4
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

section .data
    vec int_handler0

greeting:
    str Enter your name:
hello:
    str hello 
user_name: