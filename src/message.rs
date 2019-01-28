use crate::attachment::Attachment;
use crate::mail_settings::MailSettings;
use crate::personalization::Personalization;
use crate::tracking_settings::TrackingSettings;
use crate::{Asm, Contact, Content};
use serde::Serialize;
use std::collections::HashMap;

/// Message is the wrapper around the entire payload to be sent to SendGrid's API.
/// Use [MessageBuilder](struct.MessageBuilder.html) to properly construct this. The `to_json`
/// method is available to turn this struct into the request body to send to SendGrid
#[derive(Debug, Serialize)]
pub struct Message {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    personalizations: Vec<Personalization>,
    from: Contact,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to: Option<Contact>,
    subject: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    content: Vec<Content>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    attachments: Vec<Attachment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_id: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    sections: HashMap<String, String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    headers: HashMap<String, String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    categories: Vec<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    custom_args: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asm: Option<Asm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_pool_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mail_settings: Option<MailSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tracking_settings: Option<TrackingSettings>,
}

impl Message {
    /// `to_json` serializes the entire message into JSON. The result of this method call
    /// should contain the entire body for the HTTP POST to SendGrid. This method will
    /// panic if it is unable to serialize, because it is likely a bug in this crate that
    /// is causing it.
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("could not properly serialize into JSON")
    }
}

/// A `builder pattern` type for constructing `Message`
///
/// Use this to construct a Message with the desired data.
/// Make sure you call the 'build' method at the end to
/// consume this builder and return the underyling message.
///
/// # API Requirements
/// At least one personalization is required
/// From is required, but handled in the constructor
/// Subject is required, but handled in the constructor
pub struct MessageBuilder {
    message: Message,
}

impl MessageBuilder {
    /// Creates a `MessageBuilder`.
    ///
    /// # Parameters
    /// from: Contact
    /// subject: impl Into<String>
    ///
    /// # Examples
    ///
    /// ```
    /// # use sendgrid::{MessageBuilder, ContactBuilder};
    ///
    /// let builder = MessageBuilder::new(
    ///         ContactBuilder::new("from@example.com").build(),
    ///         "Subject Line"
    ///     );
    /// ```
    pub fn new(from: Contact, subject: impl Into<String>) -> Self {
        MessageBuilder {
            message: Message {
                personalizations: vec![],
                from,
                reply_to: None,
                subject: subject.into(),
                content: vec![],
                attachments: vec![],
                template_id: None,
                sections: HashMap::new(),
                headers: HashMap::new(),
                categories: vec![],
                custom_args: HashMap::new(),
                send_at: None,
                batch_id: None,
                asm: None,
                ip_pool_name: None,
                mail_settings: None,
                tracking_settings: None,
            },
        }
    }

    /// Adds a `Personalization` to the `Message`
    /// At least one is required according to the API specification
    /// Use a `PersonalizationBuilder` to construct the `Personalization`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::{MessageBuilder, ContactBuilder, PersonalizationBuilder};
    ///
    /// let builder = MessageBuilder::new(
    ///         ContactBuilder::new("from@example.com").build(),
    ///         "Subject Line"
    ///     )
    ///     .personalization(
    ///         PersonalizationBuilder::default().build()
    ///     );
    /// ```
    pub fn personalization(mut self, per: Personalization) -> Self {
        self.message.personalizations.push(per);
        self
    }

    /// Overwrites personalializations with input data.
    ///
    /// Use this to assign many personalizations at once.
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::{MessageBuilder, ContactBuilder, personalization::Personalization};
    ///
    /// let personalizations: Vec<Personalization> = Vec::new();
    /// let builder = MessageBuilder::new(
    ///         ContactBuilder::new("from@example.com").build(),
    ///         "Subject Line"
    ///     )
    ///     .personalizations(personalizations);
    /// ```
    pub fn personalizations(mut self, data: Vec<Personalization>) -> Self {
        self.message.personalizations = data;
        self
    }

    /// Adds a reply_to `Contact` to the `Message`
    /// Use a `ContactBuilder` to construct the `Contact`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::{MessageBuilder, ContactBuilder};
    ///
    /// let builder = MessageBuilder::new(
    ///         ContactBuilder::new("from@example.com").build(),
    ///         "Subject Line"
    ///     )
    ///     .reply_to(
    ///         ContactBuilder::new("reply_to@example.com").build()
    ///     );
    /// ```
    pub fn reply_to(mut self, contact: Contact) -> Self {
        self.message.reply_to = Some(contact);
        self
    }

    /// Adds a `Content` to to the `Message`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::{MessageBuilder, ContactBuilder, Content};
    ///
    /// let builder = MessageBuilder::new(
    ///         ContactBuilder::new("from@example.com").build(),
    ///         "Subject Line"
    ///         )
    ///         .content(
    ///             Content::new("text/plain", "Email Body")
    ///         );
    /// ```
    pub fn content(mut self, content: Content) -> Self {
        self.message.content.push(content);
        self
    }

    /// Adds an `Attachment` to the `Message`
    /// Use an `AttachmentBuilder` to construct the `Attachment`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::{MessageBuilder, ContactBuilder, AttachmentBuilder};
    ///
    /// let builder = MessageBuilder::new(
    ///         ContactBuilder::new("from@example.com").build(),
    ///         "Subject Line"
    ///         )
    ///         .attachment(
    ///             AttachmentBuilder::new(
    ///                 "SGVsbG8gV29ybGQh",
    ///                 "file.txt"
    ///             ).build()
    ///         );
    /// ```
    pub fn attachment(mut self, attachment: Attachment) -> Self {
        self.message.attachments.push(attachment);
        self
    }

