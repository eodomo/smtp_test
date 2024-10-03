use lettre::Message;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrow_brackets() {
        let email_address = "sample@text.com";
        assert_eq!(add_arrow_brackets(email_address), "<sample@text.com>");
    }

    #[test]
    fn create_email() -> Result<(), Box<dyn std::error::Error>> {
        let from = "test@example.com";
        let reply_to = "test@example.com";
        let to = "test@example.com";
        let subject = "Subject text";
        let body = "Body text";
        super::create_email(from, reply_to, to, subject, body)?;
        Ok(())
    }
}
