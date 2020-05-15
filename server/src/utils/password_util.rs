use crypto::{digest::Digest, sha3::Sha3};

pub fn get_hashed_password(original: String) -> String {
    let mut password_hasher = Sha3::sha3_512();
    password_hasher.input_str(&original);
    password_hasher.result_str()
}
