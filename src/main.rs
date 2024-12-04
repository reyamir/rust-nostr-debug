use nostr_sdk::prelude::*;
use std::sync::OnceLock;

static CLIENT: OnceLock<Client> = OnceLock::new();

pub fn initialize_client() {
    let opts = Options::new().gossip(true).automatic_authentication(false);
    let database = NostrLMDB::open("./db/nostr-lmdb").unwrap();
    let client: Client = ClientBuilder::default()
        .database(database)
        .opts(opts)
        .build();

    CLIENT.set(client).expect("Client is already initialized!");
}

pub fn get_client() -> &'static Client {
    CLIENT.get().expect("Client is NOT initialized!")
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // Initialize nostr client
    initialize_client();

    // Get client
    let client = get_client();

    client.add_relay("wss://relay.damus.io").await?;
    client.connect().await;

    // Test user
    let public_key =
        PublicKey::from_bech32("npub1zfss807aer0j26mwp2la0ume0jqde3823rmu97ra6sgyyg956e0s6xw445")?;

    let filter = Filter::new()
        .kind(Kind::Metadata)
        .author(public_key)
        .limit(1);

    let _ = client.fetch_metadata(public_key, None).await;

    _ = tokio::spawn(async move {
        client
            .handle_notifications(|notification| async {
                if let RelayPoolNotification::Message { message, .. } = notification {
                    println!("Message: {}", message.as_json())
                }
                Ok(false)
            })
            .await
            .unwrap();
    })
    .await;

    _ = tokio::spawn(async move {
        if let Ok(output) = client.subscribe(vec![filter], None).await {
            println!("Output: {:?}", output);
        }
    })
    .await;

    Ok(())
}
