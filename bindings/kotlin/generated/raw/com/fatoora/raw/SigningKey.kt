package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface SigningKeyLib: Library {
    fun fatoora_SigningKey_destroy(handle: Pointer)
    fun fatoora_SigningKey_generate(): ResultPointerPointer
    fun fatoora_SigningKey_from_pem(pem: Slice): ResultPointerPointer
    fun fatoora_SigningKey_from_der(der: Slice): ResultPointerPointer
    fun fatoora_SigningKey_to_pem(handle: Pointer, write: Pointer): ResultUnitPointer
    fun fatoora_SigningKey_to_der(handle: Pointer): ResultPointerPointer
}

class SigningKey internal constructor (
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
            lib.fatoora_SigningKey_destroy(handle)
    }

    companion object {
        internal val libClass: Class<SigningKeyLib> = SigningKeyLib::class.java
        internal val lib: SigningKeyLib = Native.load("fatoora_ffi", libClass)
        @JvmStatic

        fun generate(): Result<SigningKey> {

            val returnVal = lib.fatoora_SigningKey_generate();
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal
                val returnOpaque = SigningKey(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        }
        @JvmStatic

        fun fromPem(pem: String): Result<SigningKey> {
            val pemSliceMemory = PrimitiveArrayTools.borrowUtf8(pem)

            val returnVal = lib.fatoora_SigningKey_from_pem(pemSliceMemory.slice);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    val selfEdges: List<Any> = listOf()
                    val handle = nativeOkVal
                    val returnOpaque = SigningKey(handle, selfEdges, true)
                    return returnOpaque.ok()
                } else {
                    val selfEdges: List<Any> = listOf()
                    val handle = returnVal.getNativeErr()!!
                    val returnOpaque = BindingError(handle, selfEdges, true)
                    return returnOpaque.err()
                }
            } finally {
                pemSliceMemory.close()
            }
        }
        @JvmStatic

        fun fromDer(der: UByteArray): Result<SigningKey> {
            val derSliceMemory = PrimitiveArrayTools.borrow(der)

            val returnVal = lib.fatoora_SigningKey_from_der(derSliceMemory.slice);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    val selfEdges: List<Any> = listOf()
                    val handle = nativeOkVal
                    val returnOpaque = SigningKey(handle, selfEdges, true)
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

    fun toPem(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_SigningKey_to_pem(handle, write);
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

    fun toDer(): Result<Bytes> {

        val returnVal = lib.fatoora_SigningKey_to_der(handle);
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

}
