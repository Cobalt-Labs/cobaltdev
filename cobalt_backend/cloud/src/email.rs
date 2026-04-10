use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::authentication::Credentials,
    message::Mailbox,
};

pub async fn send_email(name: String, email: String, message: String) -> Result<(), String> {
    // Build sender and recipient properly
    let from = Mailbox::new(
        Some("CobaltDev".to_string()),
        "ibrahim.haji.3595@gmail.com".parse().map_err(|e| format!("Invalid from address: {}", e))?,
    );

    let to = Mailbox::new(
        None,
        "ibrahim.haji.3595@gmail.com".parse().map_err(|e| format!("Invalid to address: {}", e))?,
    );

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
        "your-email@gmail.com".to_string(),
        "your-app-password".to_string(),   // ← Use Gmail App Password
    );

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
        .map_err(|e| format!("Failed to create mailer: {}", e))?
        .credentials(creds)
        .build();

    mailer.send(email_msg).await
        .map_err(|e| format!("Failed to send email: {}", e))?;

    println!("✅ Email sent successfully from contact form!");
    Ok(())
}