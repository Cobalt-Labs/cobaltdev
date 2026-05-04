// cloud/src/email.rs
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::authentication::Credentials,
    message::Mailbox,
    Address,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct ApiResponse {
    pub message: String,
    pub success: bool,
}

pub async fn send_email_handler(
    Json(payload): Json<ContactForm>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse>)> {
    match send_email(payload.name, payload.email, payload.message).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                message: "Email sent successfully".to_string(),
                success: true,
            }),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                message: format!("Failed to send email: {}", e),
                success: false,
            }),
        )),
    }
}

pub async fn send_email(name: String, email: String, message: String) -> Result<(), String> {
    let email_addr: Address = "ibrahim.haji.3595@gmail.com"
        .parse()
        .map_err(|e| format!("Invalid email address: {}", e))?;

    let from = Mailbox::new(Some("CobaltDev".to_string()), email_addr.clone());
    let to = Mailbox::new(None, email_addr);

    let email_msg = Message::builder()
        .from(from)
        .to(to)
        .subject("New Contact Form Submission from CobaltDev")
        .body(format!(
            "Name: {}\nEmail: {}\nMessage:\n\n{}",
            name, email, message
        ))
        .map_err(|e| format!("Failed to build email: {}", e))?;

    let creds = Credentials::new(
        "ibrahim.haji.3595@gmail.com".to_string(),
        "ibrahim_3595".to_string(),
    );

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
        .map_err(|e| format!("Failed to create mailer: {}", e))?
        .credentials(creds)
        .build();

    mailer.send(email_msg).await
        .map_err(|e| format!("Failed to send email: {}", e))?;

    println!("Email sent successfully from: {}", email);
    Ok(())
}