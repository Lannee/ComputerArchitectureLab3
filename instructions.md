# Language documentation


## Instructions
- mov

- st

- add
- sub
- mul
- rem

- jmp

- cmp
- bg
- bne
- be

- hlt

### Load
#### lw - load word (32 bits)
#### lb/lbu
Load byte with/without zero extension 

### Input / Output
#### in
#### out
Outputs 32 bit value from register to specified out port

```
    mov io1, 0x1    ; specifies out port (0x1)
    mov reg3, 0x12    ; specifies output data
    out io1, reg3
```
    
<!-- - di
- ei -->


## Sections

### .code
    Instructions in this section will be loaded to the instruction memory 

### .data
    Data in this section will be loaded to the data memory 



## Registers

### General purpose
- reg0 - used for functions result
- reg1, reg2 - used for functions arguments
- reg3, reg4, reg5

- reg6 (io1), reg7 (io2) - used for input/output