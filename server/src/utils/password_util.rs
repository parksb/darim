use crypto::{digest::Digest, sha3::Sha3};

/// Returns a password that is hashed by SHA3-512.
///
/// # Arguments
///
/// * `original` - A password to be hashed
///
/// # Example
///
/// ```
/// use darim::utils::password_util::get_hashed_password;
///
/// let password = String::from("123");
/// let hashed_password = get_hashed_password(password);
/// let expected = String::from("48c8947f69c054a5caa934674ce8881d02bb18fb59d5a63eeaddff735b0e9801e87294783281ae49fc8287a0fd86779b27d7972d3e84f0fa0d826d7cb67dfefc");
///
/// assert_eq!(hashed_password, expected);
/// ```
pub fn get_hashed_password(original: String) -> String {
    let mut password_hasher = Sha3::sha3_512();
    password_hasher.input_str(&original);
    password_hasher.result_str()
}
