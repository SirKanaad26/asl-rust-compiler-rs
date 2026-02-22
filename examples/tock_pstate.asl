__instruction TestPSTATE
    __encoding TestPSTATEEnc
        __instruction_set T32
        __field Rd 8 +: 4
        __opcode '11110011 11100000 10000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rd);
    __execute
        if PSTATE.T == '0' then
            PSTATE.N = TRUE;
        else
            PSTATE.Z = FALSE;
