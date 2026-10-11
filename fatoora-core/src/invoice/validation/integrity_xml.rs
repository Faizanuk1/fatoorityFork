//! Signature preimages delegated to the private XML adapter.
use base64ct::{Base64, Encoding};
use sha2::{Digest, Sha256};

pub(super) fn invoice_digest(xml: &str) -> Result<String, String> {
    let bytes = crate::invoice::xml::canonicalize::prepare_invoice_digest_input(xml)?;
    Ok(Base64::encode_string(&Sha256::digest(bytes)))
}
pub(super) fn properties_preimage(xml: &str) -> Result<String, String> {
    crate::invoice::xml::canonicalize::properties_preimage(xml)
}
