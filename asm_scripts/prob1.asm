section .code
    di
    movn reg1, 10
    call prob1
    halt

; reg1 - аргумент
prob1:
    ; reg3 - сумма
    ; reg4 - итератор
    movn reg3, 0
    movn reg4, 0

loop:
    inc reg4 
    cmp reg4, reg1 ; выставление флага 
    ble end        ; branch if lower or equal

    movn reg6, 3
    rem reg5, reg4, reg6
    test reg5, reg5
    be add

    movn reg6, 5
    rem reg5, reg4, reg6
    test reg5, reg5
    bne loop

add:
    add reg3, reg3, reg4
    jmp loop
end:
    mov reg0, reg3
    ret

section .data