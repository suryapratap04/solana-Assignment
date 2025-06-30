use bs58;
use base64;

pub fn decode_base58(s: &str) -> Result<Vec<u8>, String> {
    bs58::decode(s).into_vec().map_err(|_| "Invalid Base58 string".to_string())
}

pub fn encode_base58(bytes: &[u8]) -> String {
    bs58::encode(bytes).into_string()
}

pub fn encode_base64(bytes: &[u8]) -> String {
    base64::encode(bytes)
}

pub fn decode_base64(s: &str) -> Result<Vec<u8>, String> {
    base64::decode(s).map_err(|_| "Invalid Base64 string".to_string())
}
