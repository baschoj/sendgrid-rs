//! This crate is a wrapper around SendGrid's v3 API using builder patterns to construct a payload
//! to send. This crate does not have batteries included, does not perform any validations, and
//! makes no assumptions other than what's specified in SendGrid's API documentation. To actually
//! call the API, you must use some other mechnism for the HTTP connection (such as the reqwest
//! crate).
//!
//! Everything stems from [Message](message/struct.Message.html) which you can construct using a
//! [MessageBuilder](message/struct.MessageBuilder.html). When you're done with the
//! `MessageBuilder` call `build()` to get the underlying `Message` and `to_json()` to get the
//! entire `Message` output as a JSON string.
//!
//! # Examples
//! ```
//! # use sendgrid_rs::{MessageBuilder, ContactBuilder, PersonalizationBuilder, MailSettingsBuilder};
//! let api_payload = MessageBuilder::new(
//!     ContactBuilder::new("from@example.com").name("from").build(),
//!     "Subject Line!",
//! )
//! .template_id("SENDGRID-TEMPLATE-ID")
//! // Don't Actually send email. If you want to really send the email, delete the line below
//! .mail_settings(MailSettingsBuilder::default().sandbox_mode().build())
//! .personalization(
//!     PersonalizationBuilder::default()
//!         .to(ContactBuilder::new("to@example.com").name("to").build())
//!         .build(),
//! )
//! .build()
//! .to_json();
//! ```

use serde::Serialize;

pub mod attachment;
pub mod mail_settings;
pub mod message;
pub mod personalization;
pub mod tracking_settings;

pub use crate::attachment::AttachmentBuilder;
pub use crate::mail_settings::MailSettingsBuilder;
pub use crate::message::MessageBuilder;
pub use crate::personalization::PersonalizationBuilder;
pub use crate::tracking_settings::{GaTrackingSettingBuilder, TrackingSettingsBuilder};

/// Type used for SendGrid's asm fields for managing subscriptions
/// Use `AsmBuilder` to construct this when adding it to a `Message`
#[derive(Debug, Serialize)]
pub struct Asm {
    group_id: i32,
    groups_to_display: Vec<i32>,
}

/// A builder pattern for constructing `Asm`
/// Make sure you call `build()` to consume the builder and retrieve the underyling `Asm`
pub struct AsmBuilder {
    asm: Asm,
}

impl AsmBuilder {
    /// Creates the builder. `group_id` is the only required paramteter which is associated
    /// to the SendGrid group_id for `Asm` subscriptions
    ///
    /// # Examples
    /// ```
    /// # use sendgrid_rs::AsmBuilder;
    ///
    /// let builder = AsmBuilder::new(1);
    /// ```
    pub fn new(group_id: i32) -> Self {
        AsmBuilder {
            asm: Asm {
                group_id,
                groups_to_display: vec![],
            },
        }
    }

    /// Adds a display group to the `Asm`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid_rs::AsmBuilder;
    ///
    /// let builder = AsmBuilder::new(1)
    ///     .group_to_display(2)
    ///     .group_to_display(3);
    /// ```
    pub fn group_to_display(mut self, group: i32) -> Self {
        self.asm.groups_to_display.push(group);
        self
    }

    /// Consumes the `AsmBuilder` and returns the underlying `Asm`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid_rs::AsmBuilder;
    ///
    /// let asm = AsmBuilder::new(1).build();
    /// ```
    pub fn build(self) -> Asm {
        self.asm
    }
}

/// `Content` is the struct used to add content fields on SendGrid's API
/// This is essentially a key/value store that serializes into the correct format
#[derive(Debug, Serialize)]
pub struct Content {
    #[serde(rename = "type")]
    c_type: String,
    value: String,
}

impl Content {
    /// Constructs a `Content`, this requires a content type (mime type) and value
    ///
    /// # Examples
    /// ```
    /// # use sendgrid_rs::Content;
    ///
    /// let content = Content::new("text/plain", "Message Content");
    /// ```
    pub fn new<S: Into<String>>(c_type: S, value: S) -> Self {
        Content {
            c_type: c_type.into(),
            value: value.into(),
        }
    }
}

/// Struct that holds the data needed for the 'contact' section in the SendGrid API.
/// Use a `ContactBuilder` to construct this.
#[derive(Serialize, Debug)]
pub struct Contact {
    email: String,
    name: Option<String>,
}

impl Contact {
    fn new(email: impl Into<String>, name: Option<String>) -> Contact {
        Contact {
            email: email.into(),
            name,
        }
    }
}

/// Builder pattern for `Contact`. Make sure you call `build()` when you're done to consume the
/// builder and return the underlying `Contact`.
pub struct ContactBuilder {
    contact: Contact,
}

impl ContactBuilder {
    /// Construct a new `ContactBuilder`. Email address is the only required parameter
    ///
    /// # Examples
    /// ```
    /// # use sendgrid_rs::ContactBuilder;
    ///
    /// let builder = ContactBuilder::new("from@example.com");
    /// ```
    pub fn new(email: impl Into<String>) -> Self {
        ContactBuilder {
            contact: Contact::new(email, None),
        }
    }

    /// Set the name of the `Contact`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid_rs::ContactBuilder;
    ///
    /// let builder = ContactBuilder::new("from@example.com")
    ///               .name("From");
    /// ```
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.contact.name = Some(name.into());
        self
    }

    /// Consume the builder and return the underlying `Contact`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid_rs::ContactBuilder;
    ///
    /// let builder = ContactBuilder::new("from@example.com").build();
    /// ```
    pub fn build(self) -> Contact {
        self.contact
    }
}

#[cfg(test)]
mod tests {
    use crate::{ContactBuilder, MessageBuilder};

    #[test]
    fn it_works() {
        let m = MessageBuilder::new(
            ContactBuilder::new("from@from.com").name("from").build(),
            "subject",
        )
        .build();
        println!("{}", m.to_json());
    }
}
