__instruction MovConditional
    __encoding MovCondEncoding
        __instruction_set A32
        __field Rd 12 +: 4
        __field Rn 16 +: 4
        __opcode 'xxxx0001 1010xxxx xxxx0000 0000xxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
    __execute __conditional
        integer result = n;
