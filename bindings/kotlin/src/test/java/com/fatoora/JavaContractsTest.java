package com.fatoora;

import org.junit.Test;
import static org.junit.Assert.*;

public class JavaContractsTest {
    @Test public void javaUsesFactoriesOverloadsAndExceptions() {
        try (SigningKey key = SigningKey.generate();
             SigningKey restored = SigningKey.fromPem(key.toPem());
             Address address = Address.create("SA", "الرياض", "Street", "1234", "12345")) {
            assertEquals(key.toPem(), restored.toPem());
            assertEquals("الرياض", address.city());
            assertNull(address.additionalStreet());
        }
        try { Config.create(99); fail("Expected BindingException"); }
        catch (BindingException e) { assertEquals(1, e.getCode()); assertNotNull(e.getMessage()); }
    }
}
