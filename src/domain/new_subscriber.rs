use crate::{domain::subscriber_name::SubscriberName, routes::SubscribeForm};

use super::SubscriberEmail;

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}

impl TryFrom<SubscribeForm> for NewSubscriber {
    type Error = String;
    fn try_from(value: SubscribeForm) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
}
