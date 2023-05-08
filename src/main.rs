use async_nats::jetstream;
use futures::stream::StreamExt;
use tracing::warn;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let client = async_nats::connect("localhost").await.unwrap();
    let js = jetstream::new(client);

    let kv = js.get_key_value("config").await.unwrap();
    let mut h = kv.watch("b.*").await.unwrap();
    while let Some(m) = h.next().await {
        warn!("{:#?}", m.unwrap());
    }
}
