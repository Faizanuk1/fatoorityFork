using System;
using System.Runtime.InteropServices;

namespace Fatoora.Raw;

using Fatoora;
using Fatoora.Diplomat;

[StructLayout(LayoutKind.Sequential)]
internal partial struct DiplomatResultBoolBindingError
{
    [StructLayout(LayoutKind.Explicit)]
    private unsafe struct InnerUnion
    {
        [FieldOffset(0)] internal byte ok;
        [FieldOffset(0)] internal BindingError* err;
    }

    private InnerUnion _inner;

    public DiplomatBool IsOk;
    public bool Ok => IsOk ? _inner.ok != 0 : throw new InvalidOperationException("Result does not contain Ok value");
    public unsafe BindingError* Err => !IsOk ? _inner.err : throw new InvalidOperationException("Result does not contain Err value");
}