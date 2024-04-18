section .code

    la reg3, string
loop:
    lbi reg4, reg3

    test reg4, reg4
    be end
    out 0, reg4
    inc reg3
    jmp loop

end:
    halt

section .data

string:
    str Hello World!