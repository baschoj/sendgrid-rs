use serde::Serialize;

/// Struct used for serializing the ClickTracking node into SendGrid's API format. Use
/// `TrackingSettingsBuilder` to configure this.
#[derive(Serialize, Debug)]
pub struct ClickTrackingSetting {
    enable: bool,
    enable_text: bool,
}

/// Struct used for serializing the OpenTracking node into SendGrid's API format. Use
/// `TrackingSettingsBuilder` to configure this.
#[derive(Serialize, Debug)]
pub struct OpenTrackingSetting {
    enable: bool,
    substitution_tag: String,
}

/// Struct used for serializing the SubscriptionTracking node into SendGrid's API format. Use
/// `TrackingSettingsBuilder` to configure this.
#[derive(Serialize, Debug)]
pub struct SubscriptionTrackingSetting {
    enable: bool,
    text: Option<String>,
    html: Option<String>,
    substitution_tag: String,
}

/// Struct used for serializing the GaTracking  node into SendGrid's API format. Use
/// `GaTrackingSettingBuilder` to construct this.
#[derive(Serialize, Debug)]
pub struct GaTrackingSetting {
    enable: bool,
    utm_source: Option<String>,
    utm_medium: Option<String>,
    utm_term: Option<String>,
    utm_content: Option<String>,
    utm_campaign: Option<String>,
}

impl Default for GaTrackingSetting {
    fn default() -> GaTrackingSetting {
        GaTrackingSetting {
            enable: true,
            utm_source: None,
            utm_medium: None,
            utm_term: None,
            utm_content: None,
            utm_campaign: None,
        }
    }
}

/// Builder pattern for constructing `GaTrackingSetting`. Make sure you call `build()` when done to
/// consume this and return the underlying `GaTrackingSetting`, Construct with `default()`.
#[derive(Default)]
pub struct GaTrackingSettingBuilder {
    setting: GaTrackingSetting,
}

impl GaTrackingSettingBuilder {
    /// Sets the GA utm_source for the `GaTrackingSetting`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid_rs::GaTrackingSettingBuilder;
    ///
    /// let builder = GaTrackingSettingBuilder::default()
    ///               .utm_source("source");
    /// ```
    pub fn utm_source(mut self, source: impl Into<String>) -> Self {
        self.setting.utm_source = Some(source.into());
        self
    }

    /// Sets the GA utm_medium for the `GaTrackingSetting`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid_rs::GaTrackingSettingBuilder;
    ///
    /// let builder = GaTrackingSettingBuilder::default()
    ///               .utm_medium("medium");
    /// ```
    pub fn utm_medium(mut self, medium: impl Into<String>) -> Self {
        self.setting.utm_medium = Some(medium.into());
        self
    }

    /// Sets the GA utm_term for the `GaTrackingSetting`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid_rs::GaTrackingSettingBuilder;
    ///
    /// let builder = GaTrackingSettingBuilder::default()
    ///               .utm_term("term");
    /// ```
    pub fn utm_term(mut self, term: impl Into<String>) -> Self {
        self.setting.utm_term = Some(term.into());
        self
    }

    /// Sets the GA utm_content for the `GaTrackingSetting`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid_rs::GaTrackingSettingBuilder;
    ///
    /// let builder = GaTrackingSettingBuilder::default()
    ///               .utm_content("content");
    /// ```
    pub fn utm_content(mut self, content: impl Into<String>) -> Self {
        self.setting.utm_content = Some(content.into());
        self
    }

    /// Sets the GA utm_campaign for the `GaTrackingSetting`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid_rs::GaTrackingSettingBuilder;
    ///
    /// let builder = GaTrackingSettingBuilder::default()
    ///               .utm_campaign("campaign");
    /// ```
    pub fn utm_campaign(mut self, campaign: impl Into<String>) -> Self {
        self.setting.utm_campaign = Some(campaign.into());
        self
    }

    /// Consumes the builder and returns the underlying `GaTrackingSetting`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid_rs::GaTrackingSettingBuilder;
    ///
    /// let setting = GaTrackingSettingBuilder::default().build();
    /// ```
    pub fn build(self) -> GaTrackingSetting {
        self.setting
    }
}

/// Configures the SendGrid API node for TrackingSettings. Use `TrackingSettingsBuilder` to
/// construct this.
#[derive(Serialize, Default, Debug)]
pub struct TrackingSettings {
    click_tracking: Option<ClickTrackingSetting>,
    open_tracking: Option<OpenTrackingSetting>,
    subscription_tracking: Option<SubscriptionTrackingSetting>,
    ganalytics: Option<GaTrackingSetting>,
}

/// Builder pattern for `TrackingSettings`. Make sure you call `build()` to consume this and return
/// the underlying `TrackingSetting`. Construct with default().
#[derive(Default)]
pub struct TrackingSettingsBuilder {
    settings: TrackingSettings,
}

impl TrackingSettingsBuilder {
    /// Sets the click_tracking setting enabled to be true. Pass true to this method to enable text
    /// tracking as well.
    ///
    /// # Examples
    /// ```
    /// # use sendgrid_rs::TrackingSettingsBuilder;
    ///
    /// let builder = TrackingSettingsBuilder::default()
    ///               .click_tracking(true);
    /// ```
    pub fn click_tracking(mut self, enable_text: bool) -> Self {
        self.settings.click_tracking = Some(ClickTrackingSetting {
            enable: true,
            enable_text,
        });
        self
    }

    /// Sets the open_tracking stting enabled to be true. Pass in the substitution_tag to be
    /// replaced with it.
    ///
    /// # Examples
    /// ```
    /// # use sendgrid_rs::TrackingSettingsBuilder;
    ///
    /// let builder = TrackingSettingsBuilder::default()
    ///               .open_tracking("[OPEN_TAG]");
    /// ```
    pub fn open_tracking(mut self, substitution_tag: impl Into<String>) -> Self {
        self.settings.open_tracking = Some(OpenTrackingSetting {
            enable: true,
            substitution_tag: substitution_tag.into(),
        });
        self
    }

    /// Sets the substitution tag and text and/or html for subscription_tracking
    ///
    /// # Parameters
    /// substitution_tag: impl Intl<String>
    /// text: Option<String>
    /// html: Option<String>
    /// # Examples
    /// ```
    /// # use sendgrid_rs::TrackingSettingsBuilder;
    ///
    /// let builder = TrackingSettingsBuilder::default()
    ///               .substitution_tag("[SUBSTITUTION_TAG]", None, None);
    /// ```
    pub fn substitution_tag(
        mut self,
        substitution_tag: impl Into<String>,
        text: Option<String>,
        html: Option<String>,
    ) -> Self {
        self.settings.subscription_tracking = Some(SubscriptionTrackingSetting {
            enable: true,
            substitution_tag: substitution_tag.into(),
            text,
            html,
        });
        self
    }

    /// Consumes the `TrackingSettingsBuilder` and returns the underlying `TrackingSettings`
    ///
    /// # Examples
    /// ```
    /// # use sendgrid_rs::TrackingSettingsBuilder;
    ///
    /// let setting = TrackingSettingsBuilder::default().build();
    /// ```
    pub fn build(self) -> TrackingSettings {
        self.settings
    }
}
