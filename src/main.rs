use clap::Parser;
use email_address_parser::EmailAddress;
use lettre::{Message, SmtpTransport, Transport};
use smtp_test::*;
use std::{io, io::Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    from: Option<String>,
    #[arg(short, long)]
    reply_to: Option<String>,
    #[arg(short, long)]
    to: Option<String>,
    #[arg(short, long)]
    subject: Option<String>,
    #[arg(short, long)]
    body: Option<String>,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut from = args.from.unwrap_or_else(|| {
        let mut from = String::new();
        print!("From: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut from).unwrap();
        from.trim().to_string()
    });
    let mut reply_to = args.reply_to.unwrap_or_else(|| {
        let mut reply_to = String::new();
        print!("Reply To: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut reply_to).unwrap();
        reply_to.trim().to_string()
    });
    let mut to = args.to.unwrap_or_else(|| {
        let mut to = String::new();
        print!("To: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut to).unwrap();
        to.trim().to_string()
    });
    let subject = args.subject.unwrap_or_else(|| {
        let mut subject = String::new();
        print!("Subject: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut subject).unwrap();
        subject.trim().to_string()
    });
    let body = args.body.unwrap_or_else(|| {
        let mut body = String::new();
        print!("Body: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut body).unwrap();
        body.trim().to_string()
    });

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
    println!("Building email...");
    let sender = SmtpTransport::builder_dangerous(sender_mx).build();
    //dbg!(&sender);

    // Send the email via remote relay
    println!("Sending email...");
    let result = sender.send(&email);
    //dbg!(&result);
    println!("{:?}", result);
    assert!(result.is_ok());

    Ok(())
}
