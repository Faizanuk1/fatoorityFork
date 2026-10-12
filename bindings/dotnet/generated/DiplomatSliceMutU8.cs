using System.Runtime.InteropServices;

namespace Fatoora.Diplomat;

[StructLayout(LayoutKind.Sequential)]
internal unsafe struct DiplomatSliceMutU8
{
    public byte* Ptr;
    public nuint Len;
}