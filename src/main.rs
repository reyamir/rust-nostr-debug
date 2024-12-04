use nostr_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let opts = Options::new().gossip(true);
    let database = NostrLMDB::open("./db/nostr-lmdb")?;
    let client: Client = ClientBuilder::default()
        .database(database)
        .opts(opts)
        .build();

    client.add_relay("wss://relay.damus.io").await?;
    client.add_discovery_relay("wss://purplepag.es").await?;

    client.connect().await;

    // Test user
    let public_key =
        PublicKey::from_bech32("npub1zfss807aer0j26mwp2la0ume0jqde3823rmu97ra6sgyyg956e0s6xw445")?;

    let filter = Filter::new()
        .kind(Kind::Metadata)
        .author(public_key)
        .limit(1);

    client
        .handle_notifications(|notification| async {
            if let RelayPoolNotification::Event { event, .. } = notification {
                println!("Event: {}", event.as_json())
            }
            Ok(false)
        })
        .await?;

    _ = tokio::spawn(async move {
        if let Ok(output) = client.subscribe(vec![filter], None).await {
            println!("Output: {:?}", output);
        }
    })
    .await;

    Ok(())
}
