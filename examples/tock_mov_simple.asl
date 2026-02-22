__instruction aarch32_MOV_i_A
    __encoding aarch32_MOV_i_T1_A
        __instruction_set T16
        __field Rd 24 +: 3
        __field imm8 16 +: 8
        __opcode '00100xxx xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer imm32 = ZeroExtend(imm8, 32);
    __execute
        integer result = imm32;
        if d == 15 then
            ALUWritePC(result);
        else
            result = 0;
