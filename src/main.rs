use lettre::{Message, SmtpTransport, Transport};
use std::io;
use trust_dns_resolver::{config::*, lookup_ip::LookupIp, Resolver};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut from = String::new();
    let mut reply_to = String::new();
    let mut to = String::new();
    let mut subject = String::new();
    let mut body = String::new();

    // Gather user info
    println!("From: ");
    io::stdin().read_line(&mut from)?;
    from = add_arrow_brackets(&from);
    println!("Reply To: ");
    io::stdin().read_line(&mut reply_to)?;
    reply_to = add_arrow_brackets(&reply_to);
    println!("To: ");
    io::stdin().read_line(&mut to)?;
    to = add_arrow_brackets(&to);
    println!("Subject: ");
    io::stdin().read_line(&mut subject)?;
    println!("Body: ");
    io::stdin().read_line(&mut body)?;

    let email = Message::builder()
        .from(from.parse()?)
        .reply_to(reply_to.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body)?;

    dbg!(&email);

    // Create TLS transport on port 25
    let sender = SmtpTransport::builder_dangerous("olerud-com.mail.protection.outlook.com").build();
    dbg!(&sender);

    // Send the email via remote relay
    let result = sender.send(&email);
    dbg!(&result);
    println!("{:?}", result);
    assert!(result.is_ok());

    Ok(())
}

fn add_arrow_brackets(email_address: &str) -> String {
    format!("<{}>", email_address.trim())
}

fn get_mx_address(host: &str) -> Result<&LookupIp, Err()> {
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let mx_response = resolver.mx_lookup(&email_server);
    match mx_response {
        Err(_) => println!("MX address not found for {email_server}"),
        Ok(mx_response) => {
            let records = mx_response.iter();
            for record in records {
                println!("{} {}", record.preference(), record.exchange());
                let lookup_response = resolver.lookup_ip(record.exchange().to_string().as_str());
                match lookup_response {
                    Err(_) => println!("This exchange host has no address."),
                    Ok(lookup_address) => {
                        let ip_addrs = lookup_response.iter();
                        for ip_addr in ip_addrs {
                            return ip_addr;
                        }
                    }
                }
            }
        }
    }
}
