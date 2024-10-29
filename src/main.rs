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

    // Fetch user's metadata
    let _metadata = client.fetch_metadata(public_key, None).await?;

    let filter = Filter::new()
        .kind(Kind::Metadata)
        .author(public_key)
        .limit(1);
    let events = client.database().query(vec![filter]).await?;
    let event = events.first().unwrap();
    let seens = client
        .database()
        .event_seen_on_relays(&event.id)
        .await?
        .unwrap();

    for url in seens {
        println!("Seen on: {}", url)
    }

    Ok(())
}
