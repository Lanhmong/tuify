use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

pub fn base64_encode(input: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(input)
}
