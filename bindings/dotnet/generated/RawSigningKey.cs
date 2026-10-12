using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct SigningKey
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SigningKey_generate", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultSigningKeyBindingError Generate();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SigningKey_from_pem", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultSigningKeyBindingError FromPem(DiplomatSliceU8 pem);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SigningKey_from_der", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultSigningKeyBindingError FromDer(DiplomatSliceU8 der);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SigningKey_to_pem", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError ToPem(SigningKey* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SigningKey_to_der", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultBytesBindingError ToDer(SigningKey* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SigningKey_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(SigningKey* handle);
}