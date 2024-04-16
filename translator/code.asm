section .code
    mov reg0, reg1
    movn reg0, 32
label1:
    mov reg1, reg0
label_in_line:  
    movn reg0, 32
    la reg1, string
    la reg6, label_in_void
    la io1, label_in_void
    out 1, reg2
    lb reg1, a
    jmp label1


section .data
a:  byte 123
    byte 255
    char b

string:
    str hello world!
label_in_void:
    byte 54