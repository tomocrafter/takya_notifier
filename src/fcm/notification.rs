use serde_with_macros::skip_serializing_none;

use serde_derive::Serialize;

/// This struct represents a FCM notification. Use the
/// corresponding `NotificationBuilder` to get an instance. You can then use
/// this notification instance when sending a FCM message.
#[skip_serializing_none]
#[derive(Serialize, Debug, PartialEq)]
pub struct Notification {
    badge: Option<String>,
    body: Option<String>,
    body_loc_args: Option<Vec<String>>,
    body_loc_key: Option<String>,
    click_action: Option<String>,
    color: Option<String>,
    icon: Option<String>,
    sound: Option<String>,
    tag: Option<String>,
    title: Option<String>,
    title_loc_args: Option<Vec<String>>,
    title_loc_key: Option<String>,
}
pub struct NotificationBuilder {
    title: Option<String>,
    body: Option<String>,
    icon: Option<String>,
    sound: Option<String>,
    badge: Option<String>,
    tag: Option<String>,
    color: Option<String>,
    click_action: Option<String>,
    body_loc_key: Option<String>,
    body_loc_args: Option<Vec<String>>,
    title_loc_key: Option<String>,
    title_loc_args: Option<Vec<String>>,
}

impl NotificationBuilder {
    pub fn new() -> NotificationBuilder {
        NotificationBuilder {
            title: None,
            body: None,
            icon: None,
            sound: None,
            badge: None,
            tag: None,
            color: None,
            click_action: None,
            body_loc_key: None,
            body_loc_args: None,
            title_loc_key: None,
            title_loc_args: None,
        }
    }

    // Set the title of the notification
    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.title = Some(title.into());
        self
    }

    /// Set the body of the notification
    pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
        self.body = Some(body.into());
        self
    }

    /// Set the notification icon.
    pub fn icon(&mut self, icon: impl Into<String>) -> &mut Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the sound to be played
    pub fn sound(&mut self, sound: impl Into<String>) -> &mut Self {
        self.sound = Some(sound.into());
        self
    }

    /// Set the badge for iOS notifications
    pub fn badge(&mut self, badge: impl Into<String>) -> &mut Self {
        self.badge = Some(badge.into());
        self
    }

    /// Tagging a notification allows you to replace existing notifications
    /// with the same tag with this new notification
    pub fn tag(&mut self, tag: impl Into<String>) -> &mut Self {
        self.tag = Some(tag.into());
        self
    }

    /// The color of the icon, in #rrggbb format
    pub fn color(&mut self, color: impl Into<String>) -> &mut Self {
        self.color = Some(color.into());
        self
    }

    /// What happens when the user clicks on the notification. Refer to
    /// https://developers.google.com/cloud-messaging/http-server-ref#table2 for
    /// details.
    pub fn click_action(&mut self, click_action: impl Into<String>) -> &mut Self {
        self.click_action = Some(click_action.into());
        self
    }

    /// Set the body key string for localization
    pub fn body_loc_key(&mut self, body_loc_key: impl Into<String>) -> &mut Self {
        self.body_loc_key = Some(body_loc_key.into());
        self
    }

    /// String value to replace format specifiers in the body string.
    pub fn body_loc_args<S>(&mut self, body_loc_args: &[S]) -> &mut Self
    where
        S: Into<String> + AsRef<str>,
    {
        let converted = body_loc_args.iter().map(|a| a.as_ref().into()).collect();

        self.body_loc_args = Some(converted);
        self
    }

    /// Set the title key string for localization
    pub fn title_loc_key(&mut self, title_loc_key: impl Into<String>) -> &mut Self {
        self.title_loc_key = Some(title_loc_key.into());
        self
    }

    /// String value to replace format specifiers in the title string.
    pub fn title_loc_args<S>(&mut self, title_loc_args: &[S]) -> &mut Self
    where
        S: Into<String> + AsRef<str>,
    {
        let converted = title_loc_args.iter().map(|a| a.as_ref().into()).collect();

        self.title_loc_args = Some(converted);
        self
    }

    /// Complete the build and get a `Notification` instance
    pub fn build(self) -> Notification {
        Notification {
            title: self.title,
            body: self.body,
            icon: self.icon,
            sound: self.sound,
            badge: self.badge,
            tag: self.tag,
            color: self.color,
            click_action: self.click_action,
            body_loc_key: self.body_loc_key,
            body_loc_args: self.body_loc_args,
            title_loc_key: self.title_loc_key,
            title_loc_args: self.title_loc_args,
        }
    }
}
