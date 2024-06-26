; reg4 - next symbol pointer
; reg3 - last symbol from input

section .code
    la reg4, cat_data
wait:
    test reg3, reg3
    bne wait

    la reg1, cat_data
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
    mov reg3, reg1
loop:
    lbi reg4, reg3

    test reg4, reg4
    be end
    out 0, reg4
    inc reg3
    jmp loop
end:
    ret


section .data
    
    vec int_handler0

cat_data:
