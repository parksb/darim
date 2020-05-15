use actix_session::Session;

pub fn set_session(session: Session, user_email: &String, user_name: &String) -> bool {
    let is_set_user_email = session.set("user_email", user_email);
    let is_set_user_name = session.set("user_name", user_name);

    return if is_set_user_email.is_err() || is_set_user_name.is_err() {
        false
    } else {
        true
    };
}

pub fn unset_session(session: Session) {
    session.clear();
}

pub fn check_session(session: &Session) -> bool {
    let user_email = session.get::<String>("user_email");
    let user_name = session.get::<String>("user_name");

    if user_email.is_err() || user_name.is_err() {
        false
    } else if user_email.unwrap().is_some() && user_name.unwrap().is_some() {
        true
    } else {
        false
    }
}
