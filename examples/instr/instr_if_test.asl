__instruction TestIf
    __encoding TestIfEnc
        __instruction_set A64
        __field Rd 0 +: 5
        __opcode 'xxxxx000 00000001 00000000 00000001'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
    __execute
        integer result = d;
        if d == 15 then
            result = 0;
        else
            result = 1;
        SetResult(result);
