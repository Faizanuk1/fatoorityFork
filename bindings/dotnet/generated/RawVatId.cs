using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct VatId
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_VatId_value", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Value(VatId* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_VatId_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(VatId* handle);
}