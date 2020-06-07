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
pub fn set_session(
    session: Session,
    user_id: &u64,
    user_email: &String,
    user_name: &String,
) -> bool {
    let is_set_user_id = session.set("user_id", user_id);
    let is_set_user_email = session.set("user_email", user_email);
    let is_set_user_name = session.set("user_name", user_name);

    return if is_set_user_id.is_err() || is_set_user_email.is_err() || is_set_user_name.is_err() {
        false
    } else {
        true
    };
}

/// Clears session.
///
/// # Arguments
///
/// * `session` - An session object
pub fn unset_session(session: Session) {
    session.clear();
}

/// Returns user session.
///
/// # Arguments
///
/// * `session` - An session object
pub fn get_session(session: &Session) -> Option<UserSession> {
    let user_id = session.get::<u64>("user_id");
    let user_email = session.get::<String>("user_email");
    let user_name = session.get::<String>("user_name");

    if user_id.is_err() || user_email.is_err() || user_name.is_err() {
        None
    } else if (&user_id).as_ref().unwrap().is_some()
        && (&user_email).as_ref().unwrap().is_some()
        && (&user_name).as_ref().unwrap().is_some()
    {
        Some(UserSession {
            user_id: user_id.unwrap().unwrap(),
            user_email: user_email.unwrap().unwrap(),
            user_name: user_name.unwrap().unwrap(),
        })
    } else {
        None
    }
}
