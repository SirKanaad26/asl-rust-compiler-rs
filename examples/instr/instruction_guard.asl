__instruction CondInstruction
    __encoding CondEncoding
        __instruction_set A32
        __field cond 28 +: 4
        __field Rd 0 +: 5
        __opcode 'xxxx1110 00000000 00000000 00000000'
        __guard cond != '1111'
        __decode
            integer d = UInt(Rd);
