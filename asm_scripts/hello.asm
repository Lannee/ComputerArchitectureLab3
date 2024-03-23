section .code

_start:
    mov io1, 0

loop:
    la reg3, string
    lbui reg4, reg3

    test reg4, reg4
    be end
    out io1, reg4
    jmp loop

end:
    hlt

section .data

string:
    str "Hello, World!"