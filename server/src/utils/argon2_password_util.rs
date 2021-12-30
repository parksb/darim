use argon2::{hash_encoded, verify_encoded, Config};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn hash_password(password: &str, salt: &str) -> String {
    let config = Config::default();
    hash_encoded(password.as_ref(), salt.as_ref(), &config).unwrap()
}

pub fn verify_hashed_password(hashed_password: &str, password: &str) -> bool {
    verify_encoded(hashed_password, password.as_ref()).unwrap()
}

pub fn generate_password_salt() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(16).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "abc";
        let salt = "stuvwxyz";

        let actual = hash_password(password, salt);
        let expected =
            "$argon2i$v=19$m=4096,t=3,p=1$c3R1dnd4eXo$S7DKb2Tp9pVInTXuo8w/Bg7bg5oselzKHrogjd/bB8c";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_verify_hashed_password() {
        let password = "abc";
        let hashed_password =
            "$argon2i$v=19$m=4096,t=3,p=1$c3R1dnd4eXo$S7DKb2Tp9pVInTXuo8w/Bg7bg5oselzKHrogjd/bB8c";

        let actual = true;
        let expected = verify_hashed_password(&hashed_password, password);

        assert_eq!(actual, expected);
    }
}
