use crate::Rowable;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ExchangeInfo {
    pub auto_delete: bool,
    pub durable: bool,
    pub internal: bool,
    pub name: String,
    #[serde(alias = "type")]
    pub t: String,
    pub user_who_performed_action: String,
    pub vhost: String,
}

impl ExchangeInfo {
    pub fn headers<'a>() -> [&'a str; 2] {
        [
            "Name",
            "Type",
        ]
    }
}

impl Rowable for ExchangeInfo {
    fn to_row(&self) -> Vec<String> {
        let nice_name = if self.name.is_empty() {
            "(AMQP DEFAULT)".to_owned()
        } else {
            self.name.clone()
        };

        vec![
            nice_name,
            self.t.clone(),
        ]
    }
}

#[derive(Deserialize, Debug)]
pub struct ExchangeBindings {
    pub source: String,
    pub vhost: String,
    #[serde(alias = "destination")]
    pub dest: String,
    #[serde(alias = "destination_type")]
    pub dest_type: String,
    pub routing_key: String,
    #[serde(alias = "properties_key")]
    pub prop_key: String,
}

impl ExchangeBindings {
    pub fn headers<'a>() -> [&'a str; 2] {
        [
            "To",
            "Routing key",
        ]
    }
}

impl Rowable for ExchangeBindings {
    fn to_row(&self) -> Vec<String> {
        vec![
            self.dest.clone(),
            self.routing_key.clone(),
        ]
    }
}

#[derive(Deserialize, Debug)]
pub struct Overview {
    pub queue_totals: OverviewQueueTotals,
}

#[derive(Deserialize, Debug)]
pub struct OverviewQueueTotals {
    pub messages: f64,
    pub messages_ready: f64,
    #[serde(alias = "messages_unacknowledged")]
    pub messages_unacked: f64,
}
