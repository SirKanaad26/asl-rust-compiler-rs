__instruction aarch32_BX_A
    __encoding aarch32_BX_T1_A
        __instruction_set T16
        __field Rm 19 +: 4
        __opcode '01000111 0xxxxxxx 00000000 00000000'
        __guard TRUE
        __unpredictable_unless 18 == '0'
        __unpredictable_unless 17 == '0'
        __unpredictable_unless 16 == '0'
        __decode
            m = UInt(Rm);
    __execute
        BXWritePC(R[m], BranchType_INDIR);
