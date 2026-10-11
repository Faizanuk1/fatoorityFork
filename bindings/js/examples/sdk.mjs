// --8<-- [start:csr]
import { CsrProperties, SigningKey } from 'fatoority';

export function generateCsr(propertiesText) {
  const key = SigningKey.generate();
  const csr = CsrProperties.fromPropertiesStr(propertiesText).build(key, 0);
  return { csrPem: csr.toPem(), keyPem: key.toPem() };
}
// --8<-- [end:csr]

// --8<-- [start:signing]
import { Signer } from 'fatoority';

export function signInvoice(xml, certificatePem, keyPem) {
  return Signer.fromPem(certificatePem, keyPem).signXml(xml);
}
// --8<-- [end:signing]

// --8<-- [start:validation]
import { Config, Xml } from 'fatoority';

export function validateInvoice(xml, previousHash, evaluatedAt) {
  return JSON.parse(Xml.validateZatca(Config.new_(0), xml, JSON.stringify({
    previous_invoice_hash: previousHash,
    evaluated_at: evaluatedAt, // Omit to use the JavaScript host clock.
  })));
}
// Inspect report.is_valid and report.stages before accepting an invoice.
// --8<-- [end:validation]

// --8<-- [start:api]
import { Config as ApiConfig, CsidProduction, SignedInvoice, ZatcaClient } from 'fatoority';

export async function reportInvoice(signedXml, token, secret, { signal, fetch: transport = globalThis.fetch } = {}) {
  const client = new ZatcaClient(ApiConfig.new_(2), { fetch: transport });
  const credentials = CsidProduction.create(2, null, token, secret);
  const invoice = SignedInvoice.fromXml(signedXml);
  const response = await client.reportSimplifiedInvoice(invoice, credentials, false, 'en', { signal });
  response.ensureAccepted();
  return response;
}
// --8<-- [end:api]
