use sendgrid::{ContactBuilder, MailSettingsBuilder, MessageBuilder, PersonalizationBuilder};

fn build_api_email() -> String {
    MessageBuilder::new(
        ContactBuilder::new("from@example.com").name("from").build(),
        "Subject Line!",
    )
    .template_id("SENDGRID-TEMPLATE-ID")
    // Don't Actually send email. If you want to really send the email, delete the line below
    .mail_settings(MailSettingsBuilder::default().sandbox_mode().build())
    .personalization(
        PersonalizationBuilder::default()
            .to(ContactBuilder::new("to@example.com").name("to").build())
            .build(),
    )
    .build()
    .to_json()
}

fn main() {
    // SendGrid's v3 API URL
    let url = "https://api.sendgrid.com/v3/mail/send";

    // API Key assigned by SendGrid
    let secret_api_key = "SENDGRID SECRET API KEY";
    let client = reqwest::Client::new()
        .post(url)
        .header("authorization", format!("bearer {}", secret_api_key))
        .header("content-type", "application/json")
        .body(build_api_email());

    client.send().expect("Error calling API");
}
