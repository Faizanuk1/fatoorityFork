package com.fatoora

import java.util.concurrent.CountDownLatch
import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit
import org.junit.Assert.*
import org.junit.Test

class OwnershipTest {
    @Test fun closeWaitsForNativeCalls() {
        val key = SigningKey.generate()
        val entered = CountDownLatch(1)
        val release = CountDownLatch(1)
        val closing = CountDownLatch(1)
        val pool = Executors.newFixedThreadPool(2)
        try {
            val read = pool.submit<String> { nativeCall(listOf(key)) {
                entered.countDown()
                check(release.await(10, TimeUnit.SECONDS))
                key.toPem()
            } }
            assertTrue(entered.await(10, TimeUnit.SECONDS))
            val close = pool.submit { closing.countDown(); key.close() }
            assertTrue(closing.await(10, TimeUnit.SECONDS))
            assertFalse(close.isDone)
            release.countDown()
            assertTrue(read.get(10, TimeUnit.SECONDS).contains("PRIVATE KEY"))
            close.get(10, TimeUnit.SECONDS)
            try { key.toPem(); fail("closed key was usable") }
            catch (expected: IllegalStateException) { }
        } finally { release.countDown(); pool.shutdownNow(); key.close() }
    }
}
