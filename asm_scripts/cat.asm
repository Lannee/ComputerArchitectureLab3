section .code

_start:
    mov io1, 0
    mov io2, 1

read_next:
    

print_next:
    la reg3, string
    lbui reg4, reg3

    test reg4, reg4
    be end
    out io1, reg4
    jmp print_next_symbol

end:
    hlt

section .data

string:
    str "Hello, World!"