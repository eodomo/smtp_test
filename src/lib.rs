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
