use url::Url;
use uuid::Uuid;

pub struct User {
    uuid: Uuid,
    email: String,
}

pub trait EmailDelivery {
    fn send_email(email_type: EmailType, user_context: String);
}

pub enum EmailType<'a> {
    EmailVerification(EmailVerificationType<'a>),
    PasswordReset(PasswordResetType<'a>),
    PasswordlessLogin(PasswordlessLoginType<'a>),
}

pub struct PasswordlessLoginType<'a> {
    email: &'a str,
    user_input_code: &'a str,
    url_with_link_code: Url,
    code_lifetime: u64,
    pre_auth_session_id: &'a str,
}

pub struct EmailVerificationType<'a> {
    user: User,
    email_verify_link: &'a str,
}

pub struct PasswordResetType<'a> {
    user: User,
    password_reset_link: &'a str,
}
