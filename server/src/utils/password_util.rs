use scrypt::{scrypt_check, scrypt_simple, ScryptParams};

/// Returns a password that is hashed by scrypt.
///
/// # Arguments
///
/// * `password` - A password to be hashed
///
/// # Example
///
/// ```
/// use scrypt::scrypt_check;
/// use darim::utils::password_util::get_hashed_password;
///
/// let password = String::from("123");
/// let hashed_password = get_hashed_password(&password);
///
/// assert!(scrypt_check(&password, &hashed_password).is_ok());
/// ```
pub fn get_hashed_password(password: &str) -> String {
    let params = ScryptParams::new(15, 8, 1).unwrap();
    scrypt_simple(password, &params).unwrap()
}

/// Compares a plain-text password between hashed password
///
/// # Arguments
///
/// * `password` - A password to compare
/// * `hashed_password` - A hashed password returned by scrypt_simple()
///
/// # Example
///
/// ```
/// use scrypt::{ScryptParams, scrypt_simple};
/// use darim::utils::password_util::check_password;
///
/// let password = String::from("123");
/// let params = ScryptParams::new(12, 8, 1).unwrap();
/// let hashed_password = scrypt_simple(&password, &params).unwrap();
///
/// assert!(check_password(&password, &hashed_password));
/// ```
pub fn check_password(password: &str, hashed_password: &str) -> bool {
    scrypt_check(password, hashed_password).is_ok()
}
