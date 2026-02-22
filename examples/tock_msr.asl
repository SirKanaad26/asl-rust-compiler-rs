__instruction aarch32_MSR_r_AS
    __encoding aarch32_MSR_r_T1_AS
        __instruction_set T32
        __field R 20 +: 1
        __field Rn 16 +: 4
        __field mask 8 +: 4
        __opcode '11110011 100xxxxx 10x0xxxx xx0xxxxx'
        __guard TRUE
        __unpredictable_unless 13 == '0'
        __unpredictable_unless 7 == '0'
        __decode
            n = UInt(Rn);
            write_spsr = (R == '1');
    __execute
        if write_spsr then
            SPSRWriteByInstr(R[n], mask);
        else
            CPSRWriteByInstr(R[n], mask);
