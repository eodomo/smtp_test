use clap::Parser;
use email_address_parser::EmailAddress;
use lettre::{
    transport::smtp::client::{Tls, TlsParameters},
    SmtpTransport, Transport,
};
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
    #[arg(short, long)]
    encrypted: Option<bool>,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let from = args.from.unwrap_or_else(|| {
        let mut from = String::new();
        print!("From: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut from).unwrap();
        from.trim().to_string()
    });
    let reply_to = args.reply_to.unwrap_or_else(|| {
        let mut reply_to = String::new();
        print!("Reply To ({from}): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut reply_to).unwrap();
        reply_to = reply_to.trim().to_string();
        if reply_to == "" {
            from.clone()
        } else {
            reply_to
        }
    });
    let to = args.to.unwrap_or_else(|| {
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

    let email = create_email(&from, &reply_to, &to, &subject, &body).unwrap();

    #[cfg(debug_assertions)]
    dbg!(&email);

    let encrypt = args.encrypted.unwrap_or_else(|| {
        let mut encrypt = String::new();
        print!("Encrypt (Y/n): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut encrypt).unwrap();
        let encrypt = encrypt.trim().to_string();
        match encrypt.as_str() {
            "n" | "N" => false,
            _ => true,
        }
    });

    // Build email
    // Create TLS transport on port 25
    println!("Building email...");
    let sender = if encrypt {
        let tls = TlsParameters::builder(sender_mx.as_str().into())
            .dangerous_accept_invalid_certs(true)
            .build()?;
        SmtpTransport::relay(sender_mx.as_str())?
            .port(25)
            .tls(Tls::Required(tls))
            .build()
    } else {
        SmtpTransport::builder_dangerous(sender_mx).build()
    };
    #[cfg(debug_assertions)]
    dbg!(&sender);

    // Send the email via remote relay
    println!("Sending email...");
    let result = sender.send(&email);
    //dbg!(&result);
    println!("{:?}", result);
    assert!(result.is_ok());

    Ok(())
}
