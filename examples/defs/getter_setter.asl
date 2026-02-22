// Simple getter (no parameters)
integer AArch64.GetValue
    return 42;

// Simple setter
AArch32.SetValue = integer val
    assert val > 0;

// Indexed getter (uses square brackets)
integer AArch64.GetElement[integer i]
    return i * 2;

// Indexed setter (uses square brackets)
AArch32.SetElement[integer idx] = integer val
    assert idx >= 0;
