use std::collections::HashMap;
use pub_sub_client::PubSubClient;
use serde::{Serialize, Deserialize};

use crate::error::Result;
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
    let client = PubSubClient::new(key_file.as_str(), std::time::Duration::from_secs(30))?;
    Ok(client)
}

pub fn create_message(name: &String, is_job: bool, jwt_secret: &String) -> (PublishedPayload<StubData>, HashMap<String, String>) {
    let id = uuid::Uuid::new_v4().to_string();
    let token = create_token(&id, jwt_secret).unwrap();

    if is_job {
        (
            PublishedPayload::Job(PublishedJobDto {
                id,
                job: name.to_string(),
                data: StubData {
                    stub: None,
                }
            }),
            HashMap::from([("token".to_string(), token)]),
        )
    } else {
        (
            PublishedPayload::Event(PublishedEventDto {
                id,
                event: name.to_string(),
                data: StubData {
                    stub: None,
                }
            }),
            HashMap::from([
                ("token".to_string(), token),
                ("eventLogId".to_string(), uuid::Uuid::new_v4().to_string()),
            ]),
        )
    }
}

pub async fn send_message(client: &PubSubClient, topic: &String, message: (PublishedPayload<StubData>, HashMap<String, String>)) -> Result<()>{
    client.publish::<PublishedPayload<StubData>, _>(topic, vec![message], None, None).await?;
    Ok(())
}
