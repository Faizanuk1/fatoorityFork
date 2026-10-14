package com.fatoora.raw

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface InvoiceOutcomeLib: Library {
}
enum class InvoiceOutcome {
    Unknown,
    Accepted,
    Rejected;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<InvoiceOutcomeLib> = InvoiceOutcomeLib::class.java
        internal val lib: InvoiceOutcomeLib = Native.load("fatoora_ffi", libClass)
        fun fromNative(native: Int): InvoiceOutcome {
            return InvoiceOutcome.entries[native]
        }

        fun default(): InvoiceOutcome {
            return Unknown
        }
    }
}
