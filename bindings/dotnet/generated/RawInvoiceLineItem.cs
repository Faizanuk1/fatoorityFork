using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct InvoiceLineItem
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_description", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Description(InvoiceLineItem* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_unit_code", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError UnitCode(InvoiceLineItem* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_quantity", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Quantity(InvoiceLineItem* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_unit_price", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError UnitPrice(InvoiceLineItem* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_total_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError TotalAmount(InvoiceLineItem* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_vat_rate", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError VatRate(InvoiceLineItem* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_vat_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError VatAmount(InvoiceLineItem* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_vat_category", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern byte VatCategory(InvoiceLineItem* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(InvoiceLineItem* handle);
}