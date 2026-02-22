__instruction ASSIGN_test
    __encoding ASSIGN_A64
        __instruction_set A64
        __field Rt 0 +: 5
        __field Rn 5 +: 5
        __opcode 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxx00xx'
        __guard TRUE
        __decode
            integer t = UInt(Rt); integer n = UInt(Rn);
    __execute
        return;
