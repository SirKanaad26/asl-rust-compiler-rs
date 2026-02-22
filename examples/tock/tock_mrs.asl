__instruction aarch32_MRS_AS
    __encoding aarch32_MRS_T1_AS
        __instruction_set T32
        __field R 20 +: 1
        __field Rd 8 +: 4
        __opcode '11110011 111xxxxx 10x0xxxx xx0xxxxx'
        __guard TRUE
        __unpredictable_unless 13 == '0'
        __unpredictable_unless 7 == '0'
        __decode
            d = UInt(Rd);
            read_spsr = (R == '1');
    __execute
        if read_spsr then
            R[d] = SPSR[];
        else
            R[d] = GetPSRFromPSTATE();
