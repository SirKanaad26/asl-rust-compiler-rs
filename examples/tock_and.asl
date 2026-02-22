__instruction aarch32_AND_r_A
    __encoding aarch32_AND_r_T1_A
        __instruction_set T16
        __field Rm 19 +: 3
        __field Rdn 16 +: 3
        __opcode '01000000 00xxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            d = UInt(Rdn);
            n = UInt(Rdn);
            m = UInt(Rm);
            setflags = TRUE;
            shift_t = SRType_LSL;
            shift_n = 0;
    __execute __conditional
        (shifted, carry) = Shift_C(R[m], shift_t, shift_n, PSTATE.C);
        result = R[n] AND shifted;
        if d == 15 then
            if setflags then
                ALUExceptionReturn(result);
            else
                ALUWritePC(result);
        else
            R[d] = result;
            if setflags then
                PSTATE.N = result<31>;
                PSTATE.Z = IsZeroBit(result);
                PSTATE.C = carry;
