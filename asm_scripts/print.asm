
; reg1 - string start pointer
print:
    mov io1, 0
loop:
    la reg3, string
    lbui reg4, reg3

    test reg4, reg4
    ret
    out io1, reg4
    jmp loop