use crate::Contact;
use serde::Serialize;
use std::collections::HashMap;

/// Used to structure and serialize the personalization node in Sendgrid's API call. Use
/// 'PersonalizationBuilder' to construct this.
#[derive(Serialize, Debug)]
pub struct Personalization {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    to: Vec<Contact>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    cc: Vec<Contact>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    bcc: Vec<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    headers: HashMap<String, String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    substitutions: HashMap<String, String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    custom_args: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_at: Option<i32>,
}

/// Builder pattern for 'Personalization'. Make sure you call 'build()' when done to consume this
/// and return the underlying 'Personalization'.
pub struct PersonalizationBuilder {
    personalization: Personalization,
}

impl Default for PersonalizationBuilder {
    fn default() -> Self {
        PersonalizationBuilder::new()
    }
}

impl PersonalizationBuilder {
    /// Construct a new 'PersonalizationBuilder'. Does not take any parameters.
    ///
    /// # Examples
    /// ```
    /// use sendgrid::PersonalizationBuilder;
    ///
    /// let builder = PersonalizationBuilder::new();
    /// ```
    pub fn new() -> Self {
        PersonalizationBuilder {
            personalization: Personalization {
                to: Vec::new(),
                cc: Vec::new(),
                bcc: Vec::new(),
                subject: None,
                headers: HashMap::new(),
                substitutions: HashMap::new(),
                custom_args: HashMap::new(),
                send_at: None,
            },
        }
    }

    /// Add a 'To' contact. Use 'ContactBuilder' to construct this.
    ///
    /// # Examples
    /// ```
    /// use sendgrid::{ContactBuilder, PersonalizationBuilder};
    ///
    /// let builder = PersonalizationBuilder::new()
    ///               .to(ContactBuilder::new("to@example.com").build());
    /// ```
    pub fn to(mut self, contact: Contact) -> Self {
        self.personalization.to.push(contact);
        self
    }

    /// Add a 'CC' contact. Use 'ContactBuilder' to construct this.
    ///
    /// # Examples
    /// ```
    /// use sendgrid::{ContactBuilder, PersonalizationBuilder};
    ///
    /// let builder = PersonalizationBuilder::new()
    ///               .cc(ContactBuilder::new("cc@example.com").build());
    /// ```
    pub fn cc(mut self, contact: Contact) -> Self {
        self.personalization.cc.push(contact);
        self
    }

    /// Add a 'BCC' contact. Use 'ContactBuilder' to construct this.
    ///
    /// # Examples
    /// ```
    /// use sendgrid::{ContactBuilder, PersonalizationBuilder};
    ///
    /// let builder = PersonalizationBuilder::new()
    ///               .bcc(ContactBuilder::new("bcc@example.com").build());
    /// ```
    pub fn bcc(mut self, contact: Contact) -> Self {
        self.personalization.bcc.push(contact);
        self
    }

    /// Set a custom subject line.
    ///
    /// # Examples
    /// ```
    /// use sendgrid::PersonalizationBuilder;
    ///
    /// let builder = PersonalizationBuilder::new()
    ///               .subject("Subject line");
    /// ```
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.personalization.subject = Some(subject.into());
        self
    }

    /// Set an email header.
    ///
    /// # Parameters
    /// key: impl Into<String>
    /// value: impl Into<String>
    ///
    /// # Examples
    /// ```
    /// use sendgrid::PersonalizationBuilder;
    ///
    /// let builder = PersonalizationBuilder::new()
    ///               .header("Key", "Value");
    /// ```
    pub fn header<S: Into<String>>(mut self, key: S, value: S) -> Self {
        self.personalization
            .headers
            .insert(key.into(), value.into());
        self
    }

    /// Set a substitution
    ///
    /// # Parameters
    /// key: impl Into<String>
    /// value: impl Into<String>
    ///
    /// # Examples
    /// ```
    /// use sendgrid::PersonalizationBuilder;
    ///
    /// let builder = PersonalizationBuilder::new()
    ///               .substitution("Key", "Value");
    ///```
    pub fn substitution<S: Into<String>>(mut self, key: S, value: S) -> Self {
        self.personalization
            .substitutions
            .insert(key.into(), value.into());
        self
    }

    /// Set a custom_arg
    ///
    /// # Parameters
    /// key: impl Into<String>
    /// value: impl Into<String>
    ///
    /// # Examples
    /// ```
    /// use sendgrid::PersonalizationBuilder;
    ///
    /// let builder = PersonalizationBuilder::new()
    ///               .custom_arg("Key", "Value");
    /// ```
    pub fn custom_arg<S: Into<String>>(mut self, key: S, value: S) -> Self {
        self.personalization
            .custom_args
            .insert(key.into(), value.into());
        self
    }

    /// Set a send_at time in seconds
    ///
    /// # Examples
    /// ```
    /// use sendgrid::PersonalizationBuilder;
    ///
    /// let builder = PersonalizationBuilder::new()
    ///               .send_at(3600);
    /// ```
    pub fn send_at(mut self, time: i32) -> Self {
        self.personalization.send_at = Some(time);
        self
    }

    /// Consume the builder and return the underlying 'Personalization'
    ///
    /// # Examples
    /// ```
    /// use sendgrid::PersonalizationBuilder;
    ///
    /// let builder = PersonalizationBuilder::new()
    ///               .build();
    /// ```
    pub fn build(self) -> Personalization {
        self.personalization
    }
}
