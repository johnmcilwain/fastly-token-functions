use std::time::{SystemTime};
use hmacsha1::{hmac_sha1};

fn main() {
    let key = base64::decode("iqFPeN2u+Z0Lm5IrsKaOFKRqEU5Gw8ePtaEkHZWuD24=").unwrap();

    let token_lifetime = 1209600; // 2 weeks

    let path = "/foo/bar.html";

    let expiration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() + token_lifetime;

    let string_to_sign = format!("{}{}", path, expiration);

    let signature = hmac_sha1(&key, string_to_sign.as_bytes());
    let sighex: String = signature.iter().map(|s| format!("{:02x?}", s)).collect();       

    let token = format!("{}_{}", expiration, sighex);
    println!("token     : {}", token);
}

