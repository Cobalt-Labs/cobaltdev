use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::authentication::Credentials,
};

pub async fn send_email(name: String, email: String, message: String) {
    let email_msg = Message::builder()
        .from("CobaltDev <your@email.com>".parse().unwrap())
        .to("your@email.com".parse().unwrap())
        .subject("New Contact Form Message")
        .body(format!(
            "Name: {}\nEmail: {}\nMessage: {}",
            name, email, message
        ))
        .unwrap();

    let creds = Credentials::new(
        "your@email.com".to_string(),
        "your_app_password".to_string(),
    );

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    let _ = mailer.send(email_msg).await;
}