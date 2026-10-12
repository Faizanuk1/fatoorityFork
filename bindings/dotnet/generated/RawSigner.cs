using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct Signer
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Signer_from_pem", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultSignerBindingError FromPem(DiplomatSliceU8 certPem, DiplomatSliceU8 keyPem);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Signer_from_der", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultSignerBindingError FromDer(DiplomatSliceU8 certDer, DiplomatSliceU8 keyDer);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Signer_certificate_der", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultBytesBindingError CertificateDer(Signer* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Signer_certificate_pem", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError CertificatePem(Signer* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Signer_sign", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultSignedInvoiceBindingError Sign(Signer* handle, FinalizedInvoice* invoice);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Signer_sign_xml", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SignXml(Signer* handle, DiplomatSliceU8 xml, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Signer_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(Signer* handle);
}