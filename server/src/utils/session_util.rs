use crate::models::auth::UserSession;
use actix_session::Session;

/// Sets user session.
///
/// # Arguments
///
/// * `session` - An session object
/// * `user_id` - A record id of the user account
/// * `user_email` -  An email of the user account
/// * `user_name` - A name of the user account
/// * `user_public_key` - A public key of the user account
/// * `user_avatar_url` - A avatar image url of the user account
pub fn set_session(
    session: &mut Session,
    user_id: u64,
    user_email: &str,
    user_name: &str,
    user_public_key: &str,
    user_avatar_url: &Option<String>,
) -> bool {
    let is_set_user_id = session.set("user_id", user_id);
    let is_set_user_email = session.set("user_email", user_email);
    let is_set_user_name = session.set("user_name", user_name);
    let is_set_user_public_key = session.set("user_public_key", user_public_key);

    let is_set_user_avatar_url = if let Some(user_avatar_url) = user_avatar_url {
        session.set("user_avatar_url", user_avatar_url)
    } else {
        Ok(())
    };

    !(is_set_user_id.is_err()
        || is_set_user_email.is_err()
        || is_set_user_name.is_err()
        || is_set_user_public_key.is_err()
        || is_set_user_avatar_url.is_err())
}

/// Clears session.
///
/// # Arguments
///
/// * `session` - An session object
pub fn unset_session(session: &mut Session) {
    session.clear();
}

/// Returns user session.
///
/// # Arguments
///
/// * `session` - An session object
pub fn get_session(session: &Session) -> Option<UserSession> {
    let user_id = if let Ok(id) = session.get::<u64>("user_id") {
        if let Some(id) = id {
            id
        } else {
            return None;
        }
    } else {
        return None;
    };

    let user_email = if let Ok(email) = session.get::<String>("user_email") {
        if let Some(email) = email {
            email
        } else {
            return None;
        }
    } else {
        return None;
    };

    let user_name = if let Ok(name) = session.get::<String>("user_name") {
        if let Some(name) = name {
            name
        } else {
            return None;
        }
    } else {
        return None;
    };

    let user_public_key = if let Ok(public_key) = session.get::<String>("user_public_key") {
        if let Some(public_key) = public_key {
            public_key
        } else {
            return None;
        }
    } else {
        return None;
    };

    let user_avatar_url = if let Ok(avatar_url) = session.get::<String>("user_avatar_url") {
        avatar_url
    } else {
        return None;
    };

    Some(UserSession {
        user_id,
        user_email,
        user_name,
        user_public_key,
        user_avatar_url,
    })
}

#[cfg(test)]
mod tests {
    use actix_session::UserSession;
    use actix_web::test;

    use super::*;

    #[test]
    fn test_set_session() {
        let req = test::TestRequest::default().to_srv_request();
        let mut session = req.get_session();

        let user_id = 10;
        let user_email = String::from("user@email.com");
        let user_name = String::from("park");
        let user_public_key = String::from("d63ee429");
        let user_avatar_url = String::from("image.jpg");

        let is_set_session = set_session(
            &mut session,
            user_id,
            &user_email,
            &user_name,
            &user_public_key,
            &Some(user_avatar_url.clone()),
        );

        assert_eq!(is_set_session, true);
        assert_eq!(session.get::<u64>("user_id").unwrap(), Some(user_id));
        assert_eq!(
            session.get::<String>("user_email").unwrap(),
            Some(user_email)
        );
        assert_eq!(session.get::<String>("user_name").unwrap(), Some(user_name));
        assert_eq!(
            session.get::<String>("user_public_key").unwrap(),
            Some(user_public_key)
        );
        assert_eq!(
            session.get::<String>("user_avatar_url").unwrap(),
            Some(user_avatar_url)
        );
    }

    #[test]
    fn test_unset_session() {
        let req = test::TestRequest::default().to_srv_request();
        let mut session = req.get_session();

        session.set("user_id", 10).unwrap();
        unset_session(&mut session);

        assert_eq!(session.get::<u64>("user_id").unwrap(), None);
    }

    #[test]
    fn test_get_session() {
        let req = test::TestRequest::default().to_srv_request();
        let session = req.get_session();

        let user_id = 10;
        let user_email = "user@email.com";
        let user_name = "park";
        let user_public_key = "d63ee429";
        let user_avatar_url = String::from("image.jpg");

        session.set("user_id", user_id).unwrap();
        session.set("user_email", user_email).unwrap();
        session.set("user_name", user_name).unwrap();
        session.set("user_public_key", user_public_key).unwrap();
        session
            .set("user_avatar_url", &Some(user_avatar_url.clone()))
            .unwrap();

        let user_session = get_session(&session);

        assert!(user_session.is_some());
        assert_eq!(user_session.as_ref().unwrap().user_id, user_id);
        assert_eq!(user_session.as_ref().unwrap().user_email, user_email);
        assert_eq!(user_session.as_ref().unwrap().user_name, user_name);
        assert_eq!(
            user_session.as_ref().unwrap().user_public_key,
            user_public_key
        );
        assert_eq!(
            user_session.as_ref().unwrap().user_avatar_url,
            Some(user_avatar_url)
        );
    }
}
