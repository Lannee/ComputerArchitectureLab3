  section .code
    la reg5, user_name
    la reg1, greeting
    call print
    call print_endl

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
    push reg6
    push reg7
    mov reg6, reg1
  loop:
    lbi reg7, reg6

    test reg7, reg7
    be end
    out 0, reg7
    inc reg6
    jmp loop
  end:  
    pop reg7
    pop reg6
    ret

  print_endl:
    push reg3
    movn reg3, 10 ; new line char
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