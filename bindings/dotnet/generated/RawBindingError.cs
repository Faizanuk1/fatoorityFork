using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct BindingError
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_BindingError_code", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern int Code(BindingError* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_BindingError_message", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Message(BindingError* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_BindingError_details_json", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void DetailsJson(BindingError* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_BindingError_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(BindingError* handle);
}