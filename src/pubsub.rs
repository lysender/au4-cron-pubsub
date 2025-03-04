use pub_sub_client::PubSubClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::Result;
use crate::jwt::create_token;

#[derive(Debug, Serialize, Deserialize)]
pub struct StubData {
    pub stub: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishedJobDto<T> {
    pub id: String,
    pub job: String,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishedEventDto<T> {
    pub id: String,
    pub event: String,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PublishedPayload<T> {
    Job(PublishedJobDto<T>),
    Event(PublishedEventDto<T>),
}

pub fn create_client(key_file: &String) -> Result<PubSubClient> {
    let res = PubSubClient::new(key_file.as_str(), std::time::Duration::from_secs(30));
    match res {
        Ok(client) => Ok(client),
        Err(err) => {
            let msg = format!("Error creating PubSub client: {}", err);
            Err(msg.into())
        }
    }
}

pub fn create_message(
    name: &str,
    is_job: bool,
    jwt_secret: &String,
) -> (PublishedPayload<StubData>, HashMap<String, String>) {
    let id = uuid::Uuid::now_v7().to_string();
    let token = create_token(&id, jwt_secret).unwrap();

    if is_job {
        (
            PublishedPayload::Job(PublishedJobDto {
                id,
                job: name.to_string(),
                data: StubData { stub: None },
            }),
            HashMap::from([("token".to_string(), token)]),
        )
    } else {
        (
            PublishedPayload::Event(PublishedEventDto {
                id,
                event: name.to_string(),
                data: StubData { stub: None },
            }),
            HashMap::from([
                ("token".to_string(), token),
                ("eventLogId".to_string(), uuid::Uuid::new_v4().to_string()),
            ]),
        )
    }
}

pub async fn send_message(
    client: &PubSubClient,
    topic: &String,
    message: (PublishedPayload<StubData>, HashMap<String, String>),
) -> Result<()> {
    let res = client
        .publish::<PublishedPayload<StubData>, _>(topic, vec![message], None, None)
        .await;

    match res {
        Ok(_) => Ok(()),
        Err(err) => {
            let msg = format!("Error sending message: {}", err);
            return Err(msg.into());
        }
    }
}
