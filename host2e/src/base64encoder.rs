use base64::encode;
use bytes::Bytes;

pub fn run(plain: Bytes) -> String {
    encode(plain)
}
