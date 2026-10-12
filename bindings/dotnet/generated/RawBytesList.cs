using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct BytesList
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_BytesList_len", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint Len(BytesList* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_BytesList_is_empty", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.U1)]
    internal static unsafe extern bool IsEmpty(BytesList* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_BytesList_get", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultBytesBindingError Get(BytesList* handle, nuint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_BytesList_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(BytesList* handle);
}