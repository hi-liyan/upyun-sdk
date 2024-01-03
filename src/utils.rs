use hmac::{Hmac, Mac};
use sha1::Sha1;

fn hmac_sha1(key: &str, data: &[u8]) -> Vec<u8> {
    let mut hmac = Hmac::<Sha1>::new_from_slice(key.as_bytes()).expect("Invalid key length");
    hmac.update(data);
    hmac.finalize().into_bytes().to_vec()
}