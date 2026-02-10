boolean isPalindrome(integer x)
    if x < 0 then
        return FALSE;
    integer original = x;
    integer reversed = 0;
    while x > 0 do
        reversed = reversed * 10 + x MOD 10;
        x = x DIV 10;
    return reversed == original;
