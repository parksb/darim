use cfg_if::cfg_if;
use scrypt::{scrypt_check, scrypt_simple, ScryptParams};

/// Returns a password that is hashed by scrypt.
///
/// # Arguments
///
/// * `password` - A password to be hashed
///
/// # Example
///
/// ```ignore
/// use scrypt::scrypt_check;
/// use darim::utils::password_util::get_hashed_password;
///
/// let password = String::from("123");
/// let hashed_password = get_hashed_password(&password);
///
/// assert!(scrypt_check(&password, &hashed_password).is_ok());
/// ```
pub fn get_hashed_password(password: &str) -> String {
    let params = get_params_for_password_hashing();
    scrypt_simple(password, &params).unwrap()
}

cfg_if! {
    if #[cfg(test)] {
        fn get_params_for_password_hashing() -> ScryptParams {
            ScryptParams::new(7, 4, 1).unwrap()
        }
    } else {
        fn get_params_for_password_hashing() -> ScryptParams {
            ScryptParams::new(15, 8, 1).unwrap()
        }
    }
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
/// ```ignore
/// use scrypt::{ScryptParams, scrypt_simple};
/// use darim::utils::password_util::check_password;
///
/// let password = String::from("123");
/// let params = ScryptParams::new(7, 4, 1).unwrap();
/// let hashed_password = scrypt_simple(&password, &params).unwrap();
///
/// assert!(check_password(&password, &hashed_password));
/// ```
pub fn check_password(password: &str, hashed_password: &str) -> bool {
    scrypt_check(password, hashed_password).is_ok()
}

#[cfg(test)]
mod tests {
    use scrypt::{scrypt_check, scrypt_simple, ScryptParams};

    use super::*;

    #[test]
    fn test_get_hashed_password() {
        let password = String::from("123");
        let hashed_password = get_hashed_password(&password);

        assert!(scrypt_check(&password, &hashed_password).is_ok());
    }

    #[test]
    fn test_check_password() {
        let password = String::from("123");
        let params = ScryptParams::new(7, 4, 1).unwrap();
        let hashed_password = scrypt_simple(&password, &params).unwrap();

        assert!(check_password(&password, &hashed_password));
    }
}
