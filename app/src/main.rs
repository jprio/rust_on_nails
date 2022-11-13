mod config;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = config.create_pool();

    let client = pool.get().await.unwrap();

    let fortunes = queries::fortunes::fortunes()
        .bind(&client)
        .all()
        .await
        .unwrap();

    dbg!(fortunes);
}

// Include the generated source code
include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));
