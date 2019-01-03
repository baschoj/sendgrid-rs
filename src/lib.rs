use serde::Serialize;

pub mod attachment;
pub mod mail_settings;
pub mod message;
pub mod personalization;
pub mod tracking_settings;

pub type MessageBuilder = crate::message::MessageBuilder;
pub type PersonalizationBuilder = crate::personalization::PersonalizationBuilder;
pub type AttachmentBuilder = crate::attachment::AttachmentBuilder;
pub type MailSettingsBuilder = crate::mail_settings::MailSettingsBuilder;
pub type TrackingSettingsBuilder = crate::tracking_settings::TrackingSettingsBuilder;
pub type GaTrackingSettingBuilder = crate::tracking_settings::GaTrackingSettingBuilder;

/// 'Asm' is the struct used for SendGrid's asm fields for managing subscriptions
/// Use 'AsmBuilder' to construct this when adding it to a 'Message'
#[derive(Debug, Serialize)]
pub struct Asm {
    group_id: i32,
    groups_to_display: Vec<i32>,
}

/// A builder pattern for constructing 'Asm'
/// Make sure you call 'build()' to consume the builder and retrieve the underyling 'Asm'
pub struct AsmBuilder {
    asm: Asm,
}

impl AsmBuilder {
    /// Creates the builder. 'group_id' is the only required paramteter which is associated
    /// to the SendGrid group_id for Asm subscriptions
    ///
    /// # Examples
    /// ```
    /// use sendgrid::AsmBuilder;
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

    /// Adds a display group to the 'Asm'
    ///
    /// # Examples
    /// ```
    /// use sendgrid::AsmBuilder;
    ///
    /// let builder = AsmBuilder::new(1)
    ///     .group_to_display(2)
    ///     .group_to_display(3);
    /// ```
    pub fn group_to_display(mut self, group: i32) -> Self {
        self.asm.groups_to_display.push(group);
        self
    }

    /// Consumes the 'AsmBuilder' and returns the underlying 'Asm'
    ///
    /// # Examples
    /// ```
    /// use sendgrid::AsmBuilder;
    ///
    /// let asm = AsmBuilder::new(1).build();
    /// ```
    pub fn build(self) -> Asm {
        self.asm
    }
}

/// 'Content' is the struct used to add content fields on SendGrid's API
/// This is essentially a key/value store that serializes into the correct format
#[derive(Debug, Serialize)]
pub struct Content {
    #[serde(rename = "type")]
    c_type: String,
    value: String,
}

impl Content {
    /// Constructs a 'Content', this requires a content type (mime type) and value
    ///
    /// # Examples
    /// ```
    /// use sendgrid::Content;
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
/// Use a 'ContactBuilder' to construct this.
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

/// Builder pattern for 'Contact'. Make sure you call 'build()' when you're done to consume the
/// builder and return the underlying 'Contact'.
pub struct ContactBuilder {
    contact: Contact,
}

impl ContactBuilder {
    /// Construct a new 'ContactBuilder'. Email address is the only required parameter
    ///
    /// # Examples
    /// ```
    /// use sendgrid::ContactBuilder;
    ///
    /// let builder = ContactBuilder::new("from@example.com");
    /// ```
    pub fn new(email: impl Into<String>) -> Self {
        ContactBuilder {
            contact: Contact::new(email, None),
        }
    }

    /// Set the name of the 'Contact'
    ///
    /// # Examples
    /// ```
    /// use sendgrid::ContactBuilder;
    ///
    /// let builder = ContactBuilder::new("from@example.com")
    ///               .name("From");
    /// ```
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.contact.name = Some(name.into());
        self
    }

    /// Consume the builder and return the underlying 'Contact'
    ///
    /// # Examples
    /// ```
    /// use sendgrid::ContactBuilder;
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
