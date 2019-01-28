use serde::Serialize;

/// Struct to store data and serialize to SendGrid's API for the mail_settings node
/// Use MailSettingsBuilder to construct this
#[derive(Serialize, Debug, Default)]
pub struct MailSettings {
    bcc: Option<BccSetting>,
    bypass_list_management: Option<BypassListSetting>,
    footer: Option<FooterSetting>,
    sandbox_mode: Option<SandboxModeSetting>,
    spam_check: Option<SpamCheckSetting>,
}

/// Builder pattern for creating `MailSettings` Make sure you call `build()` to consume this
/// builder and get the underlying `MailSettings` Construct with default().
#[derive(Default)]
pub struct MailSettingsBuilder {
    settings: MailSettings,
}

impl MailSettingsBuilder {
    /// Adds the BCC setting to the `MailSettings` struct. It will always be enabled if this is
    /// called. Just specify the email address to BCC.
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::MailSettingsBuilder;
    ///
    /// let builder = MailSettingsBuilder::default()
    ///               .bcc("bcc@example.com");
    /// ```
    pub fn bcc(mut self, email: impl Into<String>) -> Self {
        self.settings.bcc = Some(BccSetting {
            enable: true,
            email: email.into(),
        });
        self
    }

    /// Turns on the flag for bypass_list_management in `MailSettings`.
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::MailSettingsBuilder;
    ///
    /// let builder = MailSettingsBuilder::default()
    ///               .bypass_list_management();
    /// ```
    pub fn bypass_list_management(mut self) -> Self {
        self.settings.bypass_list_management = Some(BypassListSetting { enable: true });
        self
    }

    /// Adds a footer to `MailSettings` text form is the optional first parameter, and html is the
    /// second. Enabled flag is set to true when this method is called.
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::MailSettingsBuilder;
    ///
    /// let builder = MailSettingsBuilder::default()
    ///               .footer(Some(String::from("text footer")), Some(String::from("<h1>HTML Footer</h1>")));
    /// ```
    pub fn footer(mut self, text: Option<String>, html: Option<String>) -> Self {
        self.settings.footer = Some(FooterSetting {
            enable: true,
            text,
            html,
        });
        self
    }

    /// Enables the sandbox_mode flag for `MailSettings`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::MailSettingsBuilder;
    ///
    /// let builder = MailSettingsBuilder::default()
    ///               .sandbox_mode();
    /// ```
    pub fn sandbox_mode(mut self) -> Self {
        self.settings.sandbox_mode = Some(SandboxModeSetting { enable: true });
        self
    }

    /// Configures the spam_check node for `MailSettings`
    ///
    /// # Parameters
    /// threshold: Option<i32>
    /// post_to_url: Option<String>
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::MailSettingsBuilder;
    ///
    /// let builder = MailSettingsBuilder::default()
    ///               .spam_check(Some(5), Some(String::from("http://post_url")));
    /// ```
    pub fn spam_check(mut self, threshold: Option<i32>, post_to_url: Option<String>) -> Self {
        self.settings.spam_check = Some(SpamCheckSetting {
            enable: true,
            threshold,
            post_to_url,
        });
        self
    }

    /// Consumes the `MailSettingsBuilder` and returns the underlying `MailSettings`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid::MailSettingsBuilder;
    ///
    /// let builder = MailSettingsBuilder::default()
    ///               .build();
    /// ```
    pub fn build(self) -> MailSettings {
        self.settings
    }
}

/// Struct used for serializing the Bcc node into SendGrid's API format. Use `MailSettingsBuilder`
/// to configure this.
#[derive(Serialize, Debug)]
pub struct BccSetting {
    enable: bool,
    email: String,
}

/// Struct used for serializing the BypassList node into SendGrid's API format. Use
/// `MailSettingsBuilder` to configure this.
#[derive(Serialize, Debug)]
pub struct BypassListSetting {
    enable: bool,
}

/// Struct used for serializing the Footer node into SendGrid's API format. Use
/// `MailSettingsBuilder` to configure this.
#[derive(Serialize, Debug)]
pub struct FooterSetting {
    enable: bool,
    text: Option<String>,
    html: Option<String>,
}

/// Struct used for serializing the SandboxMode node into SendGrid's API format. Use
/// `MailSettingsBuilder` to configure this.
#[derive(Serialize, Debug)]
pub struct SandboxModeSetting {
    enable: bool,
}

/// Struct used for serializing the SpamCheck node into SendGrid's API format. Use
/// `MailSettingsBuilder` to configure this.
#[derive(Serialize, Debug)]
pub struct SpamCheckSetting {
    enable: bool,
    threshold: Option<i32>,
    post_to_url: Option<String>,
}
