pub fn build_email(
    from: &str,
    reply_to: &str,
    to: &str,
    subject: &str,
    body: &str,
) -> lettre::Message::MessageBuilder {
    Message::builder()
        .from(from.parse()?)
        .reply_to(reply_to.parse()?)
        .to(to.parse()?)
        .subject(subject.parse()?)
        .body(body.parse()?);
}
