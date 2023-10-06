use std::sync::Arc;

use anyhow::{Context, Result};
use tauri::{AppHandle, Manager};

use crate::{analytics, users};

use super::events;

#[derive(Clone)]
pub struct Handler {
    inner: Arc<HandlerInner>,
}

impl From<&AppHandle> for Handler {
    fn from(value: &AppHandle) -> Self {
        Self {
            inner: HandlerInner::from(value).into(),
        }
    }
}

impl Handler {
    pub fn handle(&self, event: &analytics::Event) -> Result<Vec<events::Event>> {
        self.inner.handle(event)
    }
}

struct HandlerInner {
    users: users::Controller,
    client: analytics::Client,
}

impl From<&AppHandle> for HandlerInner {
    fn from(value: &AppHandle) -> Self {
        Self {
            client: value.state::<analytics::Client>().inner().clone(),
            users: users::Controller::from(value),
        }
    }
}

impl HandlerInner {
    pub fn handle(&self, event: &analytics::Event) -> Result<Vec<events::Event>> {
        if let Some(user) = self.users.get_user().context("failed to get user")? {
            futures::executor::block_on(self.client.send(&user, event))
                .context("failed to send event")?;
        }
        Ok(vec![])
    }
}
