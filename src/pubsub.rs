use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::client::google_cloud_auth::credentials::CredentialsFile;
use google_cloud_pubsub::client::{Client, ClientConfig};
use google_cloud_pubsub::publisher::Publisher;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::Result;
use crate::jwt::create_token;

#[derive(Debug, Serialize, Deserialize)]
pub struct StubData {
    #[serde(rename(serialize = "lastId"))]
    pub last_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishedJobDto {
    pub id: String,
    pub job: String,
    pub data: StubData,
}

pub async fn create_client(key_file: &str) -> Result<Client> {
    match CredentialsFile::new_from_file(key_file.to_string()).await {
        Ok(creds) => match ClientConfig::default().with_credentials(creds).await {
            Ok(config) => match Client::new(config).await {
                Ok(client) => Ok(client),
                Err(err) => Err(format!("Error creating PubSub client: {}", err).into()),
            },
            Err(err) => Err(format!("Error creating PubSub client config: {}", err).into()),
        },
        Err(err) => Err(format!("Error reading credentials file: {}", err).into()),
    }
}

pub fn create_message(name: &str, jwt_secret: &str) -> (PublishedJobDto, HashMap<String, String>) {
    let id = uuid::Uuid::now_v7().to_string();
    let token = create_token(&id, jwt_secret).unwrap();

    (
        PublishedJobDto {
            id,
            job: name.to_string(),
            data: StubData { last_id: None },
        },
        HashMap::from([("token".to_string(), token)]),
    )
}

pub async fn send_message(
    publisher: &Publisher,
    message: (PublishedJobDto, HashMap<String, String>),
) -> Result<()> {
    let Ok(data) = serde_json::to_string(&message.0) else {
        return Err("Error serializing message data".into());
    };

    let msg = PubsubMessage {
        data: data.into(),
        attributes: message.1,
        ..Default::default()
    };

    let awaiter = publisher.publish(msg).await;
    match awaiter.get().await {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Error sending message: {}", err).into()),
    }
}
