__instruction TestInstruction
    __encoding TestEncoding
        __instruction_set A64
        __field op 20 +: 1
        __field Rd 0 +: 5
        __opcode '11111111 11111111 11111111 11111111'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
