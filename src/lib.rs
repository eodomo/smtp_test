use std::io;
use trust_dns_resolver::{config::*, Name, Resolver};

pub fn add_arrow_brackets(email_address: &str) -> String {
    format!("<{}>", email_address.trim())
}

pub fn get_mx_address(host: &str) -> Result<Name, io::Error> {
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let mx_response = resolver.mx_lookup(host);
    match mx_response {
        Err(_) => panic!("MX address not found for {}", host),
        Ok(mx_response) => {
            let records = mx_response.iter();
            for record in records {
                //println!("{} {}", record.preference(), record.exchange());
                return Ok(record.exchange().clone());
            }
        }
    }
    panic!("get_mx_addr match did not complete");
}

pub fn gather_user_info(
) -> Result<(String, String, String, String, String), Box<dyn std::error::Error>> {
    let mut from = String::new();
    let mut reply_to = String::new();
    let mut to = String::new();
    let mut subject = String::new();
    let mut body = String::new();

    print!("From: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut from)?;
    print!("Reply To: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut reply_to)?;
    print!("To: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut to)?;
    print!("Subject: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut subject)?;
    print!("Body: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut body)?;

    Ok((from, reply_to, to, subject, body))
}

pub fn create_email(
    from: &str,
    reply_to: &str,
    to: &str,
    subject: &str,
    body: &str,
) -> Result<Message, Box<dyn std::error::Error>> {
    let from = add_arrow_brackets(from);
    let reply_to = add_arrow_brackets(reply_to);
    let to = add_arrow_brackets(to);

    let email = Message::builder()
        .from(from.parse()?)
        .reply_to(reply_to.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    Ok(email)
}
