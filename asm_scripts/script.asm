
// reg1 - аргумент
prob1:
    // reg3 - сумма
    // reg4 - итератор
    mov reg3, 0
    mov reg4, 1

.loop:
    cmp reg4, reg1 // выставление флага 
    bg .end        // branch if greater 

    rem reg5, reg4, 3
    cmp reg5, 0
    be .add

    rem reg5, reg4, 5
    cmp reg5, 0
    bne .loop

.add:
    add reg3, reg4
    jmp .loop
.end:
    mov reg0, reg3
    ret





// reg0 - регистр возврата значения
// reg1, reg2 - регистр аргументов функций
// reg3 - reg5