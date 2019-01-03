use serde::Serialize;

/// This is a struct for serializing SendGrid API attachments.
/// Use 'AttachmentBuilder' to construct these.
#[derive(Serialize, Debug)]
pub struct Attachment {
    content: String,
    #[serde(rename = "type")]
    a_type: Option<String>,
    filename: String,
    disposition: Option<String>,
    content_id: Option<String>,
}

/// Builder pattern for creating an 'Attachment'
/// Make sure you call 'build()' when done to consume the builder and return the underlying 'Attachment'
pub struct AttachmentBuilder {
    attachment: Attachment,
}

impl AttachmentBuilder {
    /// Constructs an 'AttachmentBuilder'. The required parameters are the content (base64 string)
    /// and the filename.
    ///
    /// # Examples
    /// ```
    /// use sendgrid::AttachmentBuilder;
    ///
    /// let builder = AttachmentBuilder::new(
    ///                 "SGVsbG8gV29ybGQh",
    ///                 "file.txt");
    /// ```
    pub fn new<S: Into<String>>(content: S, filename: S) -> Self {
        AttachmentBuilder {
            attachment: Attachment {
                content: content.into(),
                a_type: None,
                filename: filename.into(),
                disposition: None,
                content_id: None,
            },
        }
    }

    /// Sets the mime type on the 'Attachment'
    ///
    /// # Examples
    /// ```
    /// use sendgrid::AttachmentBuilder;
    ///
    /// let builder = AttachmentBuilder::new(
    ///                 "SGVsbG8gV29ybGQh",
    ///                 "file.txt")
    ///               .attachment_type("text/plain");
    /// ```
    pub fn attachment_type(mut self, t: impl Into<String>) -> Self {
        self.attachment.a_type = Some(t.into());
        self
    }

    /// Sets the disposition of the 'Attachment'
    ///
    /// # Examples
    /// ```
    /// use sendgrid::AttachmentBuilder;
    ///
    /// let builder = AttachmentBuilder::new(
    ///                 "SGVsbG8gV29ybGQh",
    ///                 "file.txt")
    ///               .disposition("inline");
    /// ```
    pub fn disposition(mut self, disposition: impl Into<String>) -> Self {
        self.attachment.disposition = Some(disposition.into());
        self
    }

    /// Sets the content_id of the 'Attachment'
    ///
    /// # Examples
    /// ```
    /// use sendgrid::AttachmentBuilder;
    ///
    /// let builder = AttachmentBuilder::new(
    ///                 "SGVsbG8gV29ybGQh",
    ///                 "file.txt")
    ///               .content_id("id");
    /// ```
    pub fn content_id(mut self, id: impl Into<String>) -> Self {
        self.attachment.content_id = Some(id.into());
        self
    }

    /// Consumes the 'AttachmentBuilder' and returns the underlying 'Attachment'
    ///
    /// # Examples
    /// ```
    /// use sendgrid::AttachmentBuilder;
    ///
    /// let builder = AttachmentBuilder::new(
    ///                 "SGVsbG8gV29ybGQh",
    ///                 "file.txt")
    ///               .build();
    /// ```
    pub fn build(self) -> Attachment {
        self.attachment
    }
}
