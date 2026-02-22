__instruction NoPost
    __encoding NoPEncoding
        __instruction_set A64
        __field Rd 0 +: 5
        __opcode 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxx00xx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
    __execute
        return;
