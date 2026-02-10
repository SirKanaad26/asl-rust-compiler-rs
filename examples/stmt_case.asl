integer ProcessOpcode(integer opcode)
    integer result = 0;
    case opcode of
        when 0
            result = 100;
        when 1, 2
            result = 200;
        otherwise
            result = 999;
    return result;
