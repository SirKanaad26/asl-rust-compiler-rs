__instruction TestUnpred
    __encoding TestEnc
        __instruction_set A64
        __field Rt 0 +: 4
        __opcode 'xxxx0001 1101xxxx xxxxxx00 1001xxxx'
        __guard TRUE
        __unpredictable_unless 6 == '0'
        __unpredictable_unless 11 == '1'
        __decode
            integer t = UInt(Rt);
    __execute
        integer result = t;
