package com.fatoora.raw;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ConfigLib: Library {
    fun fatoora_Config_destroy(handle: Pointer)
    fun fatoora_Config_new(env: FFIUint8): ResultPointerPointer
    fun fatoora_Config_env(handle: Pointer): FFIUint8
}

class Config internal constructor (
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
            lib.fatoora_Config_destroy(handle)
    }

    companion object {
        internal val libClass: Class<ConfigLib> = ConfigLib::class.java
        internal val lib: ConfigLib = Native.load("fatoora_ffi", libClass)
        @JvmStatic

        fun new_(env: UByte): Result<Config> {

            val returnVal = lib.fatoora_Config_new(FFIUint8(env));
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal
                val returnOpaque = Config(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!!
                val returnOpaque = BindingError(handle, selfEdges, true)
                return returnOpaque.err()
            }
        }
    }

    fun env(): UByte {

        val returnVal = lib.fatoora_Config_env(handle);
        return (returnVal.toUByte())
    }

}
