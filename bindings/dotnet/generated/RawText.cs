using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct Text
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Text_value", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Value(Text* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Text_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(Text* handle);
}