    /// Sets the template id the `Message` will use.
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::{MessageBuilder, ContactBuilder};
    ///
    /// let builder = MessageBuilder::new(
    ///         ContactBuilder::new("from@example.com").build(),
    ///         "Subject Line"
    ///         )
    ///         .template_id("0001");
    /// ```
    pub fn template_id(mut self, id: impl Into<String>) -> Self {
        self.message.template_id = Some(id.into());
        self
    }

    /// Adds a section key/value pair to the `Message`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::{MessageBuilder, ContactBuilder};
    ///
    /// let builder = MessageBuilder::new(
    ///         ContactBuilder::new("from@example.com").build(),
    ///         "Subject Line"
    ///         )
    ///         .section("Key", "Value");
    /// ```
    pub fn section<S: Into<String>>(mut self, key: S, value: S) -> Self {
        self.message.sections.insert(key.into(), value.into());
        self
    }

    /// Adds a header key/value pair to the `Message`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::{MessageBuilder, ContactBuilder};
    ///
    /// let builder = MessageBuilder::new(
    ///         ContactBuilder::new("from@example.com").build(),
    ///         "Subject Line"
    ///         )
    ///         .header("Key", "Value");
    /// ```
    pub fn header<S: Into<String>>(mut self, key: S, value: S) -> Self {
        self.message.headers.insert(key.into(), value.into());
        self
    }

    /// Adds a category to the `Message`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::{MessageBuilder, ContactBuilder};
    ///
    /// let builder = MessageBuilder::new(
    ///         ContactBuilder::new("from@example.com").build(),
    ///         "Subject Line"
    ///         )
    ///         .category("Marketing");
    /// ```
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.message.categories.push(category.into());
        self
    }

    /// Adds a custom arg to the `Message`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::{MessageBuilder, ContactBuilder};
    ///
    /// let builder = MessageBuilder::new(
    ///         ContactBuilder::new("from@example.com").build(),
    ///         "Subject Line"
    ///         )
    ///         .custom_arg("arg_name", "arg_value");
    /// ```
    pub fn custom_arg<S: Into<String>>(mut self, key: S, value: S) -> Self {
        self.message.custom_args.insert(key.into(), value.into());
        self
    }

    /// Adds a send_at time to the `Message`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::{MessageBuilder, ContactBuilder};
    ///
    /// let builder = MessageBuilder::new(
    ///         ContactBuilder::new("from@example.com").build(),
    ///         "Subject Line"
    ///         )
    ///         .send_at(3600);
    /// ```
    pub fn send_at(mut self, time: i32) -> Self {
        self.message.send_at = Some(time);
        self
    }

    /// Adds a batch_id to the `Message`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::{MessageBuilder, ContactBuilder};
    ///
    /// let builder = MessageBuilder::new(
    ///         ContactBuilder::new("from@example.com").build(),
    ///         "Subject Line"
    ///         )
    ///         .batch_id("abc123");
    /// ```
    pub fn batch_id(mut self, id: impl Into<String>) -> Self {
        self.message.batch_id = Some(id.into());
        self
    }

    /// Adds an `Asm` to the `Message`
    /// Use `AsmBuilder` to construct the `Asm`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::{MessageBuilder, ContactBuilder, AsmBuilder};
    ///
    /// let builder = MessageBuilder::new(
    ///         ContactBuilder::new("from@example.com").build(),
    ///         "Subject Line"
    ///         )
    ///         .asm(
    ///             AsmBuilder::new(1)
    ///             .build()
    ///         );
    /// ```
    pub fn asm(mut self, asm: Asm) -> Self {
        self.message.asm = Some(asm);
        self
    }

    /// Adds the ip_pool_name to the `Message`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::{MessageBuilder, ContactBuilder};
    ///
    /// let builder = MessageBuilder::new(
    ///         ContactBuilder::new("from@example.com").build(),
    ///         "Subject Line"
    ///         )
    ///         .ip_pool_name("marketing_pool");
    /// ```
    pub fn ip_pool_name(mut self, name: impl Into<String>) -> Self {
        self.message.ip_pool_name = Some(name.into());
        self
    }

    /// Adds `MailSettings` to the `Message`
    /// Use `MailSettingsBuilder` to construct the `MailSettings`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::{MessageBuilder, ContactBuilder, MailSettingsBuilder};
    ///
    /// let builder = MessageBuilder::new(
    ///         ContactBuilder::new("from@example.com").build(),
    ///         "Subject Line"
    ///         )
    ///         .mail_settings(
    ///             MailSettingsBuilder::default().build()
    ///         );
    /// ```
    pub fn mail_settings(mut self, settings: MailSettings) -> Self {
        self.message.mail_settings = Some(settings);
        self
    }

    /// Adds `TrackingSettings` to the `Message`
    /// Use `TrackingSettingsBuilder` to construct the `TrackingSettings`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::{MessageBuilder, ContactBuilder, TrackingSettingsBuilder};
    ///
    /// let builder = MessageBuilder::new(
    ///         ContactBuilder::new("from@example.com").build(),
    ///         "Subject Line"
    ///         )
    ///         .tracking_settings(
    ///             TrackingSettingsBuilder::default().build()
    ///         );
    /// ```
    pub fn tracking_settings(mut self, settings: TrackingSettings) -> Self {
        self.message.tracking_settings = Some(settings);
        self
    }

    /// Consumes the `MessageBuilder` and returns the `Message`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::{MessageBuilder, ContactBuilder};
    ///
    /// let message = MessageBuilder::new(
    ///         ContactBuilder::new("from@example.com").build(),
    ///         "Subject Line"
    ///         )
    ///         .build();
    /// ```
    pub fn build(self) -> Message {
        self.message
    }
}
