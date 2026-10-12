package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface BindingErrorLib: Library {
    fun fatoora_BindingError_destroy(handle: Pointer)
    fun fatoora_BindingError_code(handle: Pointer): Int
    fun fatoora_BindingError_message(handle: Pointer, write: Pointer): Unit
    fun fatoora_BindingError_details_json(handle: Pointer, write: Pointer): Unit
}

class BindingError internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal var owned: Boolean,
): Exception("Rust error result for BindingError")  {

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
            lib.fatoora_BindingError_destroy(handle)
    }

    companion object {
        internal val libClass: Class<BindingErrorLib> = BindingErrorLib::class.java
        internal val lib: BindingErrorLib = Native.load("fatoora_ffi", libClass)
    }

    fun code(): Int {

        val returnVal = lib.fatoora_BindingError_code(handle);
        return (returnVal)
    }

    fun message(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_BindingError_message(handle, write);

        val returnString = DW.writeToString(write)
        return returnString
    }

    fun detailsJson(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.fatoora_BindingError_details_json(handle, write);

        val returnString = DW.writeToString(write)
        return returnString
    }

}
