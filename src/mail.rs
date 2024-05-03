use std::env;
use dotenvy::dotenv;
use mailjet_rs::common::Recipient;
use mailjet_rs::v3::Message;
use mailjet_rs::{Client, SendAPIVersion};

pub async fn send_mail(address: String,  message: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let pubkey = env::var("MAILJET_PUB_KEY").expect("MAILJET_PUB_KEY must be set");
    let privkey = env::var("MAILJET_PRIV_KEY").expect("MAILJET_PRIV_KEY must be set");
    let client = Client::new(
        SendAPIVersion::V3,
        pubkey.as_str(),
        privkey.as_str(),
    );

    let mut message = Message::new(
        "trash7217@gmail.com",
        "SMART Tracker",
        Some("Alerte SMART Tracker".to_string()),
        Some(message)
    );

    let recipients = vec![
        Recipient::new(&*address)
    ];

    message.push_many_recipients(recipients);

    let response = client.send(message).await;

    println!("{:?}", response);

    Ok(())
}