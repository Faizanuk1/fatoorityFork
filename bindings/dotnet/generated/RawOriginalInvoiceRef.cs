using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct OriginalInvoiceRef
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_OriginalInvoiceRef_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Id(OriginalInvoiceRef* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_OriginalInvoiceRef_uuid", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError Uuid(OriginalInvoiceRef* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_OriginalInvoiceRef_issue_date", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError IssueDate(OriginalInvoiceRef* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_OriginalInvoiceRef_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(OriginalInvoiceRef* handle);
}