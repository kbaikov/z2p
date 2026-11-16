use crate::authentication::UserId;
use crate::routes::admin::dashboard::get_username;
use crate::utils::e500;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, web};
use actix_web_flash_messages::IncomingFlashMessages;
use sqlx::PgPool;
use std::fmt::Write;

pub async fn change_password_form(
    user_id: web::ReqData<UserId>,
    flash_messages: IncomingFlashMessages,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();
    let _username = get_username(*user_id, &pool).await.map_err(e500)?;

    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Change Password</title>
</head>
<body>
    {msg_html}
    <form action="/admin/password" method="post">
        <label>Current password
            <input
                type="password"
                placeholder="Enter current password"
                name="current_password"
            >
        </label>
        <br>
        <label>New password
            <input
                type="password"
                placeholder="Enter new password"
                name="new_password"
            >
        </label>
        <br>
        <label>Confirm new password
            <input
                type="password"
                placeholder="Type the new password again"
                name="new_password_check"
            >
        </label>
        <br>
        <button type="submit">Change password</button>
    </form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>"#,
        )))
}
