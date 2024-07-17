use lettre::Message;

pub struct Email<'A> {
    from: &'A str,
    reply_to: &'A str,
    to: &'A str,
    subject: &'A str,
    body: &'A str,
}
pub fn build_email(
    from: &str,
    reply_to: &str,
    to: &str,
    subject: &str,
    body: &str,
) -> Message::MessageBuilder {
    let result = Message::builder()
        .from(from.parse()?)
        .reply_to(reply_to.parse()?)
        .to(to.parse()?)
        .subject(subject.parse()?)
        .body(body.parse()?);

    result
}
