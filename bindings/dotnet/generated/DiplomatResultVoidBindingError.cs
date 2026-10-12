using System;
using System.Runtime.InteropServices;

namespace Fatoora.Raw;

using Fatoora;
using Fatoora.Diplomat;

[StructLayout(LayoutKind.Sequential)]
internal partial struct DiplomatResultVoidBindingError
{
    [StructLayout(LayoutKind.Explicit)]
    private unsafe struct InnerUnion
    {
        [FieldOffset(0)] internal BindingError* err;
    }

    private InnerUnion _inner;

    public DiplomatBool IsOk;
    public unsafe BindingError* Err => !IsOk ? _inner.err : throw new InvalidOperationException("Result does not contain Err value");
}