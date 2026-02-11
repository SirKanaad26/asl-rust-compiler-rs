integer TryCatchTest(integer x)
    integer result = 0;
    try
        if x < 0 then
            throw NegativeError;
        result = x + 1;
    catch e
        when e == "NegativeError"
            result = -1;
        otherwise
            result = 0;
    return result;
