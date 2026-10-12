package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CsrLib: Library {
    fun fatoora_Csr_destroy(handle: Pointer)
    fun fatoora_Csr_from_der(der: Slice): ResultPointerPointer
    fun fatoora_Csr_to_der(handle: Pointer): ResultPointerPointer
    fun fatoora_Csr_to_pem(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_Csr_to_base64(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_Csr_to_pem_base64(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_Csr_subject_string(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_Csr_extension_values_der(handle: Pointer): ResultPointerPointer
}

class Csr internal constructor (
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
            lib.fatoora_Csr_destroy(handle)
    }

    companion object {
        internal val libClass: Class<CsrLib> = CsrLib::class.java
        internal val lib: CsrLib = Native.load("fatoora_ffi", libClass)
        @JvmStatic

        fun fromDer(der: UByteArray): Result<Csr> {
            val derSliceMemory = PrimitiveArrayTools.borrow(der)

            val returnVal = lib.fatoora_Csr_from_der(derSliceMemory.slice);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    val selfEdges: List<Any> = listOf()
                    val handle = nativeOkVal
                    val returnOpaque = Csr(handle, selfEdges, true)
                    return returnOpaque.ok()
                } else {
                    val selfEdges: List<Any> = listOf()
                    val handle = returnVal.getNativeErr()!!
                    val returnOpaque = BindingError(handle, selfEdges, true)
                    return returnOpaque.err()
                }
            } finally {
                derSliceMemory.close()
            }
        }
    }

    fun toDer(): Result<Bytes> {

        val returnVal = lib.fatoora_Csr_to_der(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = Bytes(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun toPem(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_Csr_to_pem(handle, write);
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

    fun toBase64(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_Csr_to_base64(handle, write);
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

    fun toPemBase64(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_Csr_to_pem_base64(handle, write);
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

    fun subjectString(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_Csr_subject_string(handle, write);
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

    fun extensionValuesDer(): Result<BytesList> {

        val returnVal = lib.fatoora_Csr_extension_values_der(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = BytesList(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

}
