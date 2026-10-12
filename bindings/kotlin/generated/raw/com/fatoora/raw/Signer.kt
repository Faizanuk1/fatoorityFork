package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface SignerLib: Library {
    fun fatoora_Signer_destroy(handle: Pointer)
    fun fatoora_Signer_from_pem(certPem: Slice, keyPem: Slice): ResultPointerPointer
    fun fatoora_Signer_from_der(certDer: Slice, keyDer: Slice): ResultPointerPointer
    fun fatoora_Signer_certificate_der(handle: Pointer): ResultPointerPointer
    fun fatoora_Signer_certificate_pem(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_Signer_sign(handle: Pointer, invoice: Pointer): ResultPointerPointer
    fun fatoora_Signer_sign_xml(handle: Pointer, xml: Slice, write: Pointer): ResultUnitPointer
}

class Signer internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
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
            lib.fatoora_Signer_destroy(handle)
    }

    companion object {
        internal val libClass: Class<SignerLib> = SignerLib::class.java
        internal val lib: SignerLib = Native.load("fatoora_ffi", libClass)
        @JvmStatic

        fun fromPem(certPem: String, keyPem: String): Result<Signer> {
            val certPemSliceMemory = PrimitiveArrayTools.borrowUtf8(certPem)
            val keyPemSliceMemory = PrimitiveArrayTools.borrowUtf8(keyPem)

            val returnVal = lib.fatoora_Signer_from_pem(certPemSliceMemory.slice, keyPemSliceMemory.slice);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    val selfEdges: List<Any> = listOf()
                    val handle = nativeOkVal
                    val returnOpaque = Signer(handle, selfEdges, true)
                    return returnOpaque.ok()
                } else {
                    val selfEdges: List<Any> = listOf()
                    val handle = returnVal.getNativeErr()!!
                    val returnOpaque = BindingError(handle, selfEdges, true)
                    return returnOpaque.err()
                }
            } finally {
                certPemSliceMemory.close()
                keyPemSliceMemory.close()
            }
        }
        @JvmStatic

        fun fromDer(certDer: UByteArray, keyDer: UByteArray): Result<Signer> {
            val certDerSliceMemory = PrimitiveArrayTools.borrow(certDer)
            val keyDerSliceMemory = PrimitiveArrayTools.borrow(keyDer)

            val returnVal = lib.fatoora_Signer_from_der(certDerSliceMemory.slice, keyDerSliceMemory.slice);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    val selfEdges: List<Any> = listOf()
                    val handle = nativeOkVal
                    val returnOpaque = Signer(handle, selfEdges, true)
                    return returnOpaque.ok()
                } else {
                    val selfEdges: List<Any> = listOf()
                    val handle = returnVal.getNativeErr()!!
                    val returnOpaque = BindingError(handle, selfEdges, true)
                    return returnOpaque.err()
                }
            } finally {
                certDerSliceMemory.close()
                keyDerSliceMemory.close()
            }
        }
    }

    fun certificateDer(): Result<Bytes> {

        val returnVal = lib.fatoora_Signer_certificate_der(handle);
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

    fun certificatePem(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_Signer_certificate_pem(handle, write);
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

    fun sign(invoice: FinalizedInvoice): Result<SignedInvoice> {

        val returnVal = lib.fatoora_Signer_sign(handle, invoice.handle /* note this is a mutable reference. Think carefully about using, especially concurrently */);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            val selfEdges: List<Any> = listOf()
            val handle = nativeOkVal
            val returnOpaque = SignedInvoice(handle, selfEdges, true)
            return returnOpaque.ok()
        } else {
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.getNativeErr()!!
            val returnOpaque = BindingError(handle, selfEdges, true)
            return returnOpaque.err()
        }
    }

    fun signXml(xml: String): Result<String> {
        val xmlSliceMemory = PrimitiveArrayTools.borrowUtf8(xml)
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_Signer_sign_xml(handle, xmlSliceMemory.slice, write);
        try {
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
        } finally {
            xmlSliceMemory.close()
        }
    }

}
