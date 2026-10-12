package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface BytesLib: Library {
    fun fatoora_Bytes_destroy(handle: Pointer)
    fun fatoora_Bytes_as_slice(handle: Pointer): Slice
}

class Bytes internal constructor (
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
            lib.fatoora_Bytes_destroy(handle)
    }

    companion object {
        internal val libClass: Class<BytesLib> = BytesLib::class.java
        internal val lib: BytesLib = Native.load("fatoora_ffi", libClass)
    }

    /**
     * The view is valid while this immutable owner remains alive.
     */
    fun asSlice(): UByteArray {
        // This lifetime edge depends on lifetimes: 'a
        val aEdges: MutableList<Any> = mutableListOf(this);

        val returnVal = lib.fatoora_Bytes_as_slice(handle);
            return PrimitiveArrayTools.getUByteArray(returnVal)
    }

}
