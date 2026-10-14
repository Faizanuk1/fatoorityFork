package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface BytesListLib: Library {
    fun fatoora_BytesList_destroy(handle: Pointer)
    fun fatoora_BytesList_len(handle: Pointer): FFISizet
    fun fatoora_BytesList_is_empty(handle: Pointer): Byte
    fun fatoora_BytesList_get(handle: Pointer, index: FFISizet): ResultPointerPointer
}

class BytesList internal constructor (
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
            lib.fatoora_BytesList_destroy(handle)
    }

    companion object {
        internal val libClass: Class<BytesListLib> = BytesListLib::class.java
        internal val lib: BytesListLib = Native.load("fatoora_ffi", libClass)
    }

    fun len(): ULong {

        val returnVal = lib.fatoora_BytesList_len(handle);
        return (returnVal.toULong())
    }

    fun isEmpty(): Boolean {

        val returnVal = lib.fatoora_BytesList_is_empty(handle);
        return (returnVal > 0)
    }

    fun get(index: ULong): Result<Bytes> {

        val returnVal = lib.fatoora_BytesList_get(handle, FFISizet(index));
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
