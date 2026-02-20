__instruction AddInstruction
    __encoding AddEncoding
        __instruction_set A64
        __field Rd 0 +: 5
        __field Rn 5 +: 5
        __opcode 'xxxxx000 00000000 00000000 00000000'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
    __postdecode
        assert d != 31;
    __execute
        integer result = d + n;
