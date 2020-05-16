use actix_session::Session;

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

pub fn unset_session(session: Session) {
    session.clear();
}

pub fn is_logged_in_user(session: &Session, user_id: Option<&u64>) -> bool {
    let user_id_in_session = session.get::<u64>("user_id");
    let user_email_in_session = session.get::<String>("user_email");
    let user_name_in_session = session.get::<String>("user_name");

    if user_id_in_session.is_err()
        || user_email_in_session.is_err()
        || user_name_in_session.is_err()
    {
        false
    } else if (&user_id_in_session).as_ref().unwrap().is_some()
        && user_email_in_session.unwrap().is_some()
        && user_name_in_session.unwrap().is_some()
    {
        if user_id.is_some() {
            if &user_id_in_session.unwrap().unwrap() == user_id.unwrap() {
                true
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    }
}
