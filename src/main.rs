use humberto::{start, Config};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::new().expect("Couldn't load config");

    println!("Starting..");
    start(config).await
}
