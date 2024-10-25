use crate::config::SyncConfig;

mod config;

#[tokio::main]
async fn main() {
    let sync_config = SyncConfig::from_config_file();
    println!("{:#?}", sync_config);
}
