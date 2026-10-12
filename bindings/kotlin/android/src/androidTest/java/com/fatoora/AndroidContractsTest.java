package com.fatoora;

import androidx.test.ext.junit.runners.AndroidJUnit4;
import androidx.test.platform.app.InstrumentationRegistry;
import java.io.ByteArrayOutputStream;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import org.junit.Test;
import org.junit.runner.RunWith;
import static org.junit.Assert.*;

@RunWith(AndroidJUnit4.class)
public class AndroidContractsTest {
    @Test public void javaApiAndNativeLoading() {
        try (SigningKey key = SigningKey.generate();
             SigningKey restored = SigningKey.fromPem(key.toPem());
             Address address = Address.create("SA", "الرياض", "Street", "1234", "12345")) {
            assertEquals(key.toPem(), restored.toPem());
            assertEquals("الرياض", address.city());
            assertNull(address.additionalStreet());
        }
        try { Config.create(99); fail("invalid environment accepted"); }
        catch (BindingException expected) { assertEquals(1, expected.getCode()); }
    }
    @Test public void clientInitializesTlsAndPreparesRequests() {
        try (Config config = Config.create(0);
             ZatcaClient client = ZatcaClient.create(config);
             CsidProduction credentials = CsidProduction.create(0, null, "token", "secret")) {
            assertNotNull(client);
            assertNotNull(credentials);
        }
    }
    @Test public void signedXmlMatchesSdk() throws Exception {
        try (InputStream stream = InstrumentationRegistry.getInstrumentation().getContext()
                .getAssets().open("cases/simplified-invoice/input.xml");
             ByteArrayOutputStream bytes = new ByteArrayOutputStream()) {
            byte[] buffer = new byte[8192];
            int count;
            while ((count = stream.read(buffer)) != -1) bytes.write(buffer, 0, count);
            assertEquals("aqY8frWKpauDQqXCsdne5hYwMQCo1DUUNKLfSAtT2mg=",
                Xml.hash(new String(bytes.toByteArray(), StandardCharsets.UTF_8)));
        }
    }
}
