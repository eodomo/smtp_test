use email_address_parser::EmailAddress;
use lettre::{Message, SmtpTransport, Transport};
use smtp_test::*;
use std::io;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut from = String::new();
    let mut reply_to = String::new();
    let mut to = String::new();
    let mut subject = String::new();
    let mut body = String::new();

    // Gather user info
    println!("From: ");
    io::stdin().read_line(&mut from)?;
    println!("Reply To: ");
    io::stdin().read_line(&mut reply_to)?;
    println!("To: ");
    io::stdin().read_line(&mut to)?;
    println!("Subject: ");
    io::stdin().read_line(&mut subject)?;
    println!("Body: ");
    io::stdin().read_line(&mut body)?;

    let sender_email_address = EmailAddress::parse(&from.trim(), None).unwrap();
    let sender_domain = sender_email_address.get_domain();
    let sender_mx = get_mx_address(sender_domain).unwrap().to_utf8();

    from = add_arrow_brackets(&from);
    reply_to = add_arrow_brackets(&reply_to);
    to = add_arrow_brackets(&to);

    let email = Message::builder()
        .from(from.parse()?)
        .reply_to(reply_to.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body)?;

    // Create TLS transport on port 25
    let sender = SmtpTransport::builder_dangerous(sender_mx).build();
    //dbg!(&sender);

    // Send the email via remote relay
    let result = sender.send(&email);
    //dbg!(&result);
    println!("{:?}", result);
    assert!(result.is_ok());

    Ok(())
}
