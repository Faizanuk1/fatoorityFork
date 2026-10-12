using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct InvoiceRequest
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceRequest_method", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Method(InvoiceRequest* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceRequest_url", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Url(InvoiceRequest* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceRequest_headers_json", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError HeadersJson(InvoiceRequest* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceRequest_body", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultBytesBindingError Body(InvoiceRequest* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceRequest_decode", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultValidationResponseBindingError Decode(InvoiceRequest* handle, ushort status, DiplomatSliceU8 body);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceRequest_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(InvoiceRequest* handle);
}