package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OriginalInvoiceRefLib: Library {
    fun fatoora_OriginalInvoiceRef_destroy(handle: Pointer)
    fun fatoora_OriginalInvoiceRef_id(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_OriginalInvoiceRef_uuid(handle: Pointer): ResultPointerPointer
    fun fatoora_OriginalInvoiceRef_issue_date(handle: Pointer): ResultPointerPointer
}

class OriginalInvoiceRef internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    @Volatile
    internal var owned: Boolean,
)  {

    init {
        if (this.owned) {
            this.registerFinalizer()
        }
    }
    private fun registerFinalizer() {
        this.owned = true
    }

    @Override
    @SuppressWarnings("Finalize")
    fun finalize() {
        if (owned)
            lib.fatoora_OriginalInvoiceRef_destroy(handle)
    }

    companion object {
        internal val libClass: Class<OriginalInvoiceRefLib> = OriginalInvoiceRefLib::class.java
        internal val lib: OriginalInvoiceRefLib = Native.load("fatoora_ffi", libClass)
    }

    fun id(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_OriginalInvoiceRef_id(handle, write);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {

            val returnString = DW.writeToString(write)
            return returnString.ok()
        } else {
            DW.lib.diplomat_buffer_write_destroy(write)
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun uuid(): Result<Text?> {

        val returnVal = lib.fatoora_OriginalInvoiceRef_uuid(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (returnVal.isOk == 1.toByte()) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal ?: return Result.success(null)
            val returnOpaque = Text(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun issueDate(): Result<Text?> {

        val returnVal = lib.fatoora_OriginalInvoiceRef_issue_date(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (returnVal.isOk == 1.toByte()) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal ?: return Result.success(null)
            val returnOpaque = Text(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

}
