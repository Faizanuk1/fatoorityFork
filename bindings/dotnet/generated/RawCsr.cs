using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct Csr
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Csr_from_der", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultCsrBindingError FromDer(DiplomatSliceU8 der);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Csr_to_der", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultBytesBindingError ToDer(Csr* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Csr_to_pem", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError ToPem(Csr* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Csr_to_base64", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError ToBase64(Csr* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Csr_to_pem_base64", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError ToPemBase64(Csr* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Csr_subject_string", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SubjectString(Csr* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Csr_extension_values_der", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultBytesListBindingError ExtensionValuesDer(Csr* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Csr_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(Csr* handle);
}