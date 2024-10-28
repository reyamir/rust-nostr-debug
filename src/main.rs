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
    let events = client.fetch_metadata(public_key, None).await;

    println!("Events: {:?}", events);

    Ok(())
}
