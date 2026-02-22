bits(8) GetLowByte(bits(32) x)
    return x<7:0>;

bit GetBit(bits(32) x, integer n)
    return x<n>;
