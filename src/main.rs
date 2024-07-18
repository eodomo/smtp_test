use lettre::{Message, SmtpTransport, Transport};
use std::io;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut from = String::new();
    let mut reply_to = String::new();
    let mut to = String::new();
    let mut subject = String::new();
    let mut body = String::new();

    // Gather user info
    println!("From: \n");
    io::stdin().read_line(&mut from)?;
    add_arrow_brackets(&from);
    io::stdin().read_line(&mut reply_to)?;
    add_arrow_brackets(&reply_to);
    io::stdin().read_line(&mut to)?;
    add_arrow_brackets(&to);
    io::stdin().read_line(&mut subject)?;
    io::stdin().read_line(&mut body)?;

    let email = Message::builder()
        .from("<eric@olerud>".parse()?)
        .reply_to("<eric@olerud>".parse()?)
        .to("<eric@olerud.com>".parse()?)
        .subject("This is a test email")
        .body(String::from("Enjoy this test email! :)"))?;

    // Create TLS transport on port 25
    let sender = SmtpTransport::builder_dangerous("olerud-com.mail.protection.outlook.com").build();

    // Send the email via remote relay
    let result = sender.send(&email);
    dbg!(&result);
    println!("{:?}", result);
    assert!(result.is_ok());

    Ok(())
}

fn add_arrow_brackets(email_address: &str) -> &str {
    format!("<{email_address}>")
}
