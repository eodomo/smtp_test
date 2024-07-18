fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    use lettre::{Message, SmtpTransport, Transport};

    let email = Message::builder()
        .from("<erico@charlesit.com>".parse()?)
        .reply_to("<erico@charlesit.com>".parse()?)
        .to("<eric@olerud.com>".parse()?)
        .subject("This is a test email")
        .body(String::from("Enjoy this test email! :)"))?;

    // Create TLS transport on port 25
    let sender =
        SmtpTransport::builder_dangerous("charlesit-com.mail.protection.outlook.com").build();
    //    let sender = SmtpTransport::relay("charlesit-com.mail.protection.outlook.com")?.build();
    // Send the email via remote relay
    let result = sender.send(&email);
    dbg!(&result);
    println!("{:?}", result);
    assert!(result.is_ok());

    Ok(())
}
