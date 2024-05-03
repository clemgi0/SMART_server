use mailjet_rs::common::Recipient;
use mailjet_rs::v3::Message;
use mailjet_rs::{Client, SendAPIVersion};

static CLIENT: Client = Client::new(
    SendAPIVersion::V3,
    "e6c50096471c260462ac96923b66c1c4",
    "a9e695004b24cc56ee2184b55837a07e",
);

pub async fn send_mail() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut message = Message::new(
        "alexandre.rosard@gmail.com",
        "Alejandro",
        Some("Chistiano Ronaldo!".to_string()),
        Some("Suuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu!".to_string())
    );

    let recipients = vec![
        Recipient::new("arthur.galet@insa-lyon.fr"),
        Recipient::new("martin.bonnefoy@insa-lyon.fr"),
        Recipient::new("elie.tarassov@insa-lyon.fr"),
    ];

    message.push_many_recipients(recipients);

    let response = CLIENT.send(message).await;

    println!("{:?}", response);

    Ok(())
}