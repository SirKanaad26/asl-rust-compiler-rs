__instruction MovRegInstr
    __encoding MovRegEnc
        __instruction_set A64
        __field Rd 0 +: 5
        __field Rn 5 +: 5
        __opcode 'xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
    __execute
        bits(64) result = Xreg(cpu, n);
        set_Xreg(cpu, d, result);